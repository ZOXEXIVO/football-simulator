use crate::r#match::{Ball, MatchContext};

#[derive(Copy, Clone)]
pub enum BallUpdateEvent {
    Goal(GoalSide, Option<u32>)
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
    ) {
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
            }
        }
    }
}