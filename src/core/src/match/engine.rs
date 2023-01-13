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
        Field::new(150, 100, &self.home_squad, &self.away_squad).play()
    }
}

fn setup_players<'s>(
    home_squad: &'s Squad,
    away_squad: &'s Squad,
) -> Vec<(&'s SquadPlayer<'s>, FieldPosition)> {
    let mut players: Vec<(&SquadPlayer<'s>, FieldPosition)> = Vec::new();

    // home
    home_squad.main_squad.iter().for_each(|home_player| {
        POSITION_POSITIONING
            .iter()
            .filter(|(positioning, _, _)| *positioning == home_player.position)
            .map(|(_, home_position, _)| home_position)
            .for_each(|position| {
                if let PositionType::Home(x, y) = position {
                    players.push((home_player, FieldPosition::new(*x, *y)));
                }
            });
    });

    // away
    away_squad.main_squad.iter().for_each(|away_player| {
        POSITION_POSITIONING
            .iter()
            .filter(|(positioning, _, _)| *positioning == away_player.position)
            .map(|(_, _, away_position)| away_position)
            .for_each(|position| {
                if let PositionType::Away(x, y) = position {
                    players.push((away_player, FieldPosition::new(*x, *y)));
                }
            });
    });

    players
}

#[derive(Debug, Clone)]
pub struct FootballMatchDetails {
    pub score: Score,
    pub players_positions: Vec<PlayerPositionData>,
}

impl FootballMatchDetails {
    pub fn new(score: Score, players_positions: Vec<PlayerPositionData>) -> Self {
        FootballMatchDetails {
            score,
            players_positions,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerPositionData {
    pub player_id: u32,
    pub x: u16,
    pub y: u16,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
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

impl<'s> Field<'s> {
    pub fn new(
        width: u16,
        height: u16,
        home_squad: &'s Squad<'s>,
        away_squad: &'s Squad<'s>,
    ) -> Self {
        let mut players_container =
            Vec::with_capacity(home_squad.main_squad.len() + away_squad.main_squad.len());

        for (player, position) in setup_players(&home_squad, &away_squad) {
            players_container.push((player, position));
        }

        let mut field = Field {
            width,
            height,
            ball: Ball::new(width / 2, height / 2),
            players: players_container,
        };

        field
    }

    pub fn play(&mut self) -> FootballMatchDetails {
        let mut match_details = FootballMatchDetails {
            score: Score { home: 0, away: 0 },
            players_positions: Vec::new(),
        };

        self.play_first_half(&mut match_details);

        self.play_rest(&mut match_details);

        self.play_second_half(&mut match_details);

        match_details
    }

    fn play_first_half(&mut self, match_details: &mut FootballMatchDetails) {
        let ms_step = 100;
        let mut current_time = 0;

        while current_time <= 45 * 60 * 1000 {
            //self.ball.move_ball();

            current_time += ms_step;
        }
    }

    fn play_rest(&mut self, match_details: &mut FootballMatchDetails) {}

    fn play_second_half(&mut self, match_details: &mut FootballMatchDetails) {}

    pub fn write_positions(&self, match_details: &mut FootballMatchDetails) {
        self.players.iter().for_each(|(player, position)| {
            match_details.players_positions.push(PlayerPositionData {
                player_id: player.player.id,
                x: position.x,
                y: position.y,
                timestamp: 0,
            });
        });
    }
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
