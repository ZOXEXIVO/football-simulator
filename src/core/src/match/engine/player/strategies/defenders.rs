use crate::r#match::{BallState, MatchPlayer, MatchState, PlayerUpdateEvent};
use nalgebra::Vector2;

pub struct DefenderStrategies {}

impl DefenderStrategies {
    pub fn move_to(
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        state: &MatchState,
    ) -> Vector2<f32> {
        Vector2::new(0.0, 0.0)
        // match state.ball_state {
        //     Some(ball_state) => if ball_state == BallState::HomeSide {},
        //     None => Vector2::new(0.0, 0.0),
        // }
    }
}
