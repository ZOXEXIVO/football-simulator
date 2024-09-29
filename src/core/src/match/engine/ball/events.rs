use crate::r#match::{MatchContext};
use crate::r#match::player::events::PlayerUpdateEvent;

#[derive(Copy, Clone)]
pub enum BallUpdateEvent {
    Goal(GoalSide, Option<u32>),
    Claimed(u32),
    Gained(u32),
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
        events: impl Iterator<Item = BallUpdateEvent>,
        context: &MatchContext,
    ) -> Vec<PlayerUpdateEvent> {
        let mut player_events = Vec::new();

        for event in events {
            match event {
                BallUpdateEvent::Goal(side, goalscorer_player_id) => {
                    match side {
                        GoalSide::Home => {
                            context.result.score.increment_home_goals()
                        }
                        GoalSide::Away => {
                            context.result.score.increment_away_goals()
                        }
                    }
                }
                BallUpdateEvent::Claimed(player_id) => {
                    player_events.push(PlayerUpdateEvent::ClaimBall(player_id));
                }
                BallUpdateEvent::Gained(player_id) => {
                    player_events.push(PlayerUpdateEvent::GainBall(player_id));
                }
            }
        }

        player_events
    }
}