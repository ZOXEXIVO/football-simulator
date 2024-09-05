use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateChangeResult,
    StateProcessingHandler,
};
use std::sync::LazyLock;

static GOALKEEPER_DIVING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_diving_data.json")));

#[derive(Default)]
pub struct GoalkeeperDivingState {}

impl StateProcessingHandler for GoalkeeperDivingState {
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