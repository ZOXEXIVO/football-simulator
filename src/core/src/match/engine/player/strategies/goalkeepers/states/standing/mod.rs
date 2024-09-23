use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{
    ConditionContext, StateChangeResult, StateProcessingContext, StateProcessingHandler,
};
use nalgebra::Vector3;
use std::sync::LazyLock;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;

static GOALKEEPER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json")));

const BALL_PROXIMITY_THRESHOLD: f32 = 20.0;
const DANGER_ZONE_RADIUS: f32 = 30.0;
const REACTION_TIME_THRESHOLD: u64 = 1000; // in milliseconds

#[derive(Default)]
pub struct GoalkeeperStandingState {}

impl StateProcessingHandler for GoalkeeperStandingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.ball().distance() < BALL_PROXIMITY_THRESHOLD {
            if ctx.ball().is_towards_player() {
                return Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::PreparingForSave));
            }
        }

        // 4. Check if an opponent is approaching the danger zone
        if self.is_opponent_in_danger_zone(ctx) {
            // 5. Transition to UnderPressure state
            return Some(StateChangeResult::with_goalkeeper_state(GoalkeeperState::UnderPressure));
        }

        // 6. Remain in Standing state
        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network processing if needed
        // For now, return None to indicate no state change
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Goalkeeper remains stationary in Standing state
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions to process in this state
    }
}

impl GoalkeeperStandingState {
    /// Determines if any opponent is within the danger zone around the penalty area.
    fn is_opponent_in_danger_zone(&self, ctx: &StateProcessingContext) -> bool {
        ctx.context.players.raw_players()
            .iter()
            .filter(|p| p.team_id != ctx.player.team_id)
            .any(|opponent| {
                let distance = (ctx.player.position - opponent.position).magnitude();
                distance < DANGER_ZONE_RADIUS
            })
    }

    /// Gets the center position of the goalkeeper's own goal.
    fn get_goal_center_position(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        let field_width = ctx.context.field_size.width as f32;
        let field_height = ctx.context.field_size.height as f32;

        if ctx.player().on_own_side() {
            // Home team's goal is on the left side
            Vector3::new(0.0, field_height / 2.0, 0.0)
        } else {
            // Away team's goal is on the right side
            Vector3::new(field_width, field_height / 2.0, 0.0)
        }
    }
}