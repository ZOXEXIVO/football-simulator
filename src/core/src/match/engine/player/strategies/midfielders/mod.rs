use crate::r#match::{MatchContext, MatchObjectsPositions, MatchPlayer, PlayerUpdateEvent, StateChangeResult, SteeringBehavior};
use crate::common::NeuralNetwork;

pub struct MidfielderStrategies {}

impl MidfielderStrategies {
    pub fn calculate(
        _context: &mut MatchContext,
        player: &MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> StateChangeResult {
        let new_velocity =   SteeringBehavior::Arrive {
            target: objects_positions.ball_position,
            slowing_distance: 10.0,
        } .calculate(player)
            .velocity;

        StateChangeResult::with_velocity(new_velocity)
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_running_data.json");

#[derive(Debug)]
pub struct MidfieldersNetLoader;

impl MidfieldersNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}