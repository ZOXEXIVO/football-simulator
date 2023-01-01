use super::distributions::random;
use crate::club::PlayerPositionType;
use crate::Squad;
use std::mem;

pub struct FootballEngine<'s> {
    pub home_squad: Squad<'s>,
    pub away_squad: Squad<'s>,
}

const MATCH_ACTIONS: u16 = 50;
const DEFAULT_MATCH_EVENTS: usize = 100;

impl<'s> FootballEngine<'s> {
    pub fn new(home_squad: Squad<'s>, away_squad: Squad<'s>) -> Self {
        FootballEngine {
            home_squad,
            away_squad,
        }
    }

    pub fn play(&mut self) -> FootballMatchDetails {
        let mut match_details = FootballMatchDetails {
            score: Score { home: 0, away: 0 },
            events: Vec::with_capacity(DEFAULT_MATCH_EVENTS),
            player_changes: vec![],
        };

        match_details
    }
}

pub struct FootballMatchDetails {
    pub score: Score,
    pub events: Vec<MatchEvent>,
    pub player_changes: Vec<PlayerChanges>,
}

pub struct Score {
    pub home: i32,
    pub away: i32,
}

pub struct PlayerChanges {}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}
