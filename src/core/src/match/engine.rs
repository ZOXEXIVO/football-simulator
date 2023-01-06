use crate::Squad;

pub struct FootballEngine<'s> {
    pub home_squad: Squad<'s>,
    pub away_squad: Squad<'s>,
}

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

        let mut field = Field {
            width: 400,
            height: 300,
            objects: FieldObjects {
                ball: Ball::new(200, 150),
                players: vec![],
            },
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

pub struct Field<'s> {
    pub width: u16,
    pub height: u16,
    pub objects: FieldObjects<'s>,
}

pub struct FieldObjects<'s> {
    pub ball: Ball,
    pub players: Vec<Squad<'s>>,
}

impl FieldObjects<'_> {
    pub fn new() -> Self {
        FieldObjects {
            ball: Ball::new(200, 150), // center ball
            players: Vec::new(),
        }
    }
}

pub struct Ball {
    pub position: Position,
}

impl Ball {
    pub fn new(x: u16, y: u16) -> Self {
        Ball {
            position: Position { x, y },
        }
    }
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct PlayerChanges {}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}
