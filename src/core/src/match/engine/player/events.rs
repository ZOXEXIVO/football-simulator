use nalgebra::Vector3;
use crate::r#match::{Ball, MatchContext, PlayerUpdateEvent};

pub struct PlayerEvents;

impl PlayerEvents {
    pub fn process(
        events: Vec<PlayerUpdateEvent>,
        ball: &mut Ball,
        _context: &mut MatchContext,
    ) {
        for event in events {
            match event {
                PlayerUpdateEvent::Goal(_player_id) => {}
                PlayerUpdateEvent::TacklingBall(_player_id) => {
                    ball.velocity = Vector3::<f32>::zeros();
                }
                PlayerUpdateEvent::PassTo(pass_target, pass_power) => {
                    let ball_pass_vector = pass_target - ball.position;
                    ball.velocity = ball_pass_vector.normalize();
                }
                PlayerUpdateEvent::RushOut(_) => {}
                PlayerUpdateEvent::StayInGoal(_) => {}
                _ => {}
            }
        }
    }
}
