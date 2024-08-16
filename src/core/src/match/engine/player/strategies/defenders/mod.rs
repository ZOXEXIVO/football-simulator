pub mod decision;
pub mod states;
use crate::common::NeuralNetwork;
use crate::r#match::strategies::defenders::states::{
    DefenderPassingState, DefenderReturningState, DefenderRunningState, DefenderShootingState,
    DefenderStandingState, DefenderTacklingState, DefenderWalkingState,
};
use crate::r#match::{
    BallState, GameState, GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer,
    PlayerState, PlayerTickContext, StateChangeResult,
};
use crate::r#match::player::events::PlayerUpdateEvent;

pub struct DefenderStrategies {}

impl DefenderStrategies {
    pub fn calculate(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        match player.state {
            PlayerState::Standing => DefenderStandingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Walking => DefenderWalkingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Running => DefenderRunningState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Tackling => DefenderTacklingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Shooting => DefenderShootingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Passing => DefenderPassingState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
            PlayerState::Returning => DefenderReturningState::process(
                player,
                context,
                tick_context,
                player_context,
                in_state_time,
                result,
            ),
        }
    }

    fn is_on_defending_half(player: &MatchPlayer, state: &GameState) -> bool {
        match state.ball_state {
            Some(ball_state) => ball_state == BallState::HomeSide && player.is_home,
            None => false,
        }
    }
}

enum DefenderBehavior {
    Defend,
    Support,
    Idle,
}