use crate::r#match::ball::Ball;
use crate::r#match::position::{FieldPosition, MatchPositionData};
use crate::r#match::squad::{PositionType, Squad, SquadPlayer, POSITION_POSITIONING};
use rand::{thread_rng, RngCore};

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

fn setup_players(home_squad: Squad, away_squad: Squad) -> Vec<SquadPlayer> {
    let mut players: Vec<SquadPlayer> = Vec::new();

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
                        home_player.position = FieldPosition::new(*x, *y);
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
                        away_player.position = FieldPosition::new(*x, *y);
                        players.push(away_player);
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
pub struct Score {
    pub home: i32,
    pub away: i32,
}

pub struct Field {
    pub width: u16,
    pub height: u16,
    pub ball: Ball,
    pub players: Vec<SquadPlayer>,
}

impl Field {
    pub fn new(width: u16, height: u16, home_squad: Squad, away_squad: Squad) -> Self {
        let mut players_container =
            Vec::with_capacity(home_squad.main_squad.len() + away_squad.main_squad.len());

        for player in setup_players(home_squad, away_squad) {
            players_container.push(player);
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
        let ms_step = 1;
        let mut current_time: u64 = 0;

        let mut rnd = thread_rng();

        while current_time <= MATCH_TIME {
            self.ball.move_ball();

            let speed = rnd.next_u32() % 3;

            // update player positions and decisions
            for player in self.players.iter_mut() {
                if Self::is_collision(&self.ball.position, &player.position) {
                    player.has_ball = true;

                    self.ball.move_towards_player(&player.position);
                } else {
                    player.has_ball = false;

                    player.speed = speed as i16;
                    //player.decision_tree.predict(self.ball, position);
                    player.position.x += player.speed * ms_step;
                    player.position.y += player.speed * ms_step;
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
