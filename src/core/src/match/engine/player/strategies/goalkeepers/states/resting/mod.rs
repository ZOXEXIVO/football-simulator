use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static GOALKEEPER_RESTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_resting_data.json")));

const RESTING_STAMINA_THRESHOLD: f32 = 60.0; // Minimum stamina to transition out of resting state
const RESTING_STAMINA_RECOVERY_RATE: f32 = 0.5; // Amount of stamina recovered per second while resting

#[derive(Default)]
pub struct GoalkeeperRestingState {}

impl StateProcessingHandler for GoalkeeperRestingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. Check if the goalkeeper's stamina is above the resting threshold
        let stamina = ctx.player.player_attributes.condition_percentage() as f32;
        if stamina >= RESTING_STAMINA_THRESHOLD {
            return Some(StateChangeResult::with_goalkeeper_state(
                GoalkeeperState::Standing,
            ));
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network processing if needed
        None
    }

    fn velocity(&self, _ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Remain stationary while resting
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}
