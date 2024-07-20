use crate::r#match::{GameTickContext, MatchContext, MatchPlayer, PlayerTickContext, StateStrategy};
use crate::r#match::player::events::PlayerUpdateEvent;

pub struct PlayerMatchState;

impl PlayerMatchState {
    pub fn process(
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) {
        let state_result = player.tactics_position.position_group().calculate(
            player.in_state_time,
            player,
            context,
            tick_context,
            player_context,
            result,
        );

        if let Some(state) = state_result.state {
            Self::change_state(player, state);
        } else {
            player.in_state_time += 1;
        }

        if let Some(velocity) = state_result.velocity {
            player.velocity = velocity;
        }
    }

    fn change_state(player: &mut MatchPlayer, state: crate::r#match::PlayerState) {
        player.in_state_time = 0;
        player.state = state;
    }
}