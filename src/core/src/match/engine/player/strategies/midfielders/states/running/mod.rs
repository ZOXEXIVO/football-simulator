use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::events::Event;
use crate::r#match::midfielders::states::MidfielderState;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext,
    StateProcessingHandler, SteeringBehavior, VectorExtensions,
};
use crate::IntegerUtils;
use nalgebra::Vector3;
use std::sync::LazyLock;

static MIDFIELDER_RUNNING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_running_data.json")));

const MAX_SHOOTING_DISTANCE: f32 = 300.0; // Maximum distance to attempt a shot
const MIN_SHOOTING_DISTANCE: f32 = 20.0; // Minimum distance to attempt a shot (e.g., edge of penalty area)

#[derive(Default)]
pub struct MidfielderRunningState {}

impl StateProcessingHandler for MidfielderRunningState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.player.has_ball {
            // If the player has the ball, consider shooting, passing, or dribbling
            if self.is_in_shooting_range(ctx) {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::DistanceShooting,
                ));
            }

            if let Some(teammate_id) = self.find_open_teammate(ctx) {
                return Some(StateChangeResult::with_midfielder_state_and_event(
                    MidfielderState::ShortPassing,
                    Event::PlayerEvent(PlayerEvent::RequestPass(teammate_id)),
                ));
            }

            // If no shooting or passing options, consider dribbling
            if self.should_dribble(ctx) {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Dribbling,
                ));
            }
        } else {
            // If the player doesn't have the ball, check if they should press, support attack, or return
            if self.should_press(ctx) {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Pressing,
                ));
            }

            if self.should_support_attack(ctx) {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::SupportingAttack,
                ));
            }

            if self.should_return_to_position(ctx) {
                return Some(StateChangeResult::with_midfielder_state(
                    MidfielderState::Returning,
                ));
            }
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Check if there's space to run between opponents
        if let Some(target_position) = self.find_space_between_opponents(ctx) {
            Some(
                SteeringBehavior::Arrive {
                    target: target_position,
                    slowing_distance: 10.0,
                }
                .calculate(ctx.player)
                .velocity,
            )
        } else if ctx.team().is_control_ball() {
            Some(
                SteeringBehavior::Arrive {
                    target: ctx.ball().direction_to_opponent_goal(),
                    slowing_distance: 200.0,
                }
                .calculate(ctx.player)
                .velocity,
            )
        } else {
            Some(
                SteeringBehavior::Wander {
                    target: ctx.player.start_position,
                    radius: IntegerUtils::random(5, 150) as f32,
                    jitter: IntegerUtils::random(0, 2) as f32,
                    distance: IntegerUtils::random(10, 150) as f32,
                    angle: IntegerUtils::random(0, 360) as f32,
                }
                .calculate(ctx.player)
                .velocity,
            )
        }
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}

impl MidfielderRunningState {
    fn is_in_shooting_range(&self, ctx: &StateProcessingContext) -> bool {
        let distance_to_goal = ctx.ball().distance_to_opponent_goal();
        distance_to_goal <= MAX_SHOOTING_DISTANCE && distance_to_goal >= MIN_SHOOTING_DISTANCE
    }

    fn find_open_teammate<'a>(&self, ctx: &StateProcessingContext<'a>) -> Option<u32> {
        // Find an open teammate to pass to
        let teammates = ctx.context.players.get_by_team(ctx.player.team_id);
        let open_teammates = teammates
            .iter()
            .filter(|teammate| {
                // Check if the teammate is open (not closely marked by an opponent)
                let opponent_distance = ctx
                    .tick_context
                    .object_positions
                    .player_distances
                    .find_closest_opponent(teammate)
                    .map(|(_, distance)| distance)
                    .unwrap_or(f32::MAX);

                opponent_distance > 5.0 // Adjust the threshold as needed
            })
            .min_by(|a, b| {
                // Prefer teammates closer to the opponent's goal
                let a_distance = (a.position - ctx.ball().direction_to_opponent_goal()).magnitude();
                let b_distance = (b.position - ctx.ball().direction_to_opponent_goal()).magnitude();
                a_distance.partial_cmp(&b_distance).unwrap()
            })
            .map(|p| p.id);

        open_teammates
    }

    fn should_press(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the player should press the opponent with the ball
        let ball_distance = ctx.ball().distance();
        let pressing_distance = 150.0; // Adjust the threshold as needed

        !ctx.team().is_control_ball() && ball_distance < pressing_distance
    }

    fn find_space_between_opponents(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        let nearest_opponents = ctx
            .tick_context
            .object_positions
            .player_distances
            .find_closest_opponents(ctx.player);

        if let Some(opponents) = nearest_opponents {
            if opponents.len() >= 2 {
                let opponent1_position = ctx.context.players.get(opponents[0].0).unwrap().position;
                let opponent2_position = ctx.context.players.get(opponents[1].0).unwrap().position;

                let midpoint = (opponent1_position + opponent2_position) * 0.5;
                let distance_between_opponents =
                    opponent1_position.distance_to(&opponent2_position);

                if distance_between_opponents > 10.0 {
                    return Some(midpoint);
                }
            }
        }

        None
    }

    fn should_dribble(&self, ctx: &StateProcessingContext) -> bool {
        // Check if there is space to dribble and no immediate pressure from opponents
        let space_ahead = self.space_ahead(ctx);
        let under_pressure = self.is_under_pressure(ctx);

        space_ahead && !under_pressure
    }

    fn should_support_attack(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the team is in possession and the player is in a good position to support the attack
        let team_in_possession = ctx.team().is_control_ball();
        let in_attacking_half = ctx.player.position.x > ctx.context.field_size.width as f32 / 2.0;

        team_in_possession && in_attacking_half
    }

    fn should_return_to_position(&self, ctx: &StateProcessingContext) -> bool {
        // Check if the player is far from their starting position and the team is not in possession
        let distance_from_start = ctx.player().distance_from_start_position();
        let team_in_possession = ctx.team().is_control_ball();

        distance_from_start > 20.0 && !team_in_possession
    }

    fn space_ahead(&self, ctx: &StateProcessingContext) -> bool {
        // Check if there is open space ahead of the player
        let space_threshold = 10.0;
        let player_direction = ctx.player.velocity.normalize();
        let space_ahead = ctx.tick_context.space.cast_ray(
            ctx.player.position,
            player_direction,
            space_threshold,
            true,
        );

        space_ahead.is_none()
    }

    fn is_under_pressure(&self, ctx: &StateProcessingContext) -> bool {
        // Check if there are opponents close to the player
        let pressure_distance = 5.0;
        let close_opponents = ctx
            .tick_context
            .object_positions
            .player_distances
            .find_closest_opponents(ctx.player)
            .map(|opponents| {
                opponents
                    .iter()
                    .filter(|(_, dist)| *dist < pressure_distance)
                    .count()
            })
            .unwrap_or(0);

        close_opponents > 0
    }
}
