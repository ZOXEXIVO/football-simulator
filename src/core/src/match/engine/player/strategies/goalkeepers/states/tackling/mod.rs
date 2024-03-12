use crate::common::NeuralNetwork;

use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerState, PlayerTickContext, PlayerUpdateEvent,
    StateChangeResult,
};

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
        let minimal_distance = 30.0;

        let (_, mut nearest_opponents) = tick_context
            .objects_positions
            .player_distances
            .players_within_distance(player, minimal_distance);

        return if !nearest_opponents.is_empty() {
            nearest_opponents.sort_by(|(left_player_id, _), (right_player_id, _)| {
                let left_player = context.players.get(*left_player_id).unwrap();
                let right_player = context.players.get(*right_player_id).unwrap();

                left_player
                    .skills
                    .technical
                    .tackling
                    .partial_cmp(&right_player.skills.technical.tackling)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let (opponent_to_tackle, _) = nearest_opponents.first().unwrap();

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
