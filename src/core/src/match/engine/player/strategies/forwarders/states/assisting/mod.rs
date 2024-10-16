use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

const KICK_POWER_MULTIPLIER: f32 = 1.5; // Multiplier for kick power calculation

static FORWARD_ASSISTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_assisting_data.json")));

#[derive(Default)]
pub struct ForwardAssistingState {}

impl StateProcessingHandler for ForwardAssistingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();

        if !ctx.team().is_control_ball() && ctx.ball().distance() < 150.0{
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Pressing,
            ));
        }

        // Check if the player is on the opponent's side of the field
        if !self.is_on_opponent_side(ctx) {
            // If not on the opponent's side, focus on creating space and moving forward
            return Some(StateChangeResult::with_forward_state(
                ForwardState::CreatingSpace,
            ));
        }

        // Check if there's an immediate threat from an opponent
        if self.is_under_pressure(ctx) {
            // If under high pressure, decide between quick pass or dribbling
            if self.should_make_quick_pass(ctx) {
                if let Some(teammate_id) = self.find_best_teammate_to_assist(ctx) {
                    //result.events.add_player_event(PlayerEvent::Pass(ctx.player.player_id, teammate_id));
                    return Some(result);
                }
            }
            // If no good passing option, try to dribble
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Dribbling,
            ));
        }

        // If not under immediate pressure, look for assist opportunities
        if let Some(teammate_id) = self.find_best_teammate_to_assist(ctx) {
            if self.is_good_assisting_position(ctx, teammate_id) {
                let teammate_position = ctx
                    .tick_context
                    .object_positions
                    .players_positions
                    .get_player_position(teammate_id)
                    .unwrap();

                // Make the assist
                let distance_to_teammate = (ctx.player.position - teammate_position).magnitude();
                let kick_power = distance_to_teammate / ctx.player.skills.technical.free_kicks
                    * KICK_POWER_MULTIPLIER;

                result.events.add_player_event(PlayerEvent::PassTo(
                    teammate_id,
                    teammate_position,
                    kick_power as f64,
                ));
                return Some(result);
            }
        }

        // If no good assist opportunity, consider other options
        if self.is_in_shooting_range(ctx) && ctx.player.has_ball {
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Shooting,
            ));
        } else if self.should_create_space(ctx) {
            return Some(StateChangeResult::with_forward_state(
                ForwardState::CreatingSpace,
            ));
        } else if self.should_ask_for_pass(ctx) {
            result
                .events
                .add_player_event(PlayerEvent::RequestPass(ctx.player.id));
            return Some(result);
        }

        // If no clear action, continue in the current state
        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        if let Some(forwards) = ctx.player().forwards_teammates().first() {
            Some(
                SteeringBehavior::Flee {
                    target: forwards.position,
                }
                .calculate(ctx.player)
                .velocity,
            )
        } else {
            Some(
                SteeringBehavior::Arrive {
                    target: ctx.ball().direction_to_opponent_goal(),
                    slowing_distance: 200.0,
                }
                .calculate(ctx.player)
                .velocity,
            )
        }
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}

impl ForwardAssistingState {
    fn is_under_pressure(&self, ctx: &StateProcessingContext) -> bool {
        if let Some((_, distance)) = ctx
            .tick_context
            .object_positions
            .player_distances
            .find_closest_opponent(ctx.player)
        {
            distance < 3.0 // Adjust based on your game's scale
        } else {
            false
        }
    }

    fn should_make_quick_pass(&self, ctx: &StateProcessingContext) -> bool {
        // Decision based on player's skills and game situation
        ctx.player.skills.technical.passing > 70.0 && ctx.player.skills.mental.decisions > 65.0
    }

    fn find_best_teammate_to_assist(&self, ctx: &StateProcessingContext) -> Option<u32> {
        ctx.tick_context
            .object_positions
            .player_distances
            .find_closest_teammates(ctx.player)
            .and_then(|teammates| {
                teammates
                    .iter()
                    .filter(|(id, _)| self.is_in_good_scoring_position(ctx, *id))
                    .min_by(|(_, dist_a), (_, dist_b)| {
                        dist_a
                            .partial_cmp(dist_b)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(id, _)| *id)
            })
    }

    fn is_good_assisting_position(&self, ctx: &StateProcessingContext, teammate_id: u32) -> bool {
        // Complex logic to determine if the current position is good for assisting
        // This could involve checking angles, distances, and opponent positions
        // Simplified version:
        if let Some(teammate) = ctx.context.players.get(teammate_id) {
            if let Some(pass_distance) = ctx
                .tick_context
                .object_positions
                .player_distances
                .get(ctx.player.id, teammate.id)
            {
                return pass_distance > 5.0 && pass_distance < 30.0;
            }
        }
        false
    }

    fn is_in_good_scoring_position(&self, ctx: &StateProcessingContext, player_id: u32) -> bool {
        if let Some(player) = ctx.context.players.get(player_id) {
            let distance_to_goal = ctx.ball().distance_to_opponent_goal();
            distance_to_goal < 20.0 // Adjust based on your game's scale
        } else {
            false
        }
    }

    fn is_in_shooting_range(&self, ctx: &StateProcessingContext) -> bool {
        let distance_to_goal = ctx.ball().distance_to_opponent_goal();
        distance_to_goal < 25.0 // Adjust based on your game's scale
    }

    fn should_create_space(&self, ctx: &StateProcessingContext) -> bool {
        // Logic to decide if the player should focus on creating space
        // This could involve checking team tactics, player positions, etc.
        // Simplified version:
        ctx.player.skills.mental.off_the_ball > 75.0
            && ctx
                .tick_context
                .object_positions
                .player_distances
                .find_closest_teammates(ctx.player)
                .map_or(false, |teammates| teammates.len() < 2)
    }

    fn is_on_opponent_side(&self, ctx: &StateProcessingContext) -> bool {
        let field_half_length = ctx.context.field_size.width as f32 / 2.0;
        ctx.player.position.x > field_half_length
    }

    fn should_ask_for_pass(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the player is in a good position to receive a pass
        let player_distance_from_goal = ctx.ball().distance_to_opponent_goal();
        let has_good_space = ctx
            .tick_context
            .object_positions
            .is_big_opponents_concentration(ctx.player);

        player_distance_from_goal < 30.0
            && has_good_space
            && ctx.player.skills.mental.off_the_ball > 70.0
    }
}
