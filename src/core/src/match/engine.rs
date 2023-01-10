use crate::{PlayerPosition, PositionType, Squad, SquadPlayer, POSITION_POSITIONING};

pub struct FootballEngine<'s> {
    pub home_squad: Squad<'s>,
    pub away_squad: Squad<'s>,
}

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
        };

        let mut field = Field {
            width: 150,
            height: 100,
            ball: Ball::new(150 / 2, 100 / 2),
            players: Vec::new(),
        };

        setup_players(&mut field, &self.home_squad, &self.away_squad);

        match_details
    }
}

fn setup_players<'s>(field: &'s mut Field<'s>, home_squad: &'s Squad, away_squad: &'s Squad) {
    let mut players: Vec<(&SquadPlayer<'s>, FieldPosition)> = Vec::new();

    // home
    home_squad.main_squad.iter().for_each(|home_player| {
        let player_positioning = POSITION_POSITIONING
            .iter()
            .find(|(positioning, _, _)| *positioning == home_player.position);

        match player_positioning {
            Some((_, PositionType::Home(x, y), _)) => {
                players.push((home_player, FieldPosition::new(*x, *y)));
            }
            _ => panic!("Unknown home player position type"),
        }
    });

    // away
    away_squad.main_squad.iter().for_each(|away_player| {
        let player_positioning = POSITION_POSITIONING
            .iter()
            .find(|(positioning, _, _)| *positioning == away_player.position);

        match player_positioning {
            Some((_, _, PositionType::Away(x, y))) => {
                players.push((away_player, FieldPosition::new(*x, *y)));
            }
            _ => panic!("Unknown away player position type"),
        }
    });

    for (player, position) in players {
        field.players.push((player, position));
    }
}

pub struct FootballMatchDetails {
    pub score: Score,
}

pub struct Score {
    pub home: i32,
    pub away: i32,
}

pub struct Field<'s> {
    pub width: u16,
    pub height: u16,
    pub ball: Ball,
    pub players: Vec<(&'s SquadPlayer<'s>, FieldPosition)>,
}

pub struct Ball {
    pub position: FieldPosition,
}

impl Ball {
    pub fn new(x: u16, y: u16) -> Self {
        Ball {
            position: FieldPosition { x, y },
        }
    }
}

pub struct FieldPosition {
    pub x: u16,
    pub y: u16,
}

impl FieldPosition {
    pub fn new(x: u16, y: u16) -> Self {
        FieldPosition { x, y }
    }
}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}
