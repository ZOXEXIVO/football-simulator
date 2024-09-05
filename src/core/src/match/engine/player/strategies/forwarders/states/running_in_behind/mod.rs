use std::sync::LazyLock;

use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{StateChangeResult, StateProcessingContext, StateProcessingHandler};

static FORWARD_RUNNING_IN_BEHIND_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_running_in_behind_data.json"))
});

#[derive(Default)]
pub struct ForwardRunningInBehindState {}

impl StateProcessingHandler for ForwardRunningInBehindState {
    fn try_fast(&self, context: &mut StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(&self, context: &mut StateProcessingContext) -> StateChangeResult {
        StateChangeResult::none()
    }
}
