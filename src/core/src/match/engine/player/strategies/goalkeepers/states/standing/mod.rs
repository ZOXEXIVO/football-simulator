use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::strategies::processor::StateChangeResult;
use crate::r#match::{ConditionContext, GameTickContext, MatchContext, MatchPlayer, StateProcessingContext, StateProcessingHandler};
use nalgebra::Vector3;
use std::sync::LazyLock;

static GOALKEEPER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json")));

#[derive(Default)]
pub struct GoalkeeperStandingState {}

impl StateProcessingHandler for GoalkeeperStandingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, ctx: ConditionContext) {
    }
}

impl GoalkeeperStandingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        StateChangeResult::new()
    }
}
