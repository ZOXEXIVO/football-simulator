use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext,
    StateProcessingHandler, SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static DEFENDER_COVERING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_covering_data.json")));

const MARKING_DISTANCE: f32 = 15.0;
const INTERCEPTION_DISTANCE: f32 = 100.0;
const FIELD_THIRD_THRESHOLD: f32 = 0.33;
const PUSH_UP_HYSTERESIS: f32 = 0.05;

#[derive(Default)]
pub struct DefenderCoveringState {}

impl StateProcessingHandler for DefenderCoveringState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.in_state_time < 300 {
            return None;
        }

        let ball_ops = ctx.ball();
        if ball_ops.on_own_side() {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Standing,
            ));
        }

        if ball_ops.distance_to_opponent_goal()
            < ctx.context.field_size.width as f32 * (FIELD_THIRD_THRESHOLD - PUSH_UP_HYSTERESIS)
            && self.should_push_up(ctx)
        {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::PushingUp,
            ));
        }

        if let Some(_) = ctx.players().opponents().nearby(MARKING_DISTANCE).next() {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Marking,
            ));
        }

        if ball_ops.is_towards_player() && ball_ops.distance() < INTERCEPTION_DISTANCE {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Intercepting,
            ));
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        let optimal_position = self.calculate_optimal_covering_position(ctx);

        Some(
            SteeringBehavior::Pursuit {
                target: optimal_position,
                velocity: ctx.player.velocity,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl DefenderCoveringState {
    fn should_push_up(&self, ctx: &StateProcessingContext) -> bool {
        let ball_ops = ctx.ball();
        let player_ops = ctx.player();

        let ball_in_attacking_third = ball_ops.distance_to_opponent_goal()
            < ctx.context.field_size.width as f32 * FIELD_THIRD_THRESHOLD;
        let team_in_possession = ctx.team().is_control_ball();
        let defender_not_last_man = !self.is_last_defender(ctx);

        ball_in_attacking_third
            && team_in_possession
            && defender_not_last_man
            && player_ops.distance_from_start_position()
                < ctx.context.field_size.width as f32 * 0.25
    }

    fn is_last_defender(&self, ctx: &StateProcessingContext) -> bool {
        ctx.players().defenders()
            .iter()
            .all(|d| d.position.x >= ctx.player.position.x)
    }

    fn calculate_optimal_covering_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let ball_position = ctx.tick_context.object_positions.ball_position;
        let player_position = ctx.player.position;
        let field_width = ctx.context.field_size.width as f32;
        let field_height = ctx.context.field_size.height as f32;

        // Calculate the center of the middle third
        let middle_third_center = Vector3::new(field_width * 0.5, field_height * 0.5, 0.0);

        // Calculate the vector from the ball to the center of our goal
        let ball_to_goal = ctx.ball().direction_to_own_goal() - ball_position;

        // Calculate a position that's between the ball and our goal, but in the middle third
        let covering_position = ball_position + ball_to_goal * 0.4; // Adjust this factor as needed

        // Blend the covering position with the middle third center and the player's current position
        let optimal_position =
            (covering_position * 0.6 + middle_third_center * 0.3 + player_position * 0.1)
                .cap_magnitude(field_width * 0.4);

        // Ensure the optimal position is within the field boundaries
        Vector3::new(
            optimal_position.x.clamp(0.0, field_width),
            optimal_position.y.clamp(0.0, field_height),
            0.0,
        )
    }
}
