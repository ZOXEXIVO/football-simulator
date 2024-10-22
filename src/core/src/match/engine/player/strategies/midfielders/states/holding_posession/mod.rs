use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static MIDFIELDER_HOLDING_POSSESSION_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_holding_possession_data.json"))
});

#[derive(Default)]
pub struct MidfielderHoldingPossessionState {}

impl StateProcessingHandler for MidfielderHoldingPossessionState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if self.is_under_pressure(ctx) {
            return if ctx.player.skills.technical.dribbling > 10.0 {
                Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Crossing,
                ))
            } else {
                Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Running,
                ))
            };
        }

        if ctx.player.skills.mental.decisions >= 10.0 {
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Distributing,
            ));
        } else {
            if ctx.in_state_time > 1000 {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Running,
                ));
            }
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(
            SteeringBehavior::Arrive {
                target: ctx.ball().direction_to_opponent_goal(),
                slowing_distance: 30.0,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl MidfielderHoldingPossessionState {
    pub fn is_under_pressure(&self, ctx: &StateProcessingContext) -> bool {
        ctx.players().opponents().exists(50.0)
    }
}
