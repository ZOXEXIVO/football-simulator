use crate::r#match::ball::Ball;
use crate::r#match::position::{FieldPosition, MatchPositionData};
use crate::r#match::squad::{PositionType, TeamSquad, POSITION_POSITIONING};
use crate::r#match::MatchPlayer;

const MATCH_TIME_INCREMENT_MS: u64 = 10;
const MATCH_TIME_MS: u64 = 1 * 60 * 1000;

pub struct FootballEngine<const W: usize, const H: usize> {
    pub field: Field,
    pub state: GameState,
}

impl<const W: usize, const H: usize> FootballEngine<W, H> {
    pub fn new(home_squad: TeamSquad, away_squad: TeamSquad) -> Self {
        FootballEngine {
            field: Field::new(W, H, home_squad, away_squad),
            state: GameState::FirstHalf,
        }
    }

    pub fn play(&mut self) -> FootballMatchDetails {
        let mut details = FootballMatchDetails::new();

        self.field.play(&mut details);

        details
    }
}

#[derive(Debug, Clone)]
pub struct FootballMatchDetails {
    pub score: Score,
    pub position_data: MatchPositionData,

    pub home_players: FieldSquad,
    pub away_players: FieldSquad,
}

impl FootballMatchDetails {
    pub fn new() -> Self {
        FootballMatchDetails {
            score: Score::new(),
            position_data: MatchPositionData::new(),
            home_players: FieldSquad::new(),
            away_players: FieldSquad::new(),
        }
    }

    pub fn write_team_players(
        &mut self,
        home_team_players: &FieldSquad,
        away_team_players: &FieldSquad,
    ) {
        self.home_players = FieldSquad::from(home_team_players);
        self.away_players = FieldSquad::from(away_team_players);
    }
}

#[derive(Debug, Clone)]
pub struct FieldSquad {
    pub main: Vec<u32>,
    pub substitutes: Vec<u32>,
}

impl FieldSquad {
    pub fn new() -> Self {
        FieldSquad {
            main: Vec::new(),
            substitutes: Vec::new(),
        }
    }

    pub fn from(field_squad: &FieldSquad) -> Self {
        FieldSquad {
            main: field_squad.main.to_vec(),
            substitutes: field_squad.substitutes.to_vec(),
        }
    }

    pub fn count(&self) -> usize {
        self.main.len() + self.substitutes.len()
    }
}

#[derive(Debug, Clone)]
pub struct Score {
    pub home: i32,
    pub away: i32,
}

impl Score {
    pub fn new() -> Self {
        Score { home: 0, away: 0 }
    }
}

pub struct Field {
    pub width: usize,
    pub height: usize,
    pub ball: Ball,
    pub players: Vec<MatchPlayer>,
    pub substitutes: Vec<MatchPlayer>,

    pub home_players: FieldSquad,
    pub away_players: FieldSquad,
}

impl Field {
    pub fn new(width: usize, height: usize, home_squad: TeamSquad, away_squad: TeamSquad) -> Self {
        let home_players = FieldSquad {
            main: home_squad.main_squad.iter().map(|p| p.player_id).collect(),
            substitutes: home_squad.substitutes.iter().map(|p| p.player_id).collect(),
        };

        let away_players = FieldSquad {
            main: away_squad.main_squad.iter().map(|p| p.player_id).collect(),
            substitutes: away_squad.substitutes.iter().map(|p| p.player_id).collect(),
        };

        let (players_on_field, substitutes) = setup_player_on_field(home_squad, away_squad);

        Field {
            width,
            height,
            ball: Ball::with_coord(width as f32 / 2.0, height as f32 / 2.0),
            players: players_on_field,
            substitutes,
            home_players,
            away_players,
        }
    }

    pub fn play(&mut self, details: &mut FootballMatchDetails) {
        self.play_inner(details);

        // write player disposition
        details.write_team_players(&self.home_players, &self.away_players);
    }

    fn play_inner(&mut self, match_details: &mut FootballMatchDetails) {
        let mut current_time: u64 = 0;

        while current_time <= MATCH_TIME_MS {
            let ball_update_events = self.ball.update();

            // handle ball
            Ball::handle_events(&ball_update_events, match_details);

            let player_positions: Vec<FieldPosition> =
                self.players.iter().map(|p| p.position).collect();

            let player_update_events = self
                .players
                .iter_mut()
                .flat_map(|p| p.update(&self.ball.position, &player_positions))
                .collect();

            // handle player
            MatchPlayer::handle_events(&player_update_events, match_details);

            let players_len = self.players.len();

            current_time += MATCH_TIME_INCREMENT_MS;

            self.write_match_positions(match_details, current_time);
        }
    }

    fn is_collision(ball_position: &FieldPosition, player_position: &FieldPosition) -> bool {
        const COLLISION_RADIUS: f32 = 2.0;

        let x_diff = (ball_position.x - player_position.x).abs();
        let y_diff = (ball_position.y - player_position.y).abs();

        x_diff <= COLLISION_RADIUS && y_diff <= COLLISION_RADIUS
    }

    pub fn write_match_positions(&self, match_details: &mut FootballMatchDetails, timestamp: u64) {
        // player positions
        self.players.iter().for_each(|player| {
            match_details.position_data.add_player_positions(
                player.player_id,
                timestamp,
                player.position.x,
                player.position.y,
            );
        });

        // player positions
        self.substitutes.iter().for_each(|sub_player| {
            match_details.position_data.add_player_positions(
                sub_player.player_id,
                timestamp,
                sub_player.position.x,
                sub_player.position.y,
            );
        });

        // write positions
        match_details.position_data.add_ball_positions(
            timestamp,
            self.ball.position.x,
            self.ball.position.y,
        );
    }
}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}

fn setup_player_on_field(
    home_squad: TeamSquad,
    away_squad: TeamSquad,
) -> (Vec<MatchPlayer>, Vec<MatchPlayer>) {
    let mut players_on_field: Vec<MatchPlayer> = Vec::with_capacity(22);
    let mut substitutes: Vec<MatchPlayer> = Vec::with_capacity(30);

    // home
    home_squad
        .main_squad
        .into_iter()
        .for_each(|mut home_player| {
            let tactics_position = home_player.tactics_position;

            POSITION_POSITIONING
                .iter()
                .filter(|(positioning, _, _)| *positioning == tactics_position)
                .map(|(_, home_position, _)| home_position)
                .for_each(|position| {
                    if let PositionType::Home(x, y) = position {
                        home_player.position = FieldPosition::new(*x as f32, *y as f32);
                        home_player.start_position = FieldPosition::new(*x as f32, *y as f32);

                        players_on_field.push(home_player);
                    }
                });
        });

    home_squad
        .substitutes
        .into_iter()
        .for_each(|mut home_player| {
            home_player.position = FieldPosition::new(0.0, 0.0);
            substitutes.push(home_player);
        });

    // away
    away_squad
        .main_squad
        .into_iter()
        .for_each(|mut away_player| {
            let tactics_position = away_player.tactics_position;

            POSITION_POSITIONING
                .iter()
                .filter(|(positioning, _, _)| *positioning == tactics_position)
                .map(|(_, _, away_position)| away_position)
                .for_each(|position| {
                    if let PositionType::Away(x, y) = position {
                        away_player.position = FieldPosition::new(*x as f32, *y as f32);
                        away_player.start_position = FieldPosition::new(*x as f32, *y as f32);

                        players_on_field.push(away_player);
                    }
                });
        });

    away_squad
        .substitutes
        .into_iter()
        .for_each(|mut away_player| {
            away_player.position = FieldPosition::new(0.0, 0.0);
            substitutes.push(away_player);
        });

    (players_on_field, substitutes)
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    FirstHalf,
    SecondHalf,
    ExtraTime,
    PenaltyShootout,
    Halftime,
    Fulltime,
    GameOver,
}
