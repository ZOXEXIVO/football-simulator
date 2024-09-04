use std::sync::LazyLock;
use crate::common::NeuralNetwork;

use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateChangeResult, StateProcessingHandler};
use crate::r#match::player::events::PlayerUpdateEvent;

static DEFENDER_HOLDING_LINE_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_holding_line_data.json"))
});

#[derive(Default)]
pub struct DefenderHoldingLineState {}

impl StateProcessingHandler for DefenderHoldingLineState {
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



