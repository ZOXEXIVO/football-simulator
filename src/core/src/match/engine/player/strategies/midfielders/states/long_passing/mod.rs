use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, MatchPlayerLite, StateChangeResult, StateProcessingContext, StateProcessingHandler};
use nalgebra::Vector3;
use std::sync::LazyLock;
use crate::r#match::events::Event;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::player::events::PlayerEvent;

static MIDFIELDER_LONG_PASSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_long_passing_data.json")));

// Constants used in passing calculations
const MAX_PASS_DISTANCE: f32 = 300.0; // Maximum distance for a short pass
const MIN_PASS_SPEED: f32 = 10.0; // Minimum speed of the pass
const MAX_PASS_SPEED: f32 = 15.0; // Maximum speed of the pass
const STAMINA_COST_PASS: f32 = 2.0; // Stamina cost of making a pass
const OPPONENT_COLLISION_RADIUS: f32 = 0.5; // Radius representing opponent's collision area


#[derive(Default)]
pub struct MidfielderLongPassingState {}

impl StateProcessingHandler for MidfielderLongPassingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Check if the midfielder still has the ball
        if !ctx.player.has_ball(ctx) {
            // Lost possession, transition to Pressing
            return Some(StateChangeResult::with_midfielder_state(
                MidfielderState::Pressing,
            ));
        }

        // Determine the best teammate to pass to
        if let Some(target_teammate) = self.find_best_teammate(ctx) {
            Some(StateChangeResult::with_midfielder_state_and_event(
                MidfielderState::Standing,
                Event::PlayerEvent(PlayerEvent::PassTo(
                    target_teammate.id,
                    target_teammate.position,
                    1.0,
                )
                )))
        } else {
            // No available teammate found, consider other options
            Some(StateChangeResult::with_midfielder_state(
                MidfielderState::HoldingPossession,
            ))
        }
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, _ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {}
}

impl MidfielderLongPassingState {
    fn find_best_teammate(&self, ctx: &StateProcessingContext<'_>) -> Option<MatchPlayerLite> {
        let max_pass_distance = MAX_PASS_DISTANCE;

        let players = ctx.players();
        let teammates = players.teammates();

        for teammate in teammates.nearby(max_pass_distance) {
            if !teammate.has_ball(ctx) {
                continue;
            }

            if !self.is_pass_feasible_ray_tracing(ctx, &teammate) {
                continue;
            }

            return Some(teammate);
        }

        None
    }

    /// Checks if the pass to the target teammate is feasible using ray tracing.
    fn is_pass_feasible_ray_tracing(
        &self,
        ctx: &StateProcessingContext,
        target_teammate: &MatchPlayerLite,
    ) -> bool {
        let player_position = ctx.player.position;
        let target_position = target_teammate.position;

        // Direction vector from player to target teammate
        let direction = (target_position - player_position).normalize();

        // Distance to the target teammate
        let distance_to_target = (target_position - player_position).magnitude();

        // Ray parameters
        let ray_origin = player_position;
        let ray_direction = direction;

        // Iterate over opponents to check for intersections
        for opponent in ctx.context.players.raw_players().iter() {
            if opponent.team_id != ctx.player.team_id {
                let opponent_position = opponent.position;

                // Check if opponent is within the pass corridor
                if self.ray_intersects_sphere(
                    ray_origin,
                    ray_direction,
                    opponent_position,
                    OPPONENT_COLLISION_RADIUS,
                    distance_to_target,
                ) {
                    // Opponent is obstructing the pass
                    return false;
                }
            }
        }
        true
    }

    /// Checks if a ray intersects with a sphere (opponent).
    fn ray_intersects_sphere(
        &self,
        ray_origin: Vector3<f32>,
        ray_direction: Vector3<f32>,
        sphere_center: Vector3<f32>,
        sphere_radius: f32,
        max_distance: f32,
    ) -> bool {
        let m = ray_origin - sphere_center;
        let b = m.dot(&ray_direction);
        let c = m.dot(&m) - sphere_radius * sphere_radius;

        // Exit if the ray's origin is outside the sphere (c > 0) and pointing away from the sphere (b > 0)
        if c > 0.0 && b > 0.0 {
            return false;
        }

        let discriminant = b * b - c;

        // A negative discriminant indicates no intersection
        if discriminant < 0.0 {
            return false;
        }

        // Compute the distance to the intersection point
        let t = -b - discriminant.sqrt();

        // If t is negative, the intersection is behind the ray's origin
        if t < 0.0 {
            return false;
        }

        // Check if the intersection point is within the maximum distance
        t <= max_distance
    }
}