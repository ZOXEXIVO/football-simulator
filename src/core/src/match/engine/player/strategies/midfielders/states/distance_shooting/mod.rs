use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateChangeResult, StateProcessingHandler};
use std::sync::LazyLock;

static MIDFIELDER_DISTANCE_SHOOTING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_distance_shooting_data.json")));

#[derive(Default)]
pub struct MidfielderDistanceShootingState {}

impl StateProcessingHandler for MidfielderDistanceShootingState {
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
