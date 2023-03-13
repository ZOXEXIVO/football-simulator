use crate::r#match::position::FieldPosition;
use crate::r#match::{
    MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent, SteeringBehavior,
};
use nalgebra::Vector2;

pub struct PassingState {}

impl PassingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        player.velocity = player.skills.running_speed();
        // if self.has_ball {
        //     // find closest teammate
        //     let closest_teammate = self.find_closest_teammate();
        //     // calculate pass vector
        //     let pass_vector = self.calculate_pass_vector(&closest_teammate);
        //     // pass the ball to the teammate
        //     self.pass_ball(pass_vector);
        //     // transition to standing state
        //     self.state = PlayerState::Standing;
        // }

        None
    }
}
