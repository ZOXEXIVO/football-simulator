use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::IntegerUtils;
use crate::r#match::{StateChangeResult, StateProcessingContext, StateProcessingHandler, SteeringBehavior};
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::player::state::PlayerState;

static DEFENDER_WALKING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_walking_data.json")));

#[derive(Default)]
pub struct DefenderWalkingState {}

impl StateProcessingHandler for DefenderWalkingState {
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult> {
        if ctx.player().on_own_side() {
            if ctx.ball().distance() < 100.0 {
                return Some(StateChangeResult::with(PlayerState::Defender(
                    DefenderState::Intercepting,
                )))
            }
        }

        if ctx.player().distance_from_start_position() > 50.0 {
            return Some(StateChangeResult::with(PlayerState::Defender(
                DefenderState::Returning,
            )))
        }

        None
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> StateChangeResult {
        StateChangeResult::none()
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>> {
        if ctx.in_state_time == 0 {
            let rnd =
            return Some(SteeringBehavior::Wander {
                target: ctx.player.start_position,
                radius: IntegerUtils::random(1, 30) as f32,
                jitter: IntegerUtils::random(1, 50) as f32,
                distance: IntegerUtils::random(1, 100) as f32,
                angle: IntegerUtils::random(1, 10) as f32,
            }.calculate(ctx.player).velocity);
        }

        None
    }
}
