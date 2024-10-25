use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
    SteeringBehavior,
};
use nalgebra::Vector3;
use std::sync::LazyLock;

const MAX_SHOOTING_DISTANCE: f32 = 300.0; // Maximum distance to attempt a shot
const MIN_SHOOTING_DISTANCE: f32 = 20.0; // Minimum distance to attempt a shot (e.g., edge of penalty area)

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
        let distance_to_goal = ctx.ball().distance_to_opponent_goal();

        if ctx.player.has_ball {
            if self.is_in_shooting_range(ctx) {
                return Some(StateChangeResult::with_forward_state(
                    ForwardState::Shooting,
                ));
            }

            if ctx.players().opponents().nearby_raw(100.0).count() > 1 {
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

            let players = ctx.players();
            let opponents = players.opponents();

            if let Some(opponent_with_ball) = opponents.with_ball().next() {
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

            if ctx.ball().distance() > ASSISTING_DISTANCE_THRESHOLD {
                return Some(StateChangeResult::with_forward_state(
                    ForwardState::Assisting,
                ));
            }
        }

        None
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
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
            // Apply pursuit behavior
            let pursuit_result = SteeringBehavior::Arrive {
                target: ctx.tick_context.object_positions.ball_position,
                slowing_distance: 30.0,
            }
            .calculate(ctx.player);

            Some(pursuit_result.velocity)
        }
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl ForwardRunningState {
    fn is_in_shooting_range(&self, ctx: &StateProcessingContext) -> bool {
        let distance_to_goal = ctx.ball().distance_to_opponent_goal();
        distance_to_goal <= MAX_SHOOTING_DISTANCE && distance_to_goal >= MIN_SHOOTING_DISTANCE
    }

    fn is_leading_forward(&self, ctx: &StateProcessingContext) -> bool {
        let players = ctx.players();
        let teammates = players.teammates();

        let forwards = teammates.forwards();

        let (leading_forward, _) =
            forwards
                .fold((None, f32::MIN), |(leading_player, max_score), player| {
                    let distance = (player.position
                        - ctx.tick_context.object_positions.ball_position)
                        .magnitude();
                    let speed = player.skills.max_speed();
                    let time_to_ball = distance / speed;

                    let score = player.skills.technical.average() + player.skills.mental.average()
                        - time_to_ball;

                    if score > max_score {
                        (Some(player), score)
                    } else {
                        (leading_player, max_score)
                    }
                });

        if let Some(leading_forward) = leading_forward {
            if leading_forward.id == ctx.player.id {
                // The current player is the leading forward
                true
            } else {
                // Check if the current player is within a certain range of the leading forward
                let distance_to_leading_forward =
                    (ctx.player.position - leading_forward.position).magnitude();
                if distance_to_leading_forward <= ASSISTING_DISTANCE_THRESHOLD {
                    // The current player is close enough to the leading forward to be considered assisting
                    false
                } else {
                    // Check if the current player has a better score than the leading forward
                    let player_distance = (ctx.player.position
                        - ctx.tick_context.object_positions.ball_position)
                        .magnitude();
                    let player_speed = ctx.player.skills.max_speed();
                    let player_time_to_ball = player_distance / player_speed;

                    let player_score = ctx.player.skills.technical.average()
                        + ctx.player.skills.mental.average()
                        - player_time_to_ball;

                    let leading_forward_distance = (leading_forward.position
                        - ctx.tick_context.object_positions.ball_position)
                        .magnitude();
                    let leading_forward_speed = leading_forward.skills.max_speed();
                    let leading_forward_time_to_ball =
                        leading_forward_distance / leading_forward_speed;

                    let leading_forward_score = leading_forward.skills.technical.average()
                        + leading_forward.skills.mental.average()
                        - leading_forward_time_to_ball;

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
        let players = ctx.players();
        let opponents = players.opponents();

        let mut nearest_opponents = opponents.nearby(150.0);

        if let Some(first) = nearest_opponents.next() {
            if let Some(second) = nearest_opponents.next() {
                let opponent1_position = first.position;
                let opponent2_position = second.position;

                let distance_between_opponents =
                    (opponent1_position - opponent2_position).magnitude();

                return distance_between_opponents > CREATING_SPACE_THRESHOLD;
            }
        }

        false
    }
}
