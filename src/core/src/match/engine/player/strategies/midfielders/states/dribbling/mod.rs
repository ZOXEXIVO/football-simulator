use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::events::Event;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;
use rand::prelude::IteratorRandom;

static MIDFIELDER_DRIBBLING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_dribbling_data.json")));

#[derive(Default)]
pub struct MidfielderDribblingState {}

impl StateProcessingHandler for MidfielderDribblingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.player.has_ball {
            // If the player has the ball, consider shooting, passing, or dribbling
            if self.is_in_shooting_position(ctx) {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::DistanceShooting,
                ));
            }

            if let Some(teammate_id) = self.find_open_teammate(ctx) {
                return Some(StateChangeResult::with_midfielder_state_and_event(
                    MidfielderState::ShortPassing,
                    Event::PlayerEvent(PlayerEvent::RequestPass(teammate_id)),
                ));
            }

            // If no shooting or passing options, consider dribbling
            if self.should_dribble(ctx) {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Dribbling,
                ));
            }
        } else {
            // If the player doesn't have the ball, check if they should press, support attack, or return
            if self.should_press(ctx) {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Pressing,
                ));
            }

            if self.should_support_attack(ctx) {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::SupportingAttack,
                ));
            }

            if self.should_return_to_position(ctx) {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Returning,
                ));
            }
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, _ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        None
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl MidfielderDribblingState {
    fn find_open_teammate<'a>(&self, ctx: &StateProcessingContext<'a>) -> Option<u32> {
        // Find an open teammate to pass to
        let players = ctx.players();
        let teammates = players.teammates();

        let teammates = teammates.nearby_raw(50.0);

        if let Some((teammate_id, _)) = teammates.choose(&mut rand::thread_rng()) {
            return Some(teammate_id)
        }

        None
    }

    fn is_in_shooting_position(&self, ctx: &StateProcessingContext) -> bool {
        let shooting_range = 25.0; // Distance from goal to consider shooting
        let player_position = ctx.player.position;
        let goal_position = ctx.ball().direction_to_opponent_goal();

        let distance_to_goal = (player_position - goal_position).magnitude();

        distance_to_goal <= shooting_range
    }

    fn should_dribble(&self, ctx: &StateProcessingContext) -> bool {
        // Check if there is space to dribble and no immediate pressure from opponents
        let space_ahead = self.space_ahead(ctx);
        let under_pressure = self.is_under_pressure(ctx);

        space_ahead && !under_pressure
    }

    fn should_support_attack(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the team is in possession and the player is in a good position to support the attack
        let team_in_possession = ctx.team().is_control_ball();
        let in_attacking_half = ctx.player.position.x > ctx.context.field_size.width as f32 / 2.0;

        team_in_possession && in_attacking_half
    }

    fn should_return_to_position(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the player is far from their starting position and the team is not in possession
        let distance_from_start = ctx.player().distance_from_start_position();
        let team_in_possession = ctx.team().is_control_ball();

        distance_from_start > 20.0 && !team_in_possession
    }

    fn space_ahead(&self, ctx: &StateProcessingContext) -> bool {
        // Check if there is open space ahead of the player
        let space_threshold = 10.0;
        let player_direction = ctx.player.velocity.normalize();
        let space_ahead = ctx.tick_context.space.cast_ray(
            ctx.player.position,
            player_direction,
            space_threshold,
            true,
        );

        space_ahead.is_none()
    }

    fn should_press(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the player should press the opponent with the ball
        let ball_distance = ctx.ball().distance();
        let pressing_distance = 150.0; // Adjust the threshold as needed

        !ctx.team().is_control_ball() && ball_distance < pressing_distance
    }

    fn is_under_pressure(&self, ctx: &StateProcessingContext) -> bool {
        let pressure_distance = 10.0;
        ctx.players().teammates().exists(pressure_distance)
    }
}
