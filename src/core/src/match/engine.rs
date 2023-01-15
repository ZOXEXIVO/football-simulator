use crate::r#match::squad::{PositionType, Squad, SquadPlayer, POSITION_POSITIONING};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng, RngCore};
use rand_distr::num_traits::real::Real;
use serde::Serialize;
use std::collections::HashMap;

const TIME_STEP_MS: u64 = 100;
const MATCH_TIME: u64 = 45 * 60 * 100;

pub struct FootballEngine {
    pub home_squad: Squad,
    pub away_squad: Squad,
}

impl FootballEngine {
    pub fn new(home_squad: Squad, away_squad: Squad) -> Self {
        FootballEngine {
            home_squad,
            away_squad,
        }
    }

    pub fn play(self) -> FootballMatchDetails {
        Field::new(150, 100, self.home_squad, self.away_squad).play()
    }
}

fn setup_players(home_squad: Squad, away_squad: Squad) -> Vec<(SquadPlayer, FieldPosition)> {
    let mut players: Vec<(SquadPlayer, FieldPosition)> = Vec::new();

    // home
    home_squad.main_squad.into_iter().for_each(|home_player| {
        POSITION_POSITIONING
            .iter()
            .filter(|(positioning, _, _)| *positioning == home_player.tactics_position)
            .map(|(_, home_position, _)| home_position)
            .for_each(|position| {
                if let PositionType::Home(x, y) = position {
                    players.push((home_player, FieldPosition::new(*x, *y)));
                }
            });
    });

    // away
    away_squad.main_squad.into_iter().for_each(|away_player| {
        POSITION_POSITIONING
            .iter()
            .filter(|(positioning, _, _)| *positioning == away_player.tactics_position)
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
pub struct MatchPositionData {
    pub ball_positions: Vec<PositionDataItem>,
    pub player_positions: HashMap<u32, Vec<PositionDataItem>>,
}

impl MatchPositionData {
    pub fn new() -> Self {
        MatchPositionData {
            ball_positions: Vec::new(),
            player_positions: HashMap::new(),
        }
    }

    pub fn add_player_positions(&mut self, player_id: u32, timestamp: u64, x: i16, y: i16) {
        if let Some(player_data) = self.player_positions.get_mut(&player_id) {
            player_data.push(PositionDataItem::new(timestamp, x, y));
        } else {
            self.player_positions
                .insert(player_id, vec![PositionDataItem::new(timestamp, x, y)]);
        }
    }

    pub fn add_ball_positions(&mut self, timestamp: u64, x: i16, y: i16) {
        self.ball_positions
            .push(PositionDataItem::new(timestamp, x, y));
    }
}

#[derive(Debug, Clone)]
pub struct PositionDataItem {
    pub timestamp: u64,
    pub x: i16,
    pub y: i16,
}

impl PositionDataItem {
    pub fn new(timestamp: u64, x: i16, y: i16) -> Self {
        PositionDataItem { timestamp, x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Score {
    pub home: i32,
    pub away: i32,
}

pub struct Field {
    pub width: u16,
    pub height: u16,
    pub ball: Ball,
    pub players: Vec<(SquadPlayer, FieldPosition)>,
}

impl Field {
    pub fn new(width: u16, height: u16, home_squad: Squad, away_squad: Squad) -> Self {
        let mut players_container =
            Vec::with_capacity(home_squad.main_squad.len() + away_squad.main_squad.len());

        for (player, position) in setup_players(home_squad, away_squad) {
            players_container.push((player, position));
        }

        Field {
            width,
            height,
            ball: Ball::new(width as i16 / 2, height as i16 / 2),
            players: players_container,
        }
    }

    pub fn play(&mut self) -> FootballMatchDetails {
        let mut match_details = FootballMatchDetails::new(Score { home: 0, away: 0 });

        self.play_first_half(&mut match_details);

        self.play_rest(&mut match_details);

        self.play_second_half(&mut match_details);

        match_details
    }

    fn play_first_half(&mut self, match_details: &mut FootballMatchDetails) {
        let mut ms_step = 1;
        let mut current_time: u64 = 0;

        let mut rnd = thread_rng();

        while current_time <= MATCH_TIME {
            self.ball.move_ball();

            let speed = rnd.next_u32() % 3;

            let mut has_collision = false;

            // update player positions and decisions
            for (player, position) in self.players.iter_mut() {
                if Self::is_collision(&self.ball.position, position) {
                    has_collision = true;

                    player.has_ball = true;

                    self.ball.move_towards_player(position);
                } else {
                    player.has_ball = false;

                    player.speed = speed as i16;
                    //player.decision_tree.predict(self.ball, position);
                    position.x += player.speed * ms_step;
                    position.y += player.speed * ms_step;
                }

                if !has_collision {
                    self.ball.random_movement();
                }
            }

            // check for goal
            if self.ball.position.x >= self.width as i16 {
                match_details.score.home += 1;
            } else if self.ball.position.x <= 0 {
                match_details.score.away += 1;
            }

            current_time += TIME_STEP_MS;

            self.write_match_positions(match_details, current_time);
        }
    }

    fn is_collision(ball_position: &FieldPosition, player_position: &FieldPosition) -> bool {
        const COLLISION_RADIUS: i16 = 10;

        let x_diff = (ball_position.x - player_position.x).abs();
        let y_diff = (ball_position.y - player_position.y).abs();

        x_diff <= COLLISION_RADIUS && y_diff <= COLLISION_RADIUS
    }

    fn play_rest(&mut self, _match_details: &mut FootballMatchDetails) {}

    fn play_second_half(&mut self, _match_details: &mut FootballMatchDetails) {}

    pub fn write_match_positions(&self, match_details: &mut FootballMatchDetails, timestamp: u64) {
        // player positions
        self.players.iter().for_each(|(player, position)| {
            match_details.position_data.add_player_positions(
                player.player_id,
                timestamp,
                position.x,
                position.y,
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

pub struct Ball {
    pub position: FieldPosition,
    pub speed: i16,
    pub direction: FieldPosition,
    rnd: ThreadRng,
}

impl Ball {
    pub fn new(x: i16, y: i16) -> Self {
        Ball {
            position: FieldPosition { x, y },
            speed: 0,
            direction: FieldPosition { x: 0, y: 0 },
            rnd: thread_rng(),
        }
    }

    pub fn move_ball(&mut self) {
        let speed = -2 + (self.rnd.next_u64() % 2) as i16;
        let speed2 = -2 + (self.rnd.next_u64() % 2) as i16;

        self.position.x += speed * speed2;
        self.position.y += speed * speed2;
    }

    fn random_movement(&mut self) {
        let mut rnd = thread_rng();

        let random_x = rnd.gen_range(-1..2);
        let random_y = rnd.gen_range(-1..2);

        self.position.x += random_x;
        self.position.y += random_y;
    }

    fn move_towards_player(&mut self, player_pos: &FieldPosition) {
        let dx = (player_pos.x - self.position.x) as f64;
        let dy = (player_pos.y - self.position.y) as f64;

        let distance = (dx.powi(2) + dy.powi(2)).sqrt();

        self.position.x += ((dx / distance) * self.speed as f64) as i16;
        self.position.y += ((dy / distance) * self.speed as f64) as i16;
    }
}

pub struct FieldPosition {
    pub x: i16,
    pub y: i16,
}

impl FieldPosition {
    pub fn new(x: i16, y: i16) -> Self {
        FieldPosition { x, y }
    }
}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}
