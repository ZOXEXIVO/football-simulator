use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext, StateProcessingHandler};
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::player::PlayerSide;

static FORWARD_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json")));

#[derive(Default)]
pub struct ForwardStandingState {}

impl StateProcessingHandler for ForwardStandingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Check if the forward still has the ball
        if ctx.player.has_ball {
            // Decide next action based on game context
            if self.is_in_shooting_range(ctx) {
                // Transition to Shooting state
                return Some(StateChangeResult::with_forward_state(ForwardState::Shooting));
            }

            if let Some(target_teammate) = self.find_best_teammate_to_pass(ctx) {
                // Transition to Passing state
                return Some(StateChangeResult::with_forward_state(ForwardState::Passing));
            }

            // If unable to shoot or pass, decide to dribble or hold position
            if self.should_dribble(ctx) {
                Some(StateChangeResult::with_forward_state(ForwardState::Dribbling))
            } else {
                None
                // Hold possession
                //return Some(StateChangeResult::with_forward_state(ForwardState::HoldingPossession));
            }
        } else {
            // If the forward doesn't have the ball, decide to move or press
            if self.should_press(ctx) {
                // Transition to Pressing state
                Some(StateChangeResult::with_forward_state(ForwardState::Pressing))
            } else {
                Some(StateChangeResult::with_forward_state(ForwardState::Running))
                // Transition to Positioning state
                //Some(StateChangeResult::with_forward_state(ForwardState::Positioning))
            }
        }
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network logic for advanced decision-making if necessary
        // For example, adjust positioning based on opponent movement
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, ctx: ConditionContext) {
        // Handle additional conditions or triggers if necessary
    }
}

impl ForwardStandingState {
    /// Determines if the forward is within shooting range of the opponent's goal.
    fn is_in_shooting_range(&self, ctx: &StateProcessingContext) -> bool {
        let distance_to_goal = self.distance_to_opponent_goal(ctx);
        distance_to_goal <= MAX_SHOOTING_DISTANCE && distance_to_goal >= MIN_SHOOTING_DISTANCE
    }

    /// Finds the best teammate to pass to based on proximity and position.
    fn find_best_teammate_to_pass<'a>(&self, ctx: &StateProcessingContext<'a>) -> Option<&'a MatchPlayer> {
        // Utilize the find_closest_teammate method from PlayerDistanceClosure
        let closest_teammates = ctx.tick_context
            .object_positions
            .player_distances
            .find_closest_teammates(&ctx.player);

        if let Some(closest_teammates) = closest_teammates {
            if let Some((teammate_id, distance)) = closest_teammates.first() {
                return Some(ctx.context.players.get(*teammate_id)?);
            }
        }

        None
    }

    /// Decides whether the forward should dribble based on game context.
    fn should_dribble(&self, ctx: &StateProcessingContext) -> bool {
        // Example logic: dribble if no immediate threat and space is available
        let safe_distance = 10.0;

        let closest_opponent = ctx.tick_context.object_positions.player_distances
            .find_closest_opponent(ctx.player);

        if let Some((_, distance)) = closest_opponent {
            distance > safe_distance
        } else {
            true
        }
    }

    /// Decides whether the forward should press the opponent.
    fn should_press(&self, ctx: &StateProcessingContext) -> bool {
        ctx.ball().distance() < PRESS_DISTANCE  && ctx.player.has_ball
    }

    /// Calculates the optimal attacking position for the forward.
    fn calculate_optimal_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        // Example logic: position towards the opponent's goal center
        let goal_position = self.get_opponent_goal_position(ctx);
        goal_position
    }

    /// Gets the position of the opponent's goal.
    fn get_opponent_goal_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let field_length = ctx.context.field_size.width as f32;
        let field_width = ctx.context.field_size.width as f32;

        if ctx.player.side.unwrap() == PlayerSide::Left {
            // Attacking towards the right (positive x)
            Vector3::new(field_length, field_width / 2.0, 0.0)
        } else {
            // Attacking towards the left (negative x)
            Vector3::new(0.0, field_width / 2.0, 0.0)
        }
    }

    /// Calculates the distance from the forward to the opponent's goal.
    fn distance_to_opponent_goal(&self, ctx: &StateProcessingContext) -> f32 {
        ctx.ball().distance_to_opponent_goal()
    }
}

// Constants used in ForwardStandingState
const MAX_SHOOTING_DISTANCE: f32 = 30.0; // Maximum distance to attempt a shot
const MIN_SHOOTING_DISTANCE: f32 = 16.5; // Minimum distance to attempt a shot (e.g., edge of penalty area)
const PRESS_DISTANCE: f32 = 20.0;        // Distance within which to press opponents
