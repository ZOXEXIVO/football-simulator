use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler};
use crate::r#match::goalkeepers::states::state::GoalkeeperState;

static GOALKEEPER_CATCHING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_catching_data.json")));

#[derive(Default)]
pub struct GoalkeeperCatchingState {}

impl StateProcessingHandler for GoalkeeperCatchingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if self.is_catch_successful(ctx) {
            //result.events.add(PlayerUpdateEvent::BallCaught(ctx.player.id));
            Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::HoldingBall))
        } else {
            // If catch fails, transition to appropriate state based on ball position
            if ctx.ball().distance() < 5.0 {
                Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::Diving))
            } else {
                Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::PreparingForSave))
            }
        }
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // During catching, the goalkeeper's velocity should be minimal
        // but we can add a small adjustment towards the ball
        let ball_position = ctx.tick_context.object_positions.ball_position;
        let direction = (ball_position - ctx.player.position).normalize();
        let speed = 0.5; // Very low speed for final adjustments

        Some(direction * speed)
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}

impl GoalkeeperCatchingState {
    fn is_catch_successful(&self, ctx: &StateProcessingContext) -> bool {
        let catch_skill =   (ctx.player.skills.technical.first_touch + ctx.player.skills.technical.technique) / 2.0;
        let ball_speed = ctx.tick_context.object_positions.ball_velocity.norm();
        let distance_to_ball = ctx.ball().distance();

        // Calculate catch probability based on skill, ball speed, and distance
        let catch_probability = catch_skill / 100.0 * (1.0 - (ball_speed / 30.0)) * (1.0 - (distance_to_ball / 5.0));

        // Random number between 0 and 1
        let random_factor = rand::random::<f32>();

        catch_probability > random_factor
    }

    fn calculate_catch_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let ball_position = ctx.tick_context.object_positions.ball_position;
        let ball_velocity = ctx.tick_context.object_positions.ball_velocity;

        // Predict ball position slightly ahead of time
        let prediction_time = 0.1; // 100ms prediction
        ball_position + ball_velocity * prediction_time
    }
}
