use itertools::Itertools;
use crate::common::NeuralNetwork;

use crate::r#match::strategies::goalkeepers::ball_heading_towards_goal;
use crate::r#match::{
    BallMetadata, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
    StateChangeResult, SteeringBehavior,
};
use crate::r#match::position::VectorExtensions;

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

        if Self::is_dangerous(player, objects_positions) {
            return StateChangeResult::with_state(PlayerState::Running);
        }

        if !ball_metadata.ball_is_on_player_home_side {
            return StateChangeResult::with_state(PlayerState::Walking);
        }

        if ball_metadata.ball_distance > 100.0 {
            return StateChangeResult::none();
        }

        if ball_metadata.ball_distance < 20.0 {
            return StateChangeResult::with_state(PlayerState::Tackling);
        }


        StateChangeResult::none()
    }

    fn is_dangerous(player: &MatchPlayer, objects_positions: &MatchObjectsPositions) -> bool {
        let mut nearest_home_count: f32 = 0.0;
        let mut nearest_away_count: f32 = 0.0;

        let nearest_players = objects_positions.players_positions
            .iter()
            .filter(|x| x.position.distance_to(&player.position) < 100.0)
            .map(|p| p.is_home);

        for (is_home, grouped_items)   in &nearest_players.group_by(|p| *p) {
            if is_home {
                nearest_home_count = grouped_items.count() as f32;
            }else {
                nearest_away_count = grouped_items.count()  as f32;
            }
        }

        (nearest_home_count + 1.0) / (nearest_away_count + 1.0) < 0.5
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
