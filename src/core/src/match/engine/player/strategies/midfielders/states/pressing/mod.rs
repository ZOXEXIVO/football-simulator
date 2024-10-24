use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static MIDFIELDER_PRESSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_pressing_data.json")));

const PRESSING_DISTANCE_THRESHOLD: f32 = 10.0; // Max distance to consider pressing
const STAMINA_THRESHOLD: f32 = 50.0; // Minimum stamina to continue pressing

#[derive(Default)]
pub struct MidfielderPressingState {}

impl StateProcessingHandler for MidfielderPressingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if !ctx.ball().is_owned() || ctx.player.has_ball {
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Running,
            ));
        }

        // 1. Check if the midfielder has enough stamina to continue pressing
        let stamina = ctx.player.player_attributes.condition_percentage() as f32;
        if stamina < STAMINA_THRESHOLD {
            // Transition to Standing state if stamina is low
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Standing,
            ));
        }

        if ctx.team().is_control_ball() {
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::SupportingAttack,
            ));
        }

        // 2. Identify the opponent player with the ball
        let players = ctx.players();
        let opponents = players.opponents();

        if let Some(opponent) = opponents.with_ball().next() {
            // 3. Calculate the distance to the opponent
            let distance_to_opponent = (ctx.player.position - opponent.position).magnitude();

            // 4. If the opponent is too far away, stop pressing
            if distance_to_opponent > PRESSING_DISTANCE_THRESHOLD {
                // Transition to Standing state
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Standing,
                ));
            }
        } else {
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network processing if needed
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        if ctx.ball().distance() < 150.0 {
            Some(
                SteeringBehavior::Pursuit {
                    target: ctx.tick_context.object_positions.ball_position,
                    velocity: ctx.player.velocity,
                }
                .calculate(ctx.player)
                .velocity,
            )
        } else {
            Some(
                SteeringBehavior::Arrive {
                    target: ctx.player.position,
                    slowing_distance: 1.0,
                }
                .calculate(ctx.player)
                .velocity,
            )
        }
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions to process in this state
    }
}
