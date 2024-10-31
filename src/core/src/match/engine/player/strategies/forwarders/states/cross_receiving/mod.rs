use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static FORWARD_CROSS_RECEIVING_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_cross_receiving_data.json"))
});

#[derive(Default)]
pub struct ForwardCrossReceivingState {}

impl StateProcessingHandler for ForwardCrossReceivingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();
        let ball_ops = ctx.ball();

        // Check if the ball is in the air
        // if !ball_ops.is_in_air() {
        //     // Transition to Running state if the ball is not in the air
        //     return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        // }

        // Check if the ball is heading towards the player
        if !ball_ops.is_towards_player() {
            // Transition to Running state if the ball is not heading towards the player
            return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        }

        // Check if the player is in a good position to receive the cross
        if !self.is_in_good_position(ctx) {
            // Move towards a better position to receive the cross
            let target_position = self.calculate_target_position(ctx);
            let direction = (target_position - ctx.player.position).normalize();
            result.velocity = Some(direction * ctx.player.skills.physical.acceleration);
        }

        // Check if the ball is within receiving range
        if ball_ops.distance() <= self.receiving_range() {
            // Attempt to receive the cross
            result
                .events
                .add_player_event(PlayerEvent::RequestBallReceive(ctx.player.id));
        }

        Some(result)
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, _ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl ForwardCrossReceivingState {
    fn is_in_good_position(&self, ctx: &StateProcessingContext) -> bool {
        let ball_position = ctx.tick_context.positions.ball.position;
        let goal_position = ctx.ball().direction_to_opponent_goal();

        // Check if the player is within the crossing zone
        let crossing_zone_width = 30.0; // Adjust based on your game's scale
        let crossing_zone_length = 20.0; // Adjust based on your game's scale
        let is_in_crossing_zone = ctx.player.position.x
            >= ball_position.x - crossing_zone_width / 2.0
            && ctx.player.position.x <= ball_position.x + crossing_zone_width / 2.0
            && ctx.player.position.y >= goal_position.y - crossing_zone_length;

        // Check if the player is not too close to opponents
        let min_distance_from_opponents = 3.0; // Adjust based on your game's scale

        let is_away_from_opponents = ctx.players().opponents().exists(min_distance_from_opponents);

        is_in_crossing_zone && is_away_from_opponents
    }

    fn calculate_target_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let ball_position = ctx.tick_context.positions.ball.position;
        let goal_position = ctx.ball().direction_to_opponent_goal();

        // Calculate the target position within the crossing zone
        let crossing_zone_width = 30.0; // Adjust based on your game's scale
        let crossing_zone_length = 20.0; // Adjust based on your game's scale
        let target_x = ball_position.x + (rand::random::<f32>() - 0.5) * crossing_zone_width;
        let target_y = goal_position.y - crossing_zone_length / 2.0;

        Vector3::new(target_x, target_y, 0.0)
    }

    fn receiving_range(&self) -> f32 {
        2.0 // Adjust based on your game's scale
    }
}
