use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::strategies::processor::StateChangeResult;
use crate::r#match::{ConditionContext, StateProcessingContext, StateProcessingHandler, VectorExtensions};
use crate::r#match::goalkeepers::states::state::GoalkeeperState;

static GOALKEEPER_WALKING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_walking_data.json")));

#[derive(Default)]
pub struct GoalkeeperWalkingState {}

impl StateProcessingHandler for GoalkeeperWalkingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Check if the ball is close and not owned by a teammate
        if self.is_ball_close(ctx) && !ctx.ball().is_owned() {
            return if self.should_come_out(ctx) {
                Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::ComingOut))
            } else {
                Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::PreparingForSave))
            }
        }

        // Check if the goalkeeper is out of position
        if self.is_out_of_position(ctx) {
            return Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::ReturningToGoal));
        }

        // Check if there's an immediate threat
        if self.is_under_threat(ctx) {
            return Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::UnderPressure));
        }

        // If the ball is far and the goalkeeper is in position, transition to Standing
        if ctx.ball().distance() > 50.0 && !self.is_out_of_position(ctx) {
            return Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::Standing));
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        let optimal_position = self.calculate_optimal_position(ctx);
        let direction = (optimal_position - ctx.player.position).normalize();
        let walking_speed = ctx.player.skills.physical.pace * 0.3; // Walking is slower than running
        Some(direction * walking_speed)
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}

impl GoalkeeperWalkingState {
    fn is_ball_close(&self, ctx: &StateProcessingContext) -> bool {
        ctx.ball().distance() < 20.0
    }

    fn is_out_of_position(&self, ctx: &StateProcessingContext) -> bool {
        let optimal_position = self.calculate_optimal_position(ctx);
        ctx.player.position.distance_to(&optimal_position) > 5.0 // Adjust this value as needed
    }

    fn is_under_threat(&self, ctx: &StateProcessingContext) -> bool {
        let player_ops = ctx.player();
        let opponents_with_ball = player_ops.opponent_with_ball();

        if !opponents_with_ball.is_empty() {
            let opponent = opponents_with_ball[0];
            let distance_to_opponent = opponent.position.distance_to(&ctx.player.position);
            distance_to_opponent < 30.0 // Adjust this value based on your game's scale
        } else {
            false
        }
    }

    fn should_come_out(&self, ctx: &StateProcessingContext) -> bool {
        let ball_distance = ctx.ball().distance();
        let goalkeeper_skills = &ctx.player.skills;

        // Decision based on ball distance and goalkeeper's skills
        ball_distance < 50.0 && goalkeeper_skills.mental.decisions > 10.0 && goalkeeper_skills.physical.acceleration > 10.0
    }

    fn calculate_optimal_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let goal_position = ctx.ball().direction_to_own_goal();

        let ball_position = ctx.tick_context.object_positions.ball_position;

        // Calculate a position slightly in front of the goal, on the line between the ball and the center of the goal
        let to_ball = ball_position - goal_position;
        let optimal_distance = 2.0; // Adjust this value based on how far out you want the goalkeeper to be

        goal_position + to_ball.normalize() * optimal_distance
    }
}