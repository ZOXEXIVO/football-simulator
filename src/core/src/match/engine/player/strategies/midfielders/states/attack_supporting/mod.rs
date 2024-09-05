use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateChangeResult, StateProcessingHandler};
use std::sync::LazyLock;

static MIDFIELDER_ATTACK_SUPPORTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_attack_supporting_data.json")));

#[derive(Default)]
pub struct MidfielderAttackSupportingState {}

impl StateProcessingHandler for MidfielderAttackSupportingState {
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
