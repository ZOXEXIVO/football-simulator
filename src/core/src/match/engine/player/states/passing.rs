use crate::r#match::{
    MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent,
};

pub struct PassingState {}

impl PassingState {
    pub fn process(
        _in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        if let Some(teammate_position) =
            objects_positions.find_closest_teammate(player, &context.state.match_state)
        {
            result.push(PlayerUpdateEvent::PassTo(
                teammate_position,
                player.skills.running_speed(),
            ))
        }

        Some(PlayerState::Standing)
    }
}
