use crate::r#match::ball::Ball;
use crate::r#match::field::Field;
use crate::r#match::position::FieldPosition;
use crate::r#match::squad::TeamSquad;
use crate::r#match::{FootballMatchResult, MatchPlayer, MatchState};

const MATCH_TIME_INCREMENT_MS: u64 = 10;
const MATCH_TIME_MS: u64 = 1 * 60 * 1000;

pub struct FootballEngine<const W: usize, const H: usize> {}

impl<const W: usize, const H: usize> FootballEngine<W, H> {
    pub fn play(home_squad: TeamSquad, away_squad: TeamSquad) -> FootballMatchResult {
        let mut result = FootballMatchResult::new(MATCH_TIME_MS);

        let mut field = Field::new(W, H, home_squad, away_squad);

        let mut current_time: u64 = 0;

        let mut state = MatchState::new();

        while current_time <= MATCH_TIME_MS {
            let ball_update_events = field.ball.update();

            // handle ball
            Ball::handle_events(&ball_update_events, &mut result);

            let player_positions: Vec<FieldPosition> =
                field.players.iter().map(|p| p.position).collect();

            let player_update_events = field
                .players
                .iter_mut()
                .flat_map(|p| p.update(&field.ball.position, &player_positions))
                .collect();

            // handle player
            MatchPlayer::handle_events(&player_update_events, &mut result);

            // let players_len = self.players.len();

            current_time += MATCH_TIME_INCREMENT_MS;

            field.write_match_positions(&mut result, current_time);
        }

        result
    }
}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
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
