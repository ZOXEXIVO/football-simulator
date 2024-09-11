use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{StateChangeResult, StateProcessingContext, StateProcessingHandler};

static GOALKEEPER_PUNCHING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_punching_data.json")));

#[derive(Default)]
pub struct GoalkeeperPunchingState {}

impl StateProcessingHandler for GoalkeeperPunchingState {
    fn try_fast(
        &self, ctx: &StateProcessingContext
    ) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(
        &self, ctx: &StateProcessingContext
    ) -> StateChangeResult {
        StateChangeResult::none()
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}