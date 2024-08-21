use nalgebra::Vector3;
use crate::r#match::{Ball, MatchContext};

pub enum PlayerUpdateEvent {
    Goal(u32),
    Assist(u32),
    BallCollision(u32),
    TryAroundPlayer(u32, Vector3<f32>),
    TacklingBall(u32),
    PassTo(Vector3<f32>, f64),
    RushOut(u32),
    StayInGoal(u32),
    CommunicateMessage(u32, &'static str),
    Rest(u32)
}

pub struct PlayerEvents;

impl PlayerEvents {
    pub fn process<'p>(
        events: impl Iterator<Item = PlayerUpdateEvent>,
        ball: &mut Ball,
        context: &mut MatchContext,
    ) {
        for event in events {
            match event {
                PlayerUpdateEvent::Goal(player_id) => {
                    let player = context.players.get_mut(player_id).unwrap();

                    player.statistics.add_goal(context.time.time)
                },
                PlayerUpdateEvent::Assist(player_id) => {
                    let player = context.players.get_mut(player_id).unwrap();

                    player.statistics.add_assist(context.time.time)
                },
                PlayerUpdateEvent::BallCollision(player_id) => {
                    let player = context.players.get_mut(player_id).unwrap();

                    if player.skills.technical.first_touch > 10.0 {
                        player.has_ball = true;
                        ball.velocity = Vector3::<f32>::zeros();
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
                PlayerUpdateEvent::Rest(player_id) => {
                    let mut player = context.players.get_mut(player_id).unwrap();
                    player.player_attributes.condition += 10;
                }
            }
        }
    }
}
