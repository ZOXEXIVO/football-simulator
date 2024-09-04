use crate::common::NeuralNetwork;
use std::sync::LazyLock;

use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::{
    BallContext, GameFieldContextInput, GameTickContext, MatchContext, MatchObjectsPositions,
    MatchPlayer, PlayerTickContext, StateChangeResult, StateProcessingHandler, SteeringBehavior,
};

static GOALKEEPER_PRESAVE_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_presave_data.json")));

#[derive(Default)]
pub struct GoalkeeperPreSaveState {}

impl StateProcessingHandler for GoalkeeperPreSaveState {
    fn try_fast(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: &PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> Option<StateChangeResult> {
        None
    }

    fn process_slow(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: &PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        StateChangeResult::none()
    }
}
