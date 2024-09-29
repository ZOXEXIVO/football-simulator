use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
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

const BALL_DISTANCE_THRESHOLD: f32 = 10.0;

impl StateProcessingHandler for ForwardRunningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        let mut result = StateChangeResult::new();

        if ctx.player.has_ball {
            result
                .events
                .add(PlayerUpdateEvent::BallMoveTowardsPlayer(ctx.player.id));

            return Some(result);
        }

        if ctx.ball().distance() < BALL_DISTANCE_THRESHOLD {
            if let Some(opponents) = ctx
                .tick_context
                .object_positions
                .player_distances
                .find_closest_opponents(ctx.player)
            {
                  if let Some(best_tackler_id) = self.find_best_tackler(ctx) {
                    result
                        .events
                        .add(PlayerUpdateEvent::ClaimBall(best_tackler_id));
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

        Some(
            SteeringBehavior::Pursuit {
                target: ctx.tick_context.object_positions.ball_position,
                velocity: player_velocity,
            }
            .calculate(ctx.player)
            .velocity
        )
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}

impl ForwardRunningState {
    fn find_best_tackler(&self, ctx: &StateProcessingContext) -> Option<u32> {
        ctx.tick_context
            .object_positions
            .player_distances
            .find_closest_opponents(ctx.player)?
            .iter()
            .max_by(|(id_a, dist_a), (id_b, dist_b)| {
                let player_a = ctx.context.players.get(*id_a).unwrap();
                let player_b = ctx.context.players.get(*id_b).unwrap();
                let score_a = player_a.skills.technical.tackling / dist_a;
                let score_b = player_b.skills.technical.tackling / dist_b;
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(player_id, _)| *player_id)
    }
}