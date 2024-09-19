use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::{ConditionContext, PlayerDistanceFromStartPosition, StateChangeResult, StateProcessingContext, StateProcessingHandler, MATCH_HALF_TIME_MS};
use crate::r#match::defenders::states::DefenderState;

static DEFENDER_TRACKING_BACK_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_tracking_back_data.json")));

#[derive(Default)]
pub struct DefenderTrackingBackState {}

impl StateProcessingHandler for DefenderTrackingBackState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult>
    {
        const CLOSE_TO_START_DISTANCE: f32 = 10.0;
        const BALL_INTERCEPTION_DISTANCE: f32 = 30.0;
        const MARKING_DISTANCE: f32 = 20.0;
        const CRITICAL_TIME_REMAINING: f32 = 300.0; // 5 minutes

        // Check if the defender has reached their starting position
        if ctx.player().distance_from_start_position() < CLOSE_TO_START_DISTANCE {
            return Some(StateChangeResult::with_defender_state(DefenderState::Standing));
        }

        // Check if the ball is close and moving towards the player
        if ctx.ball().distance() < BALL_INTERCEPTION_DISTANCE && ctx.ball().is_towards_player() {
            return Some(StateChangeResult::with_defender_state(DefenderState::Intercepting));
        }

        // Check if there's an opponent nearby to mark
        let (teammates_count, opponents_count) = ctx.player().distances();
        if opponents_count > 0 && ctx.ball().distance() < MARKING_DISTANCE {
            return Some(StateChangeResult::with_defender_state(DefenderState::Marking));
        }

        // If the team is losing and there's little time left, consider a more aggressive stance
        if ctx.player().is_team_loosing() && ctx.context.time.time > (MATCH_HALF_TIME_MS - 300) {
            return Some(StateChangeResult::with_defender_state(DefenderState::Pressing));
        }

        // If the player is tired, switch to a less demanding state
        if ctx.player().is_tired() {
            return Some(StateChangeResult::with_defender_state(DefenderState::HoldingLine));
        }

        // If the ball is on the team's own side, prioritize defensive positioning
        if ctx.ball().on_own_side() {
            match ctx.player().position_to_distance() {
                PlayerDistanceFromStartPosition::Big => None, // Continue tracking back
                PlayerDistanceFromStartPosition::Medium => Some(StateChangeResult::with_defender_state(DefenderState::HoldingLine)),
                PlayerDistanceFromStartPosition::Small => Some(StateChangeResult::with_defender_state(DefenderState::Standing)),
            }
        } else {
            None // Continue tracking back if the ball is on the opponent's side
        }
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }

    fn process_conditions(&self, ctx: ConditionContext) {

    }
}
