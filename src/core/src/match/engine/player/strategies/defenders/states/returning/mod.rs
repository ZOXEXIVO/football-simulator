use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{StateChangeResult, StateProcessingContext, StateProcessingHandler, SteeringBehavior, MATCH_HALF_TIME_MS};
use crate::r#match::defenders::states::DefenderState;

static DEFENDER_RETURNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_returning_data.json")));

#[derive(Default)]
pub struct DefenderReturningState {}

impl StateProcessingHandler for DefenderReturningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.player().distance_from_start_position() < 5.0 {
            return Some(StateChangeResult::with_defender_state(DefenderState::Standing));
        }

        if ctx.ball().is_towards_player() && ctx.ball().distance() < 40.0 {
            return Some(StateChangeResult::with_defender_state(DefenderState::Intercepting));
        }

        if ctx.player().is_team_loosing() && ctx.context.time.time > (MATCH_HALF_TIME_MS - 300)  {
            return Some(StateChangeResult::with_defender_state(DefenderState::Pressing));
        }

        // if ctx.player().is_tired() {
        //     return Some(StateChangeResult::with_defender_state(DefenderState::Walking));
        // }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(SteeringBehavior::Arrive {
            target: ctx.player.start_position,
            slowing_distance: 10.0,
        }.calculate(ctx.player).velocity)
    }
}
