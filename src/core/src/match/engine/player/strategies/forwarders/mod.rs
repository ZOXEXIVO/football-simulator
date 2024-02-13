pub mod states;

use crate::common::NeuralNetwork;
use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerUpdateEvent, StateChangeResult,
    SteeringBehavior,
};

pub struct ForwardStrategies {}

impl ForwardStrategies {
    pub fn calculate(
        in_state_time: u64,
        _context: &mut MatchContext,
        player: &MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> StateChangeResult {
        let vel = SteeringBehavior::Arrive {
            target: objects_positions.ball_position,
            slowing_distance: 10.0,
        }
        .calculate(player)
        .velocity;

        StateChangeResult::with_velocity(vel)
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_running_data.json");

#[derive(Debug)]
pub struct ForwardersNetLoader;

impl crate::r#match::ForwardersNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}
