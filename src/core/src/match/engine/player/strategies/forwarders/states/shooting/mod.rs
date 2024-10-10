use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;
use crate::r#match::events::Event;
use crate::r#match::player::events::PlayerEvent;

static FORWARD_SHOOTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_shooting_data.json")));

#[derive(Default)]
pub struct ForwardShootingState {}

impl StateProcessingHandler for ForwardShootingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        return Some(StateChangeResult::with_forward_state_and_event(
            ForwardState::Standing,
            Event::PlayerEvent(PlayerEvent::Shoot(ctx.player.id, ctx.ball().direction_to_opponent_goal())),
        ));

        // // Check if the player still has the ball
        // if !ctx.player.has_ball {
        //     // If the player doesn't have the ball, transition to Running state
        //     return Some(StateChangeResult::with_forward_state(ForwardState::Running));
        // }
        //
        // // Check if the player is still in a good shooting position
        // if !self.is_in_shooting_range(ctx) {
        //     // If not in a good shooting position, consider passing or dribbling
        //     if let Some(_) = self.find_best_teammate_to_pass(ctx) {
        //         return Some(StateChangeResult::with_forward_state(ForwardState::Passing));
        //     } else {
        //         return Some(StateChangeResult::with_forward_state(
        //             ForwardState::Dribbling,
        //         ));
        //     }
        // }
        //
        // // Check if there's an immediate threat from an opponent
        // if self.is_under_pressure(ctx) {
        //     // If under pressure, decide between quick shot or passing
        //     if self.should_take_quick_shot(ctx) {
        //         result
        //             .events
        //             .add(PlayerEvent::Shoot(ctx.player.id, direction));
        //     } else if let Some(teammate) = self.find_best_teammate_to_pass(ctx) {
        //         return Some(StateChangeResult::with_forward_state(ForwardState::Passing));
        //     }
        // } else {
        //     // If not under immediate pressure, take the shot
        //     result
        //         .events
        //         .add(PlayerEvent::Shoot(ctx.player.id, direction));
        // }
        //
        // Some(result)
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}

impl ForwardShootingState {
    fn is_in_shooting_range(&self, ctx: &StateProcessingContext) -> bool {
        let distance_to_goal = ctx.ball().distance_to_opponent_goal();
        distance_to_goal <= 30.0 // Adjust this value based on your game's scale
    }

    fn is_under_pressure(&self, ctx: &StateProcessingContext) -> bool {
        if let Some((_, distance)) = ctx
            .tick_context
            .object_positions
            .player_distances
            .find_closest_opponent(ctx.player)
        {
            distance < 5.0 // Adjust this value based on your game's scale
        } else {
            false
        }
    }

    fn should_take_quick_shot(&self, ctx: &StateProcessingContext) -> bool {
        // This could be a more complex decision based on player skills, positioning, etc.
        ctx.player.skills.technical.finishing > 70.0
    }

    fn find_best_teammate_to_pass(&self, ctx: &StateProcessingContext) -> Option<u32> {
        ctx.tick_context
            .object_positions
            .player_distances
            .find_closest_teammates(ctx.player)
            .and_then(|teammates| teammates.first().map(|(id, _)| *id))
    }
}
