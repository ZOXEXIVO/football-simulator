use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext,
    StateProcessingHandler, SteeringBehavior,
};
use crate::IntegerUtils;
use nalgebra::Vector3;
use std::sync::LazyLock;

static DEFENDER_WALKING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_walking_data.json")));

#[derive(Default)]
pub struct MidfielderWalkingState {}

impl StateProcessingHandler for MidfielderWalkingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.team().is_control_ball() {
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Running,
            ));
        }

        // 1. If the defender is on their own side and the ball is close, transition to Intercepting
        if ctx.ball().distance() < 200.0 {
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Pressing,
            ));
        }

        // 3. Remain in Walking state
        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network logic if necessary
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(
            SteeringBehavior::Wander {
                target: ctx.player.start_position,
                radius: IntegerUtils::random(5, 150) as f32,
                jitter: IntegerUtils::random(0, 2) as f32,
                distance: IntegerUtils::random(10, 150) as f32,
                angle: IntegerUtils::random(0, 180) as f32,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions
    }
}
