use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext,
    StateProcessingHandler, SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

static FORWARD_RUNNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_running_data.json")));

const CREATING_SPACE_THRESHOLD: f32 = 100.0; // Adjust based on your game's scale
const OPPONENT_DISTANCE_THRESHOLD: f32 = 5.0; // Adjust based on your game's scale
const VELOCITY_CHANGE_THRESHOLD: f32 = 2.0; // Adjust based on your game's scale

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
            let (_, opponents_count) = ctx
                .tick_context
                .object_positions
                .player_distances
                .players_within_distance_count(ctx.player, 100.0);

            if opponents_count > 1 {
                return Some(StateChangeResult::with_forward_state(ForwardState::Passing));
            }

            if distance_to_goal < SHOOTING_DISTANCE_THRESHOLD {
                return Some(StateChangeResult::with_forward_state(
                    ForwardState::Shooting,
                ));
            }

            if distance_to_goal > PASSING_DISTANCE_THRESHOLD {
                return Some(StateChangeResult::with_forward_state(ForwardState::Passing));
            }
        } else {
            if !self.is_leading_forward(ctx) {
                // If not the leading forward, transition to a supporting state
                return Some(StateChangeResult::with_forward_state(
                    ForwardState::Assisting,
                ));
            }

            if let Some(opponent_with_ball) = ctx.player().opponent_with_ball().first() {
                let opponent_distance = ctx
                    .tick_context
                    .object_positions
                    .player_distances
                    .get(ctx.player.id, opponent_with_ball.id)
                    .unwrap();
                if opponent_distance < PRESSING_DISTANCE_THRESHOLD {
                    return Some(StateChangeResult::with_forward_state(
                        ForwardState::Pressing,
                    ));
                }
            }

            if distance_to_ball > ASSISTING_DISTANCE_THRESHOLD {
                return Some(StateChangeResult::with_forward_state(
                    ForwardState::Assisting,
                ));
            }
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        if self.has_space_between_opponents(ctx){
            return Some(SteeringBehavior::Arrive {
                target: self.calculate_target_position(ctx),
                slowing_distance: 30.0,
            }.calculate(ctx.player).velocity);
        }

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
            let speed = (normalized_pace * 0.6
                + normalized_acceleration * 0.9
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
    fn is_leading_forward(&self, ctx: &StateProcessingContext) -> bool {
        let players = ctx.player();
        let forwards = players.forwards_teammates();

        let (leading_forward, _) = forwards.iter().fold(
            (None, f32::MIN),
            |(leading_player, max_score), player| {
                let distance = (player.position - ctx.tick_context.object_positions.ball_position).magnitude();
                let speed = player.skills.max_speed();
                let time_to_ball = distance / speed;

                let score = player.skills.technical.average() + player.skills.mental.average() - time_to_ball;

                if score > max_score {
                    (Some(player), score)
                } else {
                    (leading_player, max_score)
                }
            },
        );

        if let Some(leading_forward) = leading_forward {
            if leading_forward.id == ctx.player.id {
                // The current player is the leading forward
                true
            } else {
                // Check if the current player is within a certain range of the leading forward
                let distance_to_leading_forward = (ctx.player.position - leading_forward.position).magnitude();
                if distance_to_leading_forward <= ASSISTING_DISTANCE_THRESHOLD {
                    // The current player is close enough to the leading forward to be considered assisting
                    false
                } else {
                    // Check if the current player has a better score than the leading forward
                    let player_distance = (ctx.player.position - ctx.tick_context.object_positions.ball_position).magnitude();
                    let player_speed = ctx.player.skills.max_speed();
                    let player_time_to_ball = player_distance / player_speed;

                    let player_score = ctx.player.skills.technical.average() + ctx.player.skills.mental.average() - player_time_to_ball;

                    let leading_forward_distance = (leading_forward.position - ctx.tick_context.object_positions.ball_position).magnitude();
                    let leading_forward_speed = leading_forward.skills.max_speed();
                    let leading_forward_time_to_ball = leading_forward_distance / leading_forward_speed;

                    let leading_forward_score = leading_forward.skills.technical.average() + leading_forward.skills.mental.average() - leading_forward_time_to_ball;

                    player_score > leading_forward_score
                }
            }
        } else {
            // No other forwards, so the current player is the leading forward
            true
        }
    }

    fn calculate_target_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let player_position = ctx.player.position;
        let field_half_length = ctx.context.field_size.width as f32 / 2.0;
        let field_width = ctx.context.field_size.width as f32;

        let target_x = field_half_length + (field_half_length - player_position.x) * 0.8;
        let target_y = player_position.y + (field_width / 4.0) * (rand::random::<f32>() - 0.5);

        Vector3::new(target_x, target_y, 0.0)
    }

    fn has_space_between_opponents(&self, ctx: &StateProcessingContext) -> bool {
        let nearest_opponents = ctx.tick_context.object_positions.player_distances
            .find_closest_opponents(ctx.player);

        if let Some(opponents) = nearest_opponents {
            if opponents.len() >= 2 {
                let opponent1_position = ctx.context.players.get(opponents[0].0).unwrap().position;
                let opponent2_position = ctx.context.players.get(opponents[1].0).unwrap().position;

                let distance_between_opponents = (opponent1_position - opponent2_position).magnitude();
                distance_between_opponents > CREATING_SPACE_THRESHOLD
            } else {
                false
            }
        } else {
            false
        }
    }

}
