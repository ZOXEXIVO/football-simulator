use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::IntegerUtils;
use crate::r#match::{ConditionContext, PlayerDistanceFromStartPosition, StateChangeResult, StateProcessingContext, StateProcessingHandler, SteeringBehavior};
use crate::r#match::defenders::states::DefenderState;

static DEFENDER_WALKING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_walking_data.json")));

#[derive(Default)]
pub struct MidfielderWalkingState {}

impl StateProcessingHandler for MidfielderWalkingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. If the defender is on their own side and the ball is close, transition to Intercepting
        if ctx.ball().is_towards_player_with_angle(0.8) && ctx.ball().distance() < 150.0 {
            return Some(StateChangeResult::with_defender_state(DefenderState::Intercepting));
        }

        // 2. If the defender is far from their starting position, transition to Returning
        if ctx.player().position_to_distance() != PlayerDistanceFromStartPosition::Small {
            return Some(StateChangeResult::with_defender_state(DefenderState::Returning));
        }

        // 3. Remain in Walking state
        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network logic if necessary
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // 1. If this is the first tick in the state, initialize wander behavior
        if ctx.in_state_time == 0 {
            let wander_behavior = SteeringBehavior::Wander {
                target: ctx.player.start_position,
                radius: IntegerUtils::random(5, 100) as f32,
                jitter: IntegerUtils::random(1, 5) as f32,
                distance: IntegerUtils::random(10, 150) as f32,
                angle: IntegerUtils::random(0, 360) as f32,
            };

            // Store the wander behavior in the player's state if needed
            // For simplicity, we'll calculate and return the velocity directly
            let velocity = wander_behavior.calculate(ctx.player).velocity;

            return Some(velocity);
        }

        None
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions
    }
}