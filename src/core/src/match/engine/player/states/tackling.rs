use crate::r#match::{MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent};

pub struct TacklingState {}

impl TacklingState {
    pub fn process(
        _in_state_time: u64,
        player: &mut MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        _objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        player.velocity = player.skills.running_speed();
        // Check for transition to standing or walking state
        // let tackling_success = self.skills.tackling() * self.player_attributes.condition;
        // if tackling_success > 50.0 {
        //     self.has_ball = true;
        // }
        // // Check for transition to standing state
        // if self.player_attributes.condition < 20.0 {
        //     self.state = PlayerState::Standing;
        // }
        None
    }
}
