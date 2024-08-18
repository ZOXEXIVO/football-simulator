use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;
use crate::r#match::{
    CommonInjuredState, CommonReturningState,
    CommonRunningState, CommonShootingState, CommonTacklingState,
    CommonWalkingState, DefenderStrategies, ForwardStrategies, GameTickContext,
    GoalkeeperStrategies, MatchContext, MatchPlayer, MidfielderStrategies, PlayerTickContext,
};
use crate::PlayerFieldPositionGroup;
use nalgebra::Vector3;

pub type StateHandler = fn(
    in_state_time: u64,
    player: &mut MatchPlayer,
    context: &mut MatchContext,
    tick_context: &GameTickContext,
    player_context: PlayerTickContext,
    result: &mut Vec<PlayerUpdateEvent>,
) -> StateChangeResult;

pub trait StateStrategy {
    fn process(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult;
}

impl StateStrategy for PlayerFieldPositionGroup {
    fn process(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let state_handler: StateHandler = {
            match player.state {
                // Common states
                PlayerState::Walking => CommonWalkingState::process,
                PlayerState::Running => CommonRunningState::process,
                PlayerState::Tackling => CommonTacklingState::process,
                PlayerState::Shooting => CommonShootingState::process,
                PlayerState::Returning => CommonReturningState::process,
                PlayerState::Injured => CommonInjuredState::process,
                _ => {
                    match self {
                        // Specific states
                        PlayerFieldPositionGroup::Goalkeeper => GoalkeeperStrategies::process,
                        PlayerFieldPositionGroup::Defender => DefenderStrategies::process,
                        PlayerFieldPositionGroup::Midfielder => MidfielderStrategies::process,
                        PlayerFieldPositionGroup::Forward => ForwardStrategies::process,
                        _ => {
                            unimplemented!()
                        }
                    }
                }
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

pub struct StateChangeResult {
    pub state: Option<PlayerState>,
    pub velocity: Option<Vector3<f32>>,
}

impl StateChangeResult {
    pub fn with(state: PlayerState, velocity: Vector3<f32>) -> Self {
        StateChangeResult {
            state: Some(state),
            velocity: Some(velocity),
        }
    }

    pub fn none() -> Self {
        StateChangeResult {
            state: None,
            velocity: None,
        }
    }

    pub fn with_state(state: PlayerState) -> Self {
        StateChangeResult {
            state: Some(state),
            velocity: None,
        }
    }

    pub fn with_velocity(velocity: Vector3<f32>) -> Self {
        StateChangeResult {
            state: None,
            velocity: Some(velocity),
        }
    }
}
