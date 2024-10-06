use crate::r#match::player::events::{PlayerUpdateEvent, PlayerUpdateEventCollection};
use crate::r#match::MatchContext;

#[derive(Copy, Clone)]
pub enum BallUpdateEvent {
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

pub struct BallEvents;

impl BallEvents {
    pub fn handle_events<'a>(
        _current_time: u64,
        events: impl Iterator<Item = BallUpdateEvent>,
        context: &MatchContext,
    ) -> PlayerUpdateEventCollection {
        let mut player_events = PlayerUpdateEventCollection::new();

        for event in events {
            match event {
                BallUpdateEvent::Goal(side, goalscorer_player_id) => match side {
                    GoalSide::Home => context.score.increment_home_goals(),
                    GoalSide::Away => context.score.increment_away_goals(),
                },
                BallUpdateEvent::Claimed(player_id) => {
                    player_events.add(PlayerUpdateEvent::ClaimBall(player_id));
                }
                BallUpdateEvent::Gained(player_id) => {
                    player_events.add(PlayerUpdateEvent::GainBall(player_id));
                }
                BallUpdateEvent::UnClaim(player_id) => {
                    player_events.add(PlayerUpdateEvent::UnClaimBall(player_id));
                }
            }
        }

        player_events
    }
}
