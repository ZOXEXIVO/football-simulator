use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::position::VectorExtensions;
use crate::r#match::{
    ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext,
    StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;
use crate::r#match::player::events::PlayerEvent;

static FORWARD_HEADING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_heading_data.json")));

#[derive(Default)]
pub struct ForwardHeadingState {}

impl StateProcessingHandler for ForwardHeadingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();

        // Check if the ball is within heading range
        if !self.is_ball_within_heading_range(ctx) {
            // Transition to Running state if the ball is not within heading range
            return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        }

        // Check if the player can jump higher than nearby opponents
        if !self.can_outjump_opponents(ctx) {
            // Transition to Running state if the player can't outjump opponents
            return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        }

        // Calculate the heading direction
        let heading_direction = self.calculate_heading_direction(ctx);

        // Perform the heading action
        result.events.add_player_event(PlayerEvent::RequestHeading(
            ctx.player.id,
            heading_direction,
        ));

        // Transition to Running state after heading the ball
        Some(StateChangeResult::with_forward_state(ForwardState::Running))
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}

impl ForwardHeadingState {
    fn is_ball_within_heading_range(&self, ctx: &StateProcessingContext) -> bool {
        let ball_position = ctx.tick_context.object_positions.ball_position;
        let heading_range = 1.5; // Adjust based on your game's scale

        ctx.player.position.distance_to(&ball_position) <= heading_range
    }

    fn can_outjump_opponents(&self, ctx: &StateProcessingContext) -> bool {
        let player_jumping_reach = ctx.player.skills.physical.jumping;
        let opponents_close_to_ball = self.get_opponents_close_to_ball(ctx);

        // Check if the player's jumping reach is higher than the opponents' jumping reach
        opponents_close_to_ball
            .iter()
            .all(|opponent| player_jumping_reach > opponent.skills.physical.jumping)
    }

    fn get_opponents_close_to_ball<'a>(
        &self,
        ctx: &StateProcessingContext<'a>,
    ) -> Vec<&'a MatchPlayer> {
        let ball_position = ctx.tick_context.object_positions.ball_position;
        let close_distance = 2.0; // Adjust based on your game's scale

        ctx.context
            .players
            .get_by_not_team(ctx.player.team_id, None)
            .into_iter()
            .filter(|opponent| opponent.position.distance_to(&ball_position) <= close_distance)
            .collect()
    }

    fn calculate_heading_direction(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let goal_position = ctx.ball().direction_to_opponent_goal();
        let ball_position = ctx.tick_context.object_positions.ball_position;

        (goal_position - ball_position).normalize()
    }
}
