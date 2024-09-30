use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static FORWARD_PRESSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_pressing_data.json")));

#[derive(Default)]
pub struct ForwardPressingState {}

impl StateProcessingHandler for ForwardPressingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();
        let ball_ops = ctx.ball();
        let player_ops = ctx.player();

        // Check if the ball is on the opponent's side
        if ball_ops.on_own_side() {
            // Transition to Standing state if the ball is on own side
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Standing,
            ));
        }

        // Check if the player is the closest to the ball
        if let Some((closest_player_id, _)) = ctx
            .tick_context
            .object_positions
            .player_distances
            .find_closest_to_ball(ctx.tick_context.object_positions.ball_position)
        {
            if closest_player_id != ctx.player.id {
                // Transition to Running state if not the closest player to the ball
                return Some(StateChangeResult::with_forward_state(ForwardState::Running));
            }
        }

        // Check if the player has the ball
        if ctx.player.has_ball {
            // Transition to Dribbling state if the player has the ball
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Dribbling,
            ));
        }

        // Check if the player is under pressure
        if player_ops.is_under_pressure(ctx) {
            // Transition to Tackling state if under pressure and close to the ball
            if ball_ops.distance() < 2.0 {
                return Some(StateChangeResult::with_forward_state(
                    ForwardState::Tackling,
                ));
            } else {
                // Transition to Running state if under pressure but not close to the ball
                return Some(StateChangeResult::with_forward_state(ForwardState::Running));
            }
        }

        // Find the closest opponent
        if let Some((opponent_id, distance)) = ctx
            .tick_context
            .object_positions
            .player_distances
            .find_closest_opponent(&ctx.player)
        {
            let opponent = ctx.context.players.get(opponent_id).unwrap();

            // Check if the opponent has the ball
            if opponent.has_ball {
                // Move towards the opponent with the ball
                let direction = (opponent.position - ctx.player.position).normalize();
                result.velocity = Some(direction * ctx.player.skills.physical.acceleration);
            } else if distance < 5.0 {
                // Move towards the opponent if close enough
                let direction = (opponent.position - ctx.player.position).normalize();
                result.velocity = Some(direction * ctx.player.skills.physical.acceleration);
            } else {
                // Move towards the ball if the opponent is far
                let direction = (ctx.tick_context.object_positions.ball_position
                    - ctx.player.position)
                    .normalize();
                result.velocity = Some(direction * ctx.player.skills.physical.acceleration);
            }
        } else {
            // Move towards the ball if no close opponents
            let direction =
                (ctx.tick_context.object_positions.ball_position - ctx.player.position).normalize();
            result.velocity = Some(direction * ctx.player.skills.physical.acceleration);
        }

        Some(result)
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}
