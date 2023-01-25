use crate::r#match::ball::Ball;
use crate::r#match::position::{FieldPosition, MatchPositionData};
use crate::r#match::squad::{PositionType, TeamSquad, POSITION_POSITIONING};
use crate::r#match::{MatchPlayer, PlayerUpdateEvent};

const MATCH_TIME_INCREMENT_MS: u64 = 100;
const MATCH_TIME_MS: u64 = 45 * 60 * 100;

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
        self.field.play()
    }
}

#[derive(Debug, Clone)]
pub struct FootballMatchDetails {
    pub score: Score,
    pub position_data: MatchPositionData,
}

impl FootballMatchDetails {
    pub fn new(score: Score) -> Self {
        FootballMatchDetails {
            score,
            position_data: MatchPositionData::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Score {
    pub home: i32,
    pub away: i32,
}

pub struct Field {
    pub width: usize,
    pub height: usize,
    pub ball: Ball,
    pub players: Vec<MatchPlayer>,
}

impl Field {
    pub fn new(width: usize, height: usize, home_squad: TeamSquad, away_squad: TeamSquad) -> Self {
        let mut players_container =
            Vec::with_capacity(home_squad.main_squad.len() + away_squad.main_squad.len());

        for player in setup_player_on_field(home_squad, away_squad) {
            players_container.push(player);
        }

        Field {
            width,
            height,
            ball: Ball::new(width as f32 / 2.0, height as f32 / 2.0),
            players: players_container,
        }
    }

    pub fn play(&mut self) -> FootballMatchDetails {
        let mut result = FootballMatchDetails::new(Score { home: 0, away: 0 });

        self.play_inner(&mut result);

        result
    }

    fn play_inner(&mut self, match_details: &mut FootballMatchDetails) {
        let mut current_time: u64 = 0;

        while current_time <= MATCH_TIME_MS {
            let ball_evens = self.ball.update();

            let player_events: Vec<PlayerUpdateEvent> =
                self.players.iter_mut().flat_map(|p| p.update()).collect();

            let players_len = self.players.len();

            for player_idx in 0..players_len {
                for other_player_idx in 0..players_len {
                    // if player_idx == other_player_idx {
                    //     continue;
                    // }
                    //
                    // let player = &mut self.players[player_idx];
                    // let other_player = &mut self.players[other_player_idx];
                    //
                    // player.update();
                    //
                    // if player.position == other_player.position {
                    //     //player.collide_with(other_player);
                    // }
                }
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
