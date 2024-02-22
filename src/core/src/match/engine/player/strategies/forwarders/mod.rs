pub mod states;

use crate::common::NeuralNetwork;
use crate::r#match::{GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerTickContext, PlayerUpdateEvent, StateChangeResult, SteeringBehavior};

pub struct ForwardStrategies {}

impl ForwardStrategies {
    pub fn calculate(
        in_state_time: u64,
        player: &MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let vel = SteeringBehavior::Arrive {
            target: tick_context.objects_positions.ball_position,
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
