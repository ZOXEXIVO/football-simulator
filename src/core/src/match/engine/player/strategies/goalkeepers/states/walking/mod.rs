use std::sync::LazyLock;

use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::strategies::processing::StateChangeResult;
use crate::r#match::{StateProcessingContext, StateProcessingHandler};

static GOALKEEPER_WALKING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_walking_data.json")));

#[derive(Default)]
pub struct GoalkeeperWalkingState {}

impl StateProcessingHandler for GoalkeeperWalkingState {
    fn try_fast(&self, context: &mut StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(&self, context: &mut StateProcessingContext) -> StateChangeResult {
        StateChangeResult::none()
    }
}
