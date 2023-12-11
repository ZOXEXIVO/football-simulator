use crate::common::NeuralNetwork;

use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};

use crate::r#match::position::VectorExtensions;
use nalgebra::Vector3;

lazy_static! {
    static ref PLAYER_TACKLING_STATE_NETWORK: NeuralNetwork = PlayerTacklingStateNetLoader::load();
}

pub struct TacklingState {}

impl TacklingState {
    pub fn process(
        _in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        player.velocity = player.skills.running_speed();
        // Check for transition to standing or walking state
        // let tackling_success = self.skills.tackling() * self.player_attributes.condition;
        // if tackling_success > 50.0 {
        //     self.has_ball = true;
        // }
        // // Check for transition to standing state
        // if self.player_attributes.condition < 20.0 {
        //     self.state = PlayerState::Standing;
        // }
        None
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