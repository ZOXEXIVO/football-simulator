use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use nalgebra::Vector3;
use std::sync::LazyLock;

use crate::r#match::defenders::states::DefenderState;
use crate::r#match::{
    ConditionContext, MatchPlayer, StateChangeResult, StateProcessingContext,
    StateProcessingHandler,
};

static DEFENDER_HOLDING_LINE_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_holding_line_data.json")));

const MAX_DEFENSIVE_LINE_DEVIATION: f32 = 50.0;
const BALL_PROXIMITY_THRESHOLD: f32 = 10.0;
const MARKING_DISTANCE_THRESHOLD: f32 = 5.0;

#[derive(Default)]
pub struct DefenderHoldingLineState {}

impl StateProcessingHandler for DefenderHoldingLineState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. Calculate the defensive line position
        let defensive_line_position = self.calculate_defensive_line_position(ctx);

        // 2. Calculate the distance from the defender to the defensive line
        let distance_from_line = (ctx.player.position.y - defensive_line_position).abs();

        // 3. If the defender is too far from the defensive line, switch to Running state
        if distance_from_line > MAX_DEFENSIVE_LINE_DEVIATION {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Running,
            ));
        }

        // 4. Calculate the distance to the ball
        let ball_distance =
            (ctx.tick_context.object_positions.ball_position - ctx.player.position).magnitude();

        // 5. If the ball is close, decide whether to Mark or Intercept
        if ball_distance < BALL_PROXIMITY_THRESHOLD {
            let opponent_nearby = self.is_opponent_nearby(ctx);
            return Some(StateChangeResult::with_defender_state(if opponent_nearby {
                DefenderState::Marking
            } else {
                DefenderState::Intercepting
            }));
        }

        // 6. Check if we should set up an offside trap
        if self.should_set_offside_trap(ctx) {
            return Some(StateChangeResult::with_defender_state(
                DefenderState::OffsideTrap,
            ));
        }

        // 7. Remain in HoldingLine state
        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network processing if needed
        // For now, return None to indicate no state change
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::zeros())
    }

    fn process_conditions(&self, ctx: ConditionContext) {}
}

impl DefenderHoldingLineState {
    /// Calculates the defensive line position based on team tactics and defender positions.
    fn calculate_defensive_line_position(&self, ctx: &StateProcessingContext) -> f32 {
        let players = ctx.team();
        let defenders: Vec<&MatchPlayer> = players.defenders();

        // If no defenders found, use player's current position
        if defenders.is_empty() {
            return ctx.player.position.y;
        }

        // Calculate the average y-position of defenders to determine the defensive line
        let sum_y_positions: f32 = defenders.iter().map(|p| p.position.y).sum();
        sum_y_positions / defenders.len() as f32
    }

    /// Checks if an opponent player is nearby within the MARKING_DISTANCE_THRESHOLD.
    fn is_opponent_nearby(&self, ctx: &StateProcessingContext) -> bool {
        ctx.team().opponents().iter().any(|opponent| {
            let distance = (ctx.player.position - opponent.position).magnitude();
            distance < MARKING_DISTANCE_THRESHOLD
        })
    }

    /// Determines if the team should set up an offside trap.
    fn should_set_offside_trap(&self, ctx: &StateProcessingContext) -> bool {
        // Check if opponents are positioned ahead of the defensive line
        let defensive_line_position = self.calculate_defensive_line_position(ctx);

        let opponents_ahead = ctx
            .team()
            .opponents()
            .iter()
            .filter(|opponent| {
                if ctx.player().on_own_side() {
                    opponent.position.y < defensive_line_position
                } else {
                    opponent.position.y > defensive_line_position
                }
            })
            .count();

        // If multiple opponents are ahead, consider setting up an offside trap
        opponents_ahead >= 2
    }
}
