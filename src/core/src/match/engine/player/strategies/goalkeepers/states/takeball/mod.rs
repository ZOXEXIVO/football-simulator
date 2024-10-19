use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;

#[derive(Default)]
pub struct GoalkeeperTakeBallState {}

impl StateProcessingHandler for GoalkeeperTakeBallState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.ball().is_owned() {
            return Some(StateChangeResult::with_goalkeeper_state(
                GoalkeeperState::ReturningToGoal,
            ));
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(
            SteeringBehavior::Arrive {
                target: ctx.tick_context.object_positions.ball_position,
                slowing_distance: 1.0,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}
