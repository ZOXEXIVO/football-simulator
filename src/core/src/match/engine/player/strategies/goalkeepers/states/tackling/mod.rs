use crate::common::NeuralNetwork;

use crate::r#match::position::VectorExtensions;
use crate::r#match::{BallContext, GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerTickContext, PlayerUpdateEvent, StateChangeResult};

lazy_static! {
    static ref PLAYER_TACKLING_STATE_NETWORK: NeuralNetwork = PlayerTacklingStateNetLoader::load();
}

pub struct GoalkeeperTacklingState {}

impl GoalkeeperTacklingState {
    pub fn process(
        player: &MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let mut nearest_players: Vec<_> = tick_context.objects_positions
            .players_positions
            .iter()
            .filter(|p| {
                p.position.distance_to(&tick_context.objects_positions.ball_position) < 30.0
                    && p.player_id != player.player_id
            })
            .map(|p| p.player_id)
            .collect();

        return if !nearest_players.is_empty() {
            nearest_players.sort_by(|left_player_id, right_player_id| {
                let left_player = context.players.get(*left_player_id).unwrap();
                let right_player = context.players.get(*right_player_id).unwrap();

                left_player
                    .skills
                    .technical
                    .tackling
                    .partial_cmp(&right_player.skills.technical.tackling)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let opponent_to_tackle = nearest_players.first().unwrap();

            result.push(PlayerUpdateEvent::TacklingBall(*opponent_to_tackle));

            // TODO Own strategy
            StateChangeResult::with_state(PlayerState::Running)
        } else {
            StateChangeResult::none()
        };
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_tackling_data.json");

#[derive(Debug)]
pub struct PlayerTacklingStateNetLoader;

impl PlayerTacklingStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}
