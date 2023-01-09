use crate::{Squad, SquadPlayer};

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
            width: 400,
            height: 300,
            ball: Ball::new(200, 150),
            players: Vec::new(),
        };

        match_details
    }

    fn setup_home_players(
        home_squad: &Squad,
        field: &mut Field,
    ) -> Vec<(SquadPlayer<'s>, FieldPosition)> {
        let mut players: Vec<(SquadPlayer<'s>, FieldPosition)> = Vec::new();

        // let field_width = field.width;
        // let field_height = field.height;
        //
        // let defense_count = home_squad
        //     .players
        //     .iter()
        //     .filter(|(_, pos)| *pos == FieldPosition::Defense)
        //     .count();
        // let midfield_count = home_squad
        //     .players
        //     .iter()
        //     .filter(|(_, pos)| *pos == FieldPosition::Midfield)
        //     .count();
        // let attack_count = home_squad
        //     .players
        //     .iter()
        //     .filter(|(_, pos)| *pos == FieldPosition::Attack)
        //     .count();
        //
        // let y_step = field_height / (defense_count + midfield_count + attack_count) as u16;
        // let mut current_y = 0;
        //
        // for (player, position) in home_squad.players.iter() {
        //     match position {
        //         FieldPosition::Defense => {
        //             players.push((player, *position));
        //             current_y += y_step;
        //         }
        //         FieldPosition::Midfield => {
        //             players.push((player, *position));
        //             current_y += y_step;
        //         }
        //         FieldPosition::Attack => {
        //             players.push((player, *position));
        //             current_y += y_step;
        //         }
        //     }
        // }

        players
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
    pub players: Vec<(SquadPlayer<'s>, FieldPosition)>,
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

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}
