use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static FORWARD_RUNNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_running_data.json")));

#[derive(Default)]
pub struct ForwardRunningState {}

const BALL_DISTANCE_THRESHOLD: f32 = 20.0;
const MAX_PLAYER_SPEED: f32 = 50.0;

impl StateProcessingHandler for ForwardRunningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // if !ctx.player.has_ball {
        //     return Some(StateChangeResult::with_forward_state(
        //         ForwardState::Assisting,
        //     ));
        // }

        if ctx.ball().distance_to_opponent_goal() < 300.0 {
            return Some(StateChangeResult::with_forward_state(
                ForwardState::Shooting,
            ));
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        if ctx.player.has_ball {
            let goal_direction = ctx.ball().direction_to_opponent_goal();

            let player_goal_velocity = SteeringBehavior::Arrive {
                target: goal_direction,
                slowing_distance: 10.0,
            }
            .calculate(ctx.player)
            .velocity;

            Some(player_goal_velocity)
        } else {
            let player_acceleration = ctx.player.skills.physical.acceleration;
            let player_pace = ctx.player.skills.physical.pace;
            let player_stamina = ctx.player.skills.physical.stamina;
            let player_agility = ctx.player.skills.physical.agility;

            // Get current positions
            let player_position = ctx.player.position;
            let ball_position = ctx.tick_context.object_positions.ball_position;

            // Calculate the direction vector towards the ball
            let direction_to_ball = (ball_position - player_position).normalize();

            // Calculate player speed based on their attributes
            // Normalize each attribute to a 0-1 range assuming they're on a 0-100 scale
            let normalized_pace = player_pace / 100.0;
            let normalized_acceleration = player_acceleration / 20.0;
            let normalized_stamina = player_stamina / 100.0;
            let normalized_agility = player_agility / 100.0;

            // Combine attributes to determine speed
            // We're giving more weight to pace and acceleration
            let speed = (normalized_pace * 0.4
                + normalized_acceleration * 1.3
                + normalized_stamina * 0.2
                + normalized_agility * 0.1)
                * MAX_PLAYER_SPEED;

            // Calculate player velocity
            let player_velocity = direction_to_ball * speed;

            // Apply pursuit behavior
            let pursuit_result = SteeringBehavior::Pursuit {
                target: ball_position,
                velocity: player_velocity,
            }
            .calculate(ctx.player);

            Some(pursuit_result.velocity)
        }
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}
