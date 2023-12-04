pub mod passing;
pub mod passing_decision;
pub mod returning;
pub mod running;
pub mod shooting;
pub mod standing;
pub mod tackling;
pub mod walking;

use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};
pub use passing::*;
pub use passing_decision::*;
pub use returning::*;
pub use running::*;
pub use shooting::*;
pub use standing::*;
pub use tackling::*;
pub use walking::*;

pub trait PlayerStateStrategy {
    fn process(
        &mut self,
        in_state_time: u64,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState>;
}

impl PlayerStateStrategy for MatchPlayer {
    fn process(
        &mut self,
        in_state_time: u64,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        match self.state {
            PlayerState::Standing => {
                StandingState::process(in_state_time, self, context, result, objects_positions)
            }
            PlayerState::Walking => {
                WalkingState::process(in_state_time, self, context, result, objects_positions)
            }
            PlayerState::Running => {
                RunningState::process(in_state_time, self, context, result, objects_positions)
            }
            PlayerState::Tackling => {
                TacklingState::process(in_state_time, self, context, result, objects_positions)
            }
            PlayerState::Shooting => {
                ShootingState::process(in_state_time, self, context, result, objects_positions)
            }
            PlayerState::Passing => {
                PassingState::process(in_state_time, self, context, result, objects_positions)
            }
            PlayerState::PassingDecision => PassingDecisionState::process(
                in_state_time,
                self,
                context,
                result,
                objects_positions,
            ),
            PlayerState::Returning => {
                ReturningState::process(in_state_time, self, context, result, objects_positions)
            }
        }
    }
}
