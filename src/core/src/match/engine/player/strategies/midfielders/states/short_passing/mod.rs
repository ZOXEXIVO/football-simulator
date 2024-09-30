use std::sync::LazyLock;
use nalgebra::{Vector3};
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext, StateProcessingHandler};
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::player::events::PlayerUpdateEvent;

static MIDFIELDER_SHORT_PASSING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_short_passing_data.json")));

#[derive(Default)]
pub struct MidfielderShortPassingState {}

impl StateProcessingHandler for MidfielderShortPassingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Check if the midfielder still has the ball
        if !ctx.player.has_ball {
            // Lost possession, transition to Pressing
            return Some(StateChangeResult::with_midfielder_state(MidfielderState::Pressing));
        }

        // Determine the best teammate to pass to
        if let Some(target_teammate) = self.find_best_teammate(ctx) {
            // Calculate pass velocity
            let pass_velocity = self.calculate_pass_velocity(ctx, &target_teammate);

            // Create an event to change the ball's velocity and update possession
            let mut state_change = StateChangeResult::with_midfielder_state(MidfielderState::Standing);

            state_change.events.add(PlayerUpdateEvent::MoveBall(ctx.player.id, pass_velocity));

            // Transition to the next appropriate state (e.g., Standing)
            Some(state_change)
        } else {
            // No available teammate found, consider other options
            Some(StateChangeResult::with_midfielder_state(MidfielderState::HoldingPossession))
        }
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network logic if necessary
        // For more complex decision-making, you can use the neural network here
        None
    }

    fn velocity(&self, _ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Midfielder remains stationary while making the pass
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions
    }
}

impl MidfielderShortPassingState {
    /// Finds the best teammate to pass to based on proximity and position.
    fn find_best_teammate<'a>(&self, ctx: &StateProcessingContext<'a>) -> Option<&'a MatchPlayer> {
        let max_pass_distance = MAX_PASS_DISTANCE;
        let player_position = ctx.player.position;

        if let Some(distances)  = ctx.tick_context.object_positions.player_distances.find_closest_teammates(ctx.player) {
            for (teammate_id, distance) in distances {
                let player = ctx.context.players.get(teammate_id)?;

                if !player.has_ball {
                    continue;
                }

                if !self.is_pass_feasible_ray_tracing(ctx, player){
                    continue;
                }

                if distance < max_pass_distance {
                    return Some(player);
                }
            }
        }

        None
    }

    /// Calculates the pass velocity vector towards the target teammate.
    fn calculate_pass_velocity(&self, ctx: &StateProcessingContext, target_teammate: &MatchPlayer) -> Vector3<f32> {
        let player_position = ctx.player.position;
        let target_position = target_teammate.position;

        // Calculate the direction to the target
        let direction = (target_position - player_position).normalize();

        // Calculate pass speed based on player's passing skill
        let passing_skill = ctx.player.skills.technical.passing as f32 / 100.0;
        let pass_speed = MIN_PASS_SPEED + (MAX_PASS_SPEED - MIN_PASS_SPEED) * passing_skill;

        direction * pass_speed
    }

    /// Checks if the pass to the target teammate is feasible using ray tracing.
    fn is_pass_feasible_ray_tracing(&self, ctx: &StateProcessingContext, target_teammate: &MatchPlayer) -> bool {
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

// Constants used in passing calculations
const MAX_PASS_DISTANCE: f32 = 20.0; // Maximum distance for a short pass
const MIN_PASS_SPEED: f32 = 5.0;     // Minimum speed of the pass
const MAX_PASS_SPEED: f32 = 15.0;    // Maximum speed of the pass
const STAMINA_COST_PASS: f32 = 2.0;  // Stamina cost of making a pass
const OPPONENT_COLLISION_RADIUS: f32 = 0.5; // Radius representing opponent's collision area
