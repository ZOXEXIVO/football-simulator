use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::{ConditionContext, MatchPlayer, MatchPlayerLite, PlayerSide, StateChangeResult, StateProcessingContext, StateProcessingHandler};
use nalgebra::Vector3;
use std::sync::LazyLock;

static DEFENDER_OFFSIDE_TRAP_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_offside_trap_data.json")));

const OFFSIDE_TRAP_DISTANCE: f32 = 5.0; // Distance to move forward to set the trap
const OFFSIDE_TRAP_SPEED_MULTIPLIER: f32 = 1.2; // Speed multiplier when executing the trap
const OFFSIDE_TRAP_SUCCESS_THRESHOLD: f32 = 0.7; // Threshold for offside trap success
const STAMINA_THRESHOLD: f32 = 30.0; // Minimum stamina to execute the offside trap

#[derive(Default)]
pub struct DefenderOffsideTrapState {}

impl StateProcessingHandler for DefenderOffsideTrapState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // 1. Check defender's stamina
        let stamina = ctx.player.player_attributes.condition_percentage() as f32;
        if stamina < STAMINA_THRESHOLD {
            // Transition to Resting state if stamina is too low
            return Some(StateChangeResult::with_defender_state(
                DefenderState::Resting,
            ));
        }

        // 2. Determine if the offside trap should be executed
        if self.should_execute_offside_trap(ctx) {
            // 3. Execute the offside trap
            let trap_success = self.attempt_offside_trap(ctx);

            if trap_success {
                // Offside trap is successful
                // Transition to HoldingLine or appropriate state
                Some(StateChangeResult::with_defender_state(
                    DefenderState::HoldingLine,
                ))
            } else {
                // Offside trap failed; opponent may be through on goal
                // Transition to TrackingBack state to recover
                Some(StateChangeResult::with_defender_state(
                    DefenderState::TrackingBack,
                ))
            }
        } else {
            // Offside trap conditions not met; transition back to HoldingLine
            Some(StateChangeResult::with_defender_state(
                DefenderState::HoldingLine,
            ))
        }
    }

    fn process_slow(&self, _ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        // Implement neural network logic if necessary
        None
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        // Move forward quickly to execute the offside trap
        let direction = self.calculate_offside_trap_direction(ctx);
        let speed = ctx.player.skills.physical.pace * OFFSIDE_TRAP_SPEED_MULTIPLIER;
        Some(direction * speed)
    }

    fn process_conditions(&self, _ctx: ConditionContext) {
        // No additional conditions
    }
}

impl DefenderOffsideTrapState {
    /// Determines if the offside trap should be executed based on team tactics and opponent positions.
    fn should_execute_offside_trap(&self, _ctx: &StateProcessingContext) -> bool {
        // Check if team tactics allow for offside traps
        // if !ctx.context.team_tactics.allow_offside_trap {
        //     return false;
        // }

        // Ensure that all defenders are in sync (simplified for this example)
        // You might check if other defenders are also moving forward
        true
    }

    /// Attempts to execute the offside trap and returns whether it was successful.
    fn attempt_offside_trap(&self, ctx: &StateProcessingContext) -> bool {
        // Get the positions of opponents and the defensive line
        let defensive_line_position = self.calculate_defensive_line_position(ctx);
        let opponent_ahead = self.is_opponent_ahead(defensive_line_position, ctx);

        // Simulate chance of success based on teamwork and concentration
        let teamwork = ctx.player.skills.mental.teamwork as f32 / 100.0;
        let concentration = ctx.player.skills.mental.concentration as f32 / 100.0;
        let overall_skill = (teamwork + concentration) / 2.0;

        let success_chance = overall_skill;

        success_chance > OFFSIDE_TRAP_SUCCESS_THRESHOLD && opponent_ahead
    }

    /// Calculates the defensive line position based on the current positions of defenders.
    fn calculate_defensive_line_position(&self, ctx: &StateProcessingContext) -> f32 {
        let defenders: Vec<MatchPlayerLite> = ctx
            .players()
            .teammates()
            .defenders()
            .map(|p| p)
            .collect();

        // If no defenders found, use player's current position
        if defenders.is_empty() {
            return ctx.player.position.x;
        }

        // Calculate the highest x-position (furthest forward) among defenders
        if ctx.player.side.unwrap() == PlayerSide::Left {
            // Home team moving from left to right
            defenders
                .iter()
                .map(|p| p.position.x)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        } else {
            // Away team moving from right to left
            defenders
                .iter()
                .map(|p| p.position.x)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        }
    }

    /// Checks if any opponent is ahead of the defensive line.
    fn is_opponent_ahead(
        &self,
        defensive_line_position: f32,
        ctx: &StateProcessingContext,
    ) -> bool {
        ctx.context
            .players
            .raw_players()
            .iter()
            .filter(|p| p.team_id != ctx.player.team_id)
            .any(|opponent| {
                if ctx.player.side.unwrap() == PlayerSide::Left {
                    // Opponent is ahead if their x-position is greater than the defensive line
                    opponent.position.x > defensive_line_position
                } else {
                    // For away team, opponent is ahead if their x-position is less
                    opponent.position.x < defensive_line_position
                }
            })
    }

    /// Calculates the direction for the defender to move when executing the offside trap.
    fn calculate_offside_trap_direction(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        if ctx.player.side.unwrap() == PlayerSide::Left {
            // Home team moves forward in positive x-direction
            Vector3::new(1.0, 0.0, 0.0)
        } else {
            // Away team moves forward in negative x-direction
            Vector3::new(-1.0, 0.0, 0.0)
        }
    }
}
