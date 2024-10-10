use crate::r#match::events::Event;
use crate::r#match::player::events::PlayerEvent;
use crate::r#match::MatchContext;

#[derive(Copy, Clone)]
pub enum BallEvent {
    Goal(GoalSide, Option<u32>),
    Claimed(u32),
    UnClaim(u32),
    Gained(u32),
}

#[derive(Copy, Clone)]
pub enum GoalSide {
    Home,
    Away,
}

pub struct BallEventDispatcher;

impl BallEventDispatcher {
    pub fn dispatch(event: BallEvent, context: &MatchContext) -> Vec<Event> {
        let mut remaining_events = Vec::new();

        match event {
            BallEvent::Goal(side, goalscorer_player_id) => match side {
                GoalSide::Home => context.score.increment_home_goals(),
                GoalSide::Away => context.score.increment_away_goals(),
            },
            BallEvent::Claimed(player_id) => {
                remaining_events.push(Event::PlayerEvent(PlayerEvent::ClaimBall(player_id)));
            }
            BallEvent::Gained(player_id) => {
                remaining_events.push(Event::PlayerEvent(PlayerEvent::GainBall(player_id)));
            }
            BallEvent::UnClaim(player_id) => {
                remaining_events.push(Event::PlayerEvent(PlayerEvent::UnClaimBall(
                    player_id,
                )));
            }
        }

        remaining_events
    }
}
