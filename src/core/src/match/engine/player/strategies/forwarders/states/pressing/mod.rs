use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
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

        if ctx.player.has_ball {
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Dribbling,
            ));
        }

        if ctx.team().is_control_ball() {
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Assisting,
            ));
        }

        let ball_ops = ctx.ball();

        // Check if the ball is on the opponent's side
        if ball_ops.on_own_side() {
            // Transition to Standing state if the ball is on own side
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Standing,
            ));
        }

        let players = ctx.players();
        let opponents = players.opponents();
        if let Some(opponent) = opponents.nearby(100.0).next()
        {
            // Check if the opponent has the ball
            if opponent.has_ball {
                // Move towards the opponent with the ball
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

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(
            SteeringBehavior::Arrive {
                target: ctx.tick_context.object_positions.ball_position,
                slowing_distance: 10.0,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}
