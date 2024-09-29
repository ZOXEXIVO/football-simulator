use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler, SteeringBehavior};
use crate::r#match::player::events::PlayerUpdateEvent;

static FORWARD_RUNNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_running_data.json")));

#[derive(Default)]
pub struct ForwardRunningState {}

impl StateProcessingHandler for ForwardRunningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();

        if ctx.player.has_ball {
            result.events.add(PlayerUpdateEvent::BallMoveTowardsPlayer(ctx.player.id));
        } else {
            if ctx.ball().distance() < 10.0 {
                if let Some(opponents) = ctx.tick_context.object_positions.player_distances.find_closest_opponents(ctx.player) {
                    println!("Ball find_closest_opponents = NOT EMPTY");
                    let max = opponents.iter().max_by(|(opponent_a, opponent_a_distance), (opponent_b, opponent_b_distance)|  {
                        let player_a = ctx.context.players.get(*opponent_a).unwrap();
                        let player_b = ctx.context.players.get(*opponent_b).unwrap();

                        player_a.skills.technical.tackling.partial_cmp(&player_b.skills.technical.tackling).unwrap_or(std::cmp::Ordering::Equal)
                    });

                    if let Some((opponent_id, opponent_distance)) = max {
                        result.events.add(PlayerUpdateEvent::TacklingBall(*opponent_id));
                    }
                }
            }
        }

        Some(result)
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        let player_acceleration = ctx.player.skills.physical.acceleration;

        // Get current positions
        let player_position = ctx.player.position;
        let ball_position = ctx.tick_context.object_positions.ball_position;

        // Calculate the direction vector towards the ball
        let direction_to_ball = (ball_position - player_position).normalize();
        let player_velocity = (direction_to_ball * player_acceleration).normalize();

        Some(SteeringBehavior::Pursuit {
            target: ctx.tick_context.object_positions.ball_position,
            velocity: player_velocity
        }.calculate(ctx.player).velocity)
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}
