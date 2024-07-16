use nalgebra::Vector3;
use crate::r#match::{Ball, MatchContext, PlayerUpdateEvent};

pub struct PlayerEvents;

impl PlayerEvents {
    pub fn process<'p>(
        events: impl Iterator<Item = &'p PlayerUpdateEvent>,
        ball: &mut Ball,
        context: &mut MatchContext,
    ) {
        for event in events {
            match event {
                PlayerUpdateEvent::Goal(_player_id) => {},
                PlayerUpdateEvent::BallCollision(player_id) => {
                    let player = context.players.get_mut(*player_id).unwrap();

                    if player.skills.technical.first_touch > 10.0 {
                        player.has_ball = true;
                    }
                },
                PlayerUpdateEvent::TacklingBall(_player_id) => {
                    ball.velocity = Vector3::<f32>::zeros();
                }
                PlayerUpdateEvent::PassTo(pass_target, pass_power) => {
                    let ball_pass_vector = pass_target - ball.position;
                    ball.velocity = ball_pass_vector.normalize();
                }
                PlayerUpdateEvent::RushOut(_) => {}
                PlayerUpdateEvent::StayInGoal(_) => {},
                PlayerUpdateEvent::TryAroundPlayer(player_id, player_position) => {

                },
                PlayerUpdateEvent::CommunicateMessage(player_id, message) => {}
            }
        }
    }
}
