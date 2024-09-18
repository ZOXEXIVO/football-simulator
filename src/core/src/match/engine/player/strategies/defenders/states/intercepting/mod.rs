use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::{
    PlayerDistanceFromStartPosition, StateChangeResult, StateProcessingContext,
    StateProcessingHandler, SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static DEFENDER_INTERCEPTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_intercepting_data.json")));

#[derive(Default)]
pub struct DefenderInterceptingState {}

impl StateProcessingHandler for DefenderInterceptingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.ball().on_own_side() {
            if ctx.player().position_to_distance() == PlayerDistanceFromStartPosition::Big {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::TrackingBack,
                ));
            }

            let distance_to_ball = ctx.ball().distance();

            if distance_to_ball < 10.0 {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::Intercepting,
                ));
            }

            if distance_to_ball >= 10.0 && distance_to_ball < 20.0 {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::Marking,
                ));
            }

            if distance_to_ball >= 20.0 {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::HoldingLine,
                ));
            }
        } else {
            if ctx.player().position_to_distance() == PlayerDistanceFromStartPosition::Small {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::OffsideTrap,
                ));
            }
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> StateChangeResult {
        StateChangeResult::none()
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Player's acceleration
        let player_acceleration = ctx.player.skills.physical.acceleration;

        // Get current positions
        let player_position = ctx.player.position;
        let ball_position = ctx.tick_context.objects_positions.ball_position;

        // Calculate the direction vector towards the ball
        let direction_to_ball = (ball_position - player_position).normalize();

        // Scale direction by player's acceleration to get velocity
        let player_velocity = (direction_to_ball * player_acceleration).normalize();

        Some(
            SteeringBehavior::Pursuit {
                target: ctx.tick_context.objects_positions.ball_position,
                velocity: player_velocity,
            }
            .calculate(ctx.player)
            .velocity,
        )
    }
}
