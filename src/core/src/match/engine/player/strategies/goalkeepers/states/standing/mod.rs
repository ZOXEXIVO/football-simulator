use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::strategies::processing::StateChangeResult;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext};
use std::sync::LazyLock;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::state::PlayerState;

static GOALKEEPER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json"))
});

pub struct GoalkeeperStandingState {}

impl GoalkeeperStandingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if player_context.ball_context.on_own_side {
            if player_context.ball_context.is_heading_towards_player {
                return StateChangeResult::with_state(PlayerState::Goalkeeper(GoalkeeperState::PreparingForSave));
            }
        }

        if player.player_attributes.condition_percentage() > 60 {
            return StateChangeResult::with_state(PlayerState::Goalkeeper(GoalkeeperState::Resting));
        }

        StateChangeResult::none()
    }
}
