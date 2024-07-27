use nalgebra::Vector3;
use crate::r#match::{Ball, BallPosition, MatchContext};

#[derive(Copy, Clone)]
pub enum BallUpdateEvent {
    HomeGoal,
    AwayGoal,
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
                BallUpdateEvent::AwayGoal => {
                    context.result.score.away += 1;
                    // context.result.score.details.push(GoalDetail {
                    //     player_id: goal_scorer,
                    //     assistant: goal_assistant,
                    //     minute: (current_time / 1000 / 60) as u8,
                    // })
                }
                BallUpdateEvent::HomeGoal => {
                    context.result.score.home += 1;
                    // context.result.score.details.push(GoalDetail {
                    //     player_id: goal_scorer,
                    //     assistant: goal_assistant,
                    //     minute: (current_time / 1000 / 60) as u8,
                    // })
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