mod decision;
pub mod states;

use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::goalkeepers::states::{
    GoalkeeperCatchingState, GoalkeeperComingOutState, GoalkeeperDistributingState,
    GoalkeeperDivingState, GoalkeeperHoldingState, GoalkeeperJumpingState, GoalkeeperKickingState,
    GoalkeeperPenaltyState, GoalkeeperPickingUpState, GoalkeeperPreSaveState,
    GoalkeeperPressureState, GoalkeeperPunchingState, GoalkeeperRestingState,
    GoalkeeperReturningGoalState, GoalkeeperShootingState, GoalkeeperStandingState,
    GoalkeeperSweepingState, GoalkeeperTacklingState, GoalkeeperThrowingState,
    GoalkeeperWalkingState,
};
use crate::r#match::strategies::processing::StateChangeResult;
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateProcessor,
};

pub struct GoalkeeperStrategies {}

impl GoalkeeperStrategies {
    pub fn process(
        state: GoalkeeperState,
        state_processor: &mut StateProcessor,
    ) -> StateChangeResult {
        match state {
            GoalkeeperState::Standing => {
                state_processor.process(GoalkeeperStandingState::default())
            }
            GoalkeeperState::Resting => state_processor.process(GoalkeeperRestingState::default()),
            GoalkeeperState::Jumping => state_processor.process(GoalkeeperJumpingState::default()),
            GoalkeeperState::Diving => state_processor.process(GoalkeeperDivingState::default()),
            GoalkeeperState::Catching => {
                state_processor.process(GoalkeeperCatchingState::default())
            }
            GoalkeeperState::Punching => {
                state_processor.process(GoalkeeperPunchingState::default())
            }
            GoalkeeperState::Kicking => state_processor.process(GoalkeeperKickingState::default()),
            GoalkeeperState::HoldingBall => {
                state_processor.process(GoalkeeperHoldingState::default())
            }
            GoalkeeperState::Throwing => {
                state_processor.process(GoalkeeperThrowingState::default())
            }
            GoalkeeperState::PickingUpBall => {
                state_processor.process(GoalkeeperPickingUpState::default())
            }
            GoalkeeperState::Distributing => {
                state_processor.process(GoalkeeperDistributingState::default())
            }
            GoalkeeperState::ComingOut => {
                state_processor.process(GoalkeeperComingOutState::default())
            }
            GoalkeeperState::ReturningToGoal => {
                state_processor.process(GoalkeeperReturningGoalState::default())
            }
            GoalkeeperState::Tackling => {
                state_processor.process(GoalkeeperTacklingState::default())
            }
            GoalkeeperState::Sweeping => {
                state_processor.process(GoalkeeperSweepingState::default())
            }
            GoalkeeperState::UnderPressure => {
                state_processor.process(GoalkeeperPressureState::default())
            }
            GoalkeeperState::Shooting => {
                state_processor.process(GoalkeeperShootingState::default())
            }
            GoalkeeperState::PreparingForSave => {
                state_processor.process(GoalkeeperPreSaveState::default())
            }
            GoalkeeperState::PenaltySave => {
                state_processor.process(GoalkeeperPenaltyState::default())
            }
            GoalkeeperState::Walking => state_processor.process(GoalkeeperWalkingState::default()),
        }
    }
}
