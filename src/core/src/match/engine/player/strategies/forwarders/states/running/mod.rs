use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::{ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext, StateProcessingHandler, SteeringBehavior};
use nalgebra::Vector3;
use std::sync::LazyLock;
use rand::prelude::SliceRandom;
use crate::r#match::events::Event;
use crate::r#match::player::events::PlayerEvent;

static FORWARD_RUNNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_running_data.json")));

#[derive(Default)]
pub struct ForwardRunningState {}

const PRESSING_DISTANCE_THRESHOLD: f32 = 50.0;
const BALL_DISTANCE_THRESHOLD: f32 = 20.0;
const MAX_PLAYER_SPEED: f32 = 50.0;
const SHOOTING_DISTANCE_THRESHOLD: f32 = 200.0;
const PASSING_DISTANCE_THRESHOLD: f32 = 500.0;
const ASSISTING_DISTANCE_THRESHOLD: f32 = 200.0;
const TARGET_REACHED_THRESHOLD: f32 = 10.0;

impl StateProcessingHandler for ForwardRunningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let distance_to_ball = ctx.ball().distance();
        let distance_to_goal = ctx.ball().distance_to_opponent_goal();

        if ctx.player.has_ball {
            let (_, opponents_count) = ctx.tick_context.object_positions.
                player_distances.players_within_distance_count(ctx.player, 100.0);

            if opponents_count > 1 {
                return Some(StateChangeResult::with_forward_state(ForwardState::Passing));
            }

            if distance_to_goal < SHOOTING_DISTANCE_THRESHOLD {
                return Some(StateChangeResult::with_forward_state(ForwardState::Shooting));
            }

            if distance_to_goal > PASSING_DISTANCE_THRESHOLD {
                return Some(StateChangeResult::with_forward_state(ForwardState::Passing));
            }
        } else {
            if let Some(opponent_with_ball) = ctx.player().opponent_with_ball().first() {
                let opponent_distance = ctx.tick_context.object_positions.player_distances.get(ctx.player.id, opponent_with_ball.id).unwrap();
                if opponent_distance < PRESSING_DISTANCE_THRESHOLD {
                    return Some(StateChangeResult::with_forward_state(ForwardState::Pressing));
                }
            }

            if distance_to_ball > ASSISTING_DISTANCE_THRESHOLD {
                return Some(StateChangeResult::with_forward_state(ForwardState::Assisting));
            }
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
                + normalized_acceleration * 0.3
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

impl ForwardRunningState {
}