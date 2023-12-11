use crate::common::NeuralNetwork;
use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};
use crate::r#match::position::VectorExtensions;

lazy_static! {
    static ref PLAYER_RETURNING_STATE_NETWORK: NeuralNetwork = PlayerReturningStateNetLoader::load();
}

pub struct ReturningState {}

impl ReturningState {
    pub fn process(
        _in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if player.position.distance_to(&player.start_position) < 10.0 {
            return Some(PlayerState::Standing);
        }

        None
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_returning_data.json");

#[derive(Debug)]
pub struct PlayerReturningStateNetLoader;

impl PlayerReturningStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}