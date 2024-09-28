use nalgebra::Vector3;
use crate::r#match::{Ball, BallPosition, MatchContext};

#[derive(Copy, Clone)]
pub enum BallUpdateEvent {
    Goal(GoalSide, Option<u32>),
    PlayerCollision(u32),
    UpdateVelocity(Vector3<f32>)
}

#[derive(Copy, Clone)]
pub enum GoalSide {
    Home,
    Away
}

pub struct BallEvents;

impl BallEvents {
    pub fn handle_events<'a>(
        _current_time: u64,
        ball: &mut Ball,
        events: impl Iterator<Item = &'a BallUpdateEvent>,
        context: &MatchContext,
    ) {
        for event in events {
            match *event {
                BallUpdateEvent::Goal(side, goalscorer_player_id) => {
                    match side {
                        GoalSide::Home => {
                            context.result.score.increment_home_goals()
                        }
                        GoalSide::Away => {
                            context.result.score.increment_away_goals()
                        }
                    }
                },
                BallUpdateEvent::PlayerCollision(player_id) => {

                },
                BallUpdateEvent::UpdateVelocity(new_ball_velocity) => {
                    ball.velocity = new_ball_velocity;
                }
            }
        }
    }

}