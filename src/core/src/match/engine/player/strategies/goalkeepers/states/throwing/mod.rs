use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{StateChangeResult, StateProcessingContext, StateProcessingHandler};

static GOALKEEPER_THROWING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_throwing_data.json")));

#[derive(Default)]
pub struct GoalkeeperThrowingState {}

impl StateProcessingHandler for GoalkeeperThrowingState {
    fn try_fast(&self, context: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(&self, context: &StateProcessingContext) -> StateChangeResult {
        StateChangeResult::none()
    }

    fn velocity(&self) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}
