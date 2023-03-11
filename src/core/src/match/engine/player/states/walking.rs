use crate::r#match::position::FieldPosition;
use crate::r#match::{
    MatchObjectsPositions, MatchPlayer, PlayerState, PlayerUpdateEvent, SteeringBehavior,
};
use nalgebra::Vector2;

pub struct WalkingState {}

impl WalkingState {
    pub fn process(
        player: &mut MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Option<PlayerState> {
        let direction = SteeringBehavior::Seek {
            target: FieldPosition::new(848.0, 275.0),
        }
        .calculate(player);

        player.velocity = Vector2::new(direction.velocity.x, direction.velocity.y);

        None
    }
}
