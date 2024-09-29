use std::fmt::{Display, Formatter};
use crate::r#match::player::events::{PlayerUpdateEvent, PlayerUpdateEventCollection};
use crate::r#match::{
    GameTickContext, MatchContext, MatchPlayer,
};
use crate::r#match::defenders::states::DefenderState;
use crate::r#match::forwarders::states::ForwardState;
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::midfielders::states::MidfielderState;

#[derive(Debug, Clone, Copy)]
pub enum PlayerState {
    Running,
    Tackling,
    Shooting,
    Returning,
    Injured,
    Goalkeeper(GoalkeeperState),
    Defender(DefenderState),
    Midfielder(MidfielderState),
    Forward(ForwardState)
}

impl Display for PlayerState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            PlayerState::Running => write!(f, "Running"),
            PlayerState::Tackling => write!(f, "Tackling"),
            PlayerState::Shooting => write!(f, "Shooting"),
            PlayerState::Returning => write!(f, "Returning"),
            PlayerState::Injured => write!(f, "Injured"),
            PlayerState::Goalkeeper(state) => write!(f, "Goalkeeper: {}", state),
            PlayerState::Defender(state) => write!(f, "Defender: {}", state),
            PlayerState::Midfielder(state) => write!(f, "Midfielder: {}", state),
            PlayerState::Forward(state) => write!(f, "Forward: {}", state)
        }
    }
}

pub struct PlayerMatchState;

impl PlayerMatchState {
    pub fn process(
        player: &mut MatchPlayer,
        context: &MatchContext,
        tick_context: &GameTickContext
    ) -> PlayerUpdateEventCollection {
        let mut state_change_result = player.tactics_position.position_group().process(
            player.in_state_time,
            player,
            context,
            tick_context
        );

        if let Some(state) = state_change_result.state {
            Self::change_state(player, state);
        } else {
            player.in_state_time += 1;
        }

        if let Some(velocity) = state_change_result.velocity {
            player.velocity = velocity;
        }

        state_change_result.events
    }

    fn change_state(player: &mut MatchPlayer, state: PlayerState) {
        player.in_state_time = 0;
        player.state = state;
    }
}
