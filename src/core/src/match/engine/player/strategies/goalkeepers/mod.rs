mod decision;
pub mod states;

use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::goalkeepers::states::{
    GoalkeeperCatchingState, GoalkeeperComingOutState, GoalkeeperDistributingState,
    GoalkeeperDivingState, GoalkeeperHoldingState, GoalkeeperJumpingState, GoalkeeperKickingState,
    GoalkeeperPenaltyState, GoalkeeperPickingUpState, GoalkeeperPreSaveState,
    GoalkeeperPressureState, GoalkeeperPunchingState, GoalkeeperRestingState,
    GoalkeeperReturningGoalState, GoalkeeperStandingState, GoalkeeperSweepingState,
    GoalkeeperThrowingState,
};
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;
use crate::r#match::strategies::goalkeepers::states::{
    GoalkeeperShootingState, GoalkeeperTacklingState,
};
use crate::r#match::strategies::processing::StateChangeResult;
use crate::r#match::strategies::StateHandler;
use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext};

pub struct GoalkeeperStrategies {}

impl GoalkeeperStrategies {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let state_handler: StateHandler = match player.state {
            PlayerState::Goalkeeper(GoalkeeperState::Standing) => GoalkeeperStandingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::Resting) => GoalkeeperRestingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::Jumping) => GoalkeeperJumpingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::Diving) => GoalkeeperDivingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::Catching) => GoalkeeperCatchingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::Punching) => GoalkeeperPunchingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::Kicking) => GoalkeeperKickingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::HoldingBall) => {
                GoalkeeperHoldingState::process
            }
            PlayerState::Goalkeeper(GoalkeeperState::Throwing) => GoalkeeperThrowingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::PickingUpBall) => {
                GoalkeeperPickingUpState::process
            }
            PlayerState::Goalkeeper(GoalkeeperState::Distributing) => {
                GoalkeeperDistributingState::process
            }
            PlayerState::Goalkeeper(GoalkeeperState::ComingOut) => {
                GoalkeeperComingOutState::process
            }
            PlayerState::Goalkeeper(GoalkeeperState::ReturningToGoal) => {
                GoalkeeperReturningGoalState::process
            }
            PlayerState::Goalkeeper(GoalkeeperState::Tackling) => GoalkeeperTacklingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::Sweeping) => GoalkeeperSweepingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::UnderPressure) => {
                GoalkeeperPressureState::process
            }
            PlayerState::Goalkeeper(GoalkeeperState::Shooting) => GoalkeeperShootingState::process,
            PlayerState::Goalkeeper(GoalkeeperState::PreparingForSave) => {
                GoalkeeperPreSaveState::process
            }
            PlayerState::Goalkeeper(GoalkeeperState::PenaltySave) => {
                GoalkeeperPenaltyState::process
            }
            _ => {
                return StateChangeResult::none();
            }
        };

        state_handler(
            in_state_time,
            player,
            context,
            tick_context,
            player_context,
            result,
        )
    }
}
