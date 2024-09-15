use nalgebra::Vector3;
use crate::r#match::{Ball, BallPosition, MatchContext};

#[derive(Copy, Clone)]
pub enum BallUpdateEvent {
    Goal(u32),
    ChangeBallSide(BallPosition),
    PlayerCollision(u32),
    UpdateVelocity(Vector3<f32>)
}

pub struct BallEvents;

impl BallEvents {
    pub fn handle_events<'a>(
        _current_time: u64,
        ball: &mut Ball,
        events: impl Iterator<Item = &'a BallUpdateEvent>,
        context: &mut MatchContext,
    ) {
        for event in events {
            match *event {
                BallUpdateEvent::Goal(team_id) => {
                    if context.result.score.home_team.team_id == team_id {
                        context.result.score.home_team.score += 1;
                    } else {
                        context.result.score.away_team.score += 1;
                    }
                }
                BallUpdateEvent::ChangeBallSide(_position) => {
                    // let ball_state = match position {
                    //     BallPosition::Home => BallState::HomeSide,
                    //     BallPosition::Away => BallState::AwaySide,
                    // };

                    //context.state.set_ball_state(ball_state)
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