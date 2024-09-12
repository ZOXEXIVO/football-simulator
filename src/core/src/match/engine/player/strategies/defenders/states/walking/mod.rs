use std::sync::LazyLock;
use nalgebra::Vector3;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
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
            return Some(StateChangeResult::with_state(PlayerState::Defender(
                DefenderState::Intercepting,
            )));
        } else {
            return Some(StateChangeResult::with_state(PlayerState::Defender(
                DefenderState::Intercepting,
            )));
        }
    }

    fn process_slow(&self, ctx: &StateProcessingContext) -> StateChangeResult {
        StateChangeResult::none()
    }

    fn velocity(&self, ctx: &StateProcessingContext) -> Vector3<f32> {
        SteeringBehavior::Wander {
            target: ctx.player.start_position,
            radius: 0.5,
            jitter: 0.2,
            distance: 30.0,
            angle: 20.0,
        }.calculate(ctx.player).velocity
    }
}
