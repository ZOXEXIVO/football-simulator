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
const MAX_PLAYER_SPEED: f32 = 50.0;

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
        let normalized_acceleration = player_acceleration / 100.0;
        let normalized_stamina = player_stamina / 100.0;
        let normalized_agility = player_agility / 100.0;

        // Combine attributes to determine speed
        // We're giving more weight to pace and acceleration
        let speed = (normalized_pace * 0.4 +
            normalized_acceleration * 0.3 +
            normalized_stamina * 0.2 +
            normalized_agility * 0.1) * MAX_PLAYER_SPEED;

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