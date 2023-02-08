use crate::r#match::ball::Ball;
use crate::r#match::position::{FieldPosition, MatchPositionData};
use crate::r#match::squad::{PositionType, TeamSquad, POSITION_POSITIONING};
use crate::r#match::{MatchPlayer, PlayerUpdateEvent};

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
    pub home_team_players: Vec<u32>,
    pub away_team_players: Vec<u32>,
}

impl FootballMatchDetails {
    pub fn new() -> Self {
        FootballMatchDetails {
            score: Score::new(),
            position_data: MatchPositionData::new(),
            home_team_players: Vec::new(),
            away_team_players: Vec::new(),
        }
    }

    pub fn write_team_players(
        &mut self,
        home_team_players: &Vec<u32>,
        away_team_players: &Vec<u32>,
    ) {
        for &player_id in home_team_players {
            self.home_team_players.push(player_id);
        }

        for &player_id in away_team_players {
            self.away_team_players.push(player_id);
        }
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
    // Player disposition
    pub home_team_players: Vec<u32>,
    pub away_team_players: Vec<u32>,
}

impl Field {
    pub fn new(width: usize, height: usize, home_squad: TeamSquad, away_squad: TeamSquad) -> Self {
        let mut players_container =
            Vec::with_capacity(home_squad.main_squad.len() + away_squad.main_squad.len());

        let home_team_players: Vec<u32> =
            home_squad.main_squad.iter().map(|p| p.player_id).collect();
        let away_team_players: Vec<u32> =
            away_squad.main_squad.iter().map(|p| p.player_id).collect();

        for player in setup_player_on_field(home_squad, away_squad) {
            players_container.push(player);
        }

        Field {
            width,
            height,
            ball: Ball::with_coord(width as f32 / 2.0, height as f32 / 2.0),
            players: players_container,
            home_team_players,
            away_team_players,
        }
    }

    pub fn play(&mut self, details: &mut FootballMatchDetails) {
        self.play_inner(details);

        // write player disposition
        details.write_team_players(&self.home_team_players, &self.away_team_players);
    }

    fn play_inner(&mut self, match_details: &mut FootballMatchDetails) {
        let mut current_time: u64 = 0;

        while current_time <= MATCH_TIME_MS {
            Ball::handle_events(&self.ball.update(), match_details);
            MatchPlayer::handle_events(
                &self.players.iter_mut().flat_map(|p| p.update()).collect(),
                match_details,
            );

            let players_len = self.players.len();

            for player_idx in 0..players_len {
                for other_player_idx in 0..players_len {}
            }

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

fn setup_player_on_field(home_squad: TeamSquad, away_squad: TeamSquad) -> Vec<MatchPlayer> {
    let mut players: Vec<MatchPlayer> = Vec::new();

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
                        players.push(home_player);
                    }
                });
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
                        players.push(away_player);
                    }
                });
        });

    players
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
