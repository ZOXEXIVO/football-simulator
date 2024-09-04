use std::cmp::PartialEq;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::common::NeuralNetwork;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::strategies::processing::StateChangeResult;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerDistanceFromStartPosition, PlayerTickContext, StateProcessingHandler};
use std::sync::LazyLock;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::state::PlayerState;

static GOALKEEPER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> = LazyLock::new(|| {
    DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json"))
});

#[derive(Default)]
pub struct GoalkeeperStandingState {}

impl StateProcessingHandler for GoalkeeperStandingState {
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

impl GoalkeeperStandingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        if in_state_time > 100 {
            return StateChangeResult::with_state(PlayerState::Goalkeeper(GoalkeeperState::ComingOut));
        }

        if !player_context.ball_context.on_own_side {
           return StateChangeResult::none();
        }

        if player_context.ball_context.is_heading_towards_player && player_context.ball_context.ball_distance < 100.0{
            return StateChangeResult::with_state(PlayerState::Goalkeeper(GoalkeeperState::PreparingForSave));
        }

        if player.player_attributes.condition_percentage() < 60 {
            return StateChangeResult::with_state(PlayerState::Goalkeeper(GoalkeeperState::Resting));
        }

        match player_context.player_context.distance_to_start_position {
            PlayerDistanceFromStartPosition::Medium | PlayerDistanceFromStartPosition::Big => {
                return StateChangeResult::with_state(PlayerState::Goalkeeper(GoalkeeperState::ReturningToGoal));
            }
            _ => {}
        }

        StateChangeResult::none()
    }
}
