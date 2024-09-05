use std::sync::LazyLock;

use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::strategies::processing::StateChangeResult;
use crate::r#match::{StateProcessingContext, StateProcessingHandler};

static COMMON_TACKLING_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_common_tackling_data.json"))
});

#[derive(Default)]
pub struct CommonTacklingState {}

impl StateProcessingHandler for CommonTacklingState {
    fn try_fast(&self, context: &mut StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(&self, context: &mut StateProcessingContext) -> StateChangeResult {
        StateChangeResult::none()
    }
}
