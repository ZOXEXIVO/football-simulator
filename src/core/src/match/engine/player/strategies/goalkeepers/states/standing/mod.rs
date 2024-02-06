use crate::common::NeuralNetwork;

use crate::r#match::position::{PlayerFieldPosition, VectorExtensions};
use crate::r#match::{
    BallMetadata, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
    StateChangeResult,
};

lazy_static! {
    static ref PLAYER_STANDING_STATE_NETWORK: NeuralNetwork = PlayerStandingStateNetLoader::load();
}

pub struct GoalkeeperStandingState {}

impl GoalkeeperStandingState {
    pub fn process(
        player: &MatchPlayer,
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: BallMetadata,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if ball_metadata.ball_is_on_player_home_side
            && Self::is_dangerous(player, objects_positions)
        {
            return StateChangeResult::with_state(PlayerState::Running);
        }

        if let Some(nearest_opponent) = Self::nearest_opponent(player, objects_positions) {
            let distance_to_opponent = nearest_opponent.position.distance_to(&player.position);
            if distance_to_opponent < 50.0 {
                return StateChangeResult::with_state(PlayerState::Running);
            }
        }

        if ball_metadata.ball_distance > 100.0 {
            return StateChangeResult::none();
        }

        if ball_metadata.ball_distance < 20.0 {
            return StateChangeResult::with_state(PlayerState::Tackling);
        }

        return if ball_metadata.is_ball_heading_towards_goal {
            if Self::should_rush_out(player, objects_positions, ball_metadata) {
                StateChangeResult::with_state(PlayerState::Running)
            } else {
                StateChangeResult::with_state(PlayerState::Walking)
            }
        } else {
            StateChangeResult::none()
        };
    }

    fn should_rush_out(
        player: &MatchPlayer,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: BallMetadata,
    ) -> bool {
        objects_positions.ball_position.y.abs() < 10.0
            && objects_positions
                .ball_position
                .distance_to(&player.position)
                < 50.0
    }

    fn is_dangerous(player: &MatchPlayer, objects_positions: &MatchObjectsPositions) -> bool {
        let (nearest_home_count, nearest_away_count) = objects_positions
            .players_positions
            .iter()
            .filter(|p| p.position.distance_to(&player.position) < 100.0)
            .map(|p| p.is_home)
            .partition::<Vec<_>, _>(|&is_home| is_home);

        let nearest_home_count = nearest_home_count.len() as f32;
        let nearest_away_count = nearest_away_count.len() as f32;

        (nearest_home_count + 1.0) / (nearest_away_count + 1.0) < 0.5
    }

    fn nearest_opponent<'p>(
        player: &MatchPlayer,
        objects_positions: &'p MatchObjectsPositions,
    ) -> Option<&'p PlayerFieldPosition> {
        objects_positions
            .players_positions
            .iter()
            .filter(|p| p.is_home != player.is_home && p.player_id != player.player_id) // Consider only opponents and exclude the current player
            .min_by(|a, b| {
                let distance_a_squared = (a.position - player.position).norm_squared();
                let distance_b_squared = (b.position - player.position).norm_squared();

                distance_a_squared
                    .partial_cmp(&distance_b_squared)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_standing_data.json");

#[derive(Debug)]
pub struct PlayerStandingStateNetLoader;

impl PlayerStandingStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}
