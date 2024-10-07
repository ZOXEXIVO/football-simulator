use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::{
    ConditionContext, PlayerSide, StateChangeResult, StateProcessingContext,
    StateProcessingHandler, VectorExtensions,
};
use nalgebra::Vector3;
use std::sync::LazyLock;
use crate::r#match::player::events::PlayerUpdateEvent;

static GOALKEEPER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json")));

const BALL_PROXIMITY_THRESHOLD: f32 = 200.0;
const DANGER_ZONE_RADIUS: f32 = 30.0;
const REACTION_TIME_THRESHOLD: u64 = 1000; // in milliseconds
const OPTIMAL_DISTANCE_FROM_GOAL: f32 = 200.0; //

#[derive(Default)]
pub struct GoalkeeperStandingState {}

impl StateProcessingHandler for GoalkeeperStandingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();

        if ctx.ball().is_towards_player_with_angle(0.8)
            && ctx.ball().distance() < BALL_PROXIMITY_THRESHOLD
        {
            if ctx.ball().is_towards_player() {
                return Some(StateChangeResult::with_goalkeeper_state(
                    GoalkeeperState::PreparingForSave,
                ));
            }
        }

        if self.is_opponent_in_danger_zone(ctx) {
            return Some(StateChangeResult::with_goalkeeper_state(
                GoalkeeperState::UnderPressure,
            ));
        }

        if ctx.ball().distance() > BALL_PROXIMITY_THRESHOLD * 2.0 {
            if ctx.player.side == Some(PlayerSide::Right) {
                let distance = ctx.ball().distance();
            }

            return Some(StateChangeResult::with_goalkeeper_state(
                GoalkeeperState::Walking,
            ));
        }

        // Adjust position if needed
        let optimal_position = self.calculate_optimal_position(ctx);
        if ctx.player.position.distance_to(&optimal_position) > 0.5 {
            result.events.add(PlayerUpdateEvent::MovePlayer(ctx.player.id, optimal_position));
            return Some(result);
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network processing if needed
        // For now, return None to indicate no state change
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        let optimal_position = self.calculate_optimal_position(ctx);
        let direction = (optimal_position - ctx.player.position).normalize();
        let speed = ctx.player.skills.physical.acceleration * 0.1; // Slow movement for minor adjustments
        Some(direction * speed)
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions to process in this state
    }
}

impl GoalkeeperStandingState {
    fn is_opponent_in_danger_zone(&self, ctx: &StateProcessingContext) -> bool {
        ctx.player().opponents().iter().any(|opponent| {
            let distance = (ctx.player.position - opponent.position).magnitude();
            distance < DANGER_ZONE_RADIUS && opponent.has_ball
        })
    }

    fn get_goal_center_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        ctx.ball().direction_to_own_goal()
    }

    fn calculate_optimal_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let goal_center = self.get_goal_center_position(ctx);
        let ball_position = ctx.tick_context.object_positions.ball_position;

        // Calculate a position on the line between the ball and the center of the goal
        let to_ball = ball_position - goal_center;
        let optimal_position = goal_center + to_ball.normalize() * OPTIMAL_DISTANCE_FROM_GOAL;

        // Ensure the goalkeeper stays within the penalty area
        self.clamp_to_penalty_area(ctx, optimal_position)
    }

    fn clamp_to_penalty_area(
        &self,
        ctx: &StateProcessingContext,
        position: Vector3<f32>,
    ) -> Vector3<f32> {
        let penalty_area = ctx
            .context
            .penalty_area(ctx.player.side == Some(PlayerSide::Left));
        Vector3::new(
            position.x.clamp(penalty_area.min.x, penalty_area.max.x),
            position.y.clamp(penalty_area.min.y, penalty_area.max.y),
            0.0,
        )
    }
}
