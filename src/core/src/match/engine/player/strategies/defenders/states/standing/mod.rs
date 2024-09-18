use log::{debug, info};
use std::sync::LazyLock;

use nalgebra::Vector3;

use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::player::state::PlayerState;
use crate::r#match::{
    PlayerDistanceFromStartPosition, StateChangeResult, StateProcessingContext,
    StateProcessingHandler,
};

static DEFENDER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json")));

#[derive(Default)]
pub struct DefenderStandingState {}

impl StateProcessingHandler for DefenderStandingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.ball().on_own_side() {
            // OWN BALL SIDE
            if ctx.ball().is_towards_player() {
                if ctx.player().position_to_distance() == PlayerDistanceFromStartPosition::Big {
                    return Some(StateChangeResult::with_defender_state(
                        DefenderState::Returning,
                    ));
                }

                let (teammates_count, opponents_count) = ctx.player().distances();
                if opponents_count > 2 {
                    return Some(StateChangeResult::with(PlayerState::Defender(
                        DefenderState::Intercepting,
                    )));
                }

                if opponents_count > 2 && teammates_count < 1 {
                    return Some(StateChangeResult::with_defender_state(
                        DefenderState::Clearing,
                    ));
                }

                if ctx.ball().distance() < 100.0 {
                    if ctx.ball().speed() > 20.0 {
                        return Some(StateChangeResult::with_defender_state(
                            DefenderState::TrackingBack
                        ));
                    }

                    return Some(StateChangeResult::with_defender_state(DefenderState::Intercepting));
                }
            } else {
                // no towards player
            }
        } else {
            // BALL ON OTHER FIELD SIDE
            if ctx.player().is_tired() {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::Walking,
                ));
            }

            if ctx.in_state_time > 150 {
                return Some(StateChangeResult::with_defender_state(
                    DefenderState::Walking,
                ));
            }
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> StateChangeResult {
        let input = self.prepare_network_input(ctx);
        let output = DEFENDER_STANDING_STATE_NETWORK.run(&input);

        self.interpret_network_output(ctx, output)
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        Some(Vector3::new(0.0, 0.0, 0.0))
    }
}

impl DefenderStandingState {
    fn prepare_network_input(&self, ctx: &StateProcessingContext) -> Vec<f64> {
        vec![
            ctx.ball().distance() as f64,
            ctx.ball().speed() as f64,
            if ctx.ball().on_own_side() { 1.0 } else { 0.0 },
            if ctx.ball().is_towards_player() { 1.0 } else { 0.0 },
            ctx.player().distance_from_start_position() as f64,
            ctx.player.skills.physical.stamina as f64,
            ctx.player.skills.mental.positioning as f64,
            ctx.player.skills.mental.decisions as f64,
            ctx.context.time.time as f64,
            ctx.in_state_time as f64,
            ctx.player.skills.physical.acceleration as f64,
            ctx.player.skills.technical.tackling as f64,
            ctx.player.skills.technical.marking as f64
        ]
    }

    fn interpret_network_output(&self, ctx: &StateProcessingContext, output: Vec<f64>) -> StateChangeResult {
        let max_index = output.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index)
            .unwrap_or(0);

        let new_state = match max_index {
            0 => DefenderState::Standing, // No change
            1 => DefenderState::Returning,
            2 => DefenderState::Intercepting,
            3 => DefenderState::Clearing,
            4 => DefenderState::TrackingBack,
            5 => DefenderState::Walking,
            6 => DefenderState::Marking,
            7 => DefenderState::Pressing,
            8 => DefenderState::HoldingLine,
            _ => DefenderState::Standing,
        };

        StateChangeResult::with_defender_state(new_state)
    }
}