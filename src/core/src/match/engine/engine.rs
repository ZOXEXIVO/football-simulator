use crate::r#match::ball::Ball;
use crate::r#match::field::Field;
use crate::r#match::position::FieldPosition;
use crate::r#match::squad::TeamSquad;
use crate::r#match::{FootballMatchResult, GameState, MatchPlayer, MatchState};

const MATCH_TIME_INCREMENT_MS: u64 = 10;
const MATCH_HALF_TIME_MS: u64 = 1 * 60 * 1000;

pub struct FootballEngine<const W: usize, const H: usize> {}

impl<const W: usize, const H: usize> FootballEngine<W, H> {
    pub fn play(home_squad: TeamSquad, away_squad: TeamSquad) -> FootballMatchResult {
        let mut result = FootballMatchResult::with_match_time(MATCH_HALF_TIME_MS * 2);

        let mut field = Field::new(W, H);

        field.setup(home_squad, away_squad);

        result.write_team_players(
            field.home_players.as_ref().unwrap(),
            field.away_players.as_ref().unwrap(),
        );

        let mut match_state = MatchState::new();

        for current_game_state in [GameState::FirstHalf, GameState::SecondHalf] {
            let mut current_time: u64 = 0;

            match_state.set_game_state(current_game_state);

            while current_time <= MATCH_HALF_TIME_MS {
                let ball_update_events = field.ball.update(&match_state);

                // handle ball
                Ball::handle_events(ball_update_events, &match_state, &mut result);

                let player_positions: Vec<FieldPosition> =
                    field.players.iter().map(|p| p.position).collect();

                let player_update_events = field
                    .players
                    .iter_mut()
                    .flat_map(|p| p.update(&match_state, &field.ball.position, &player_positions))
                    .collect();

                // handle player
                MatchPlayer::handle_events(player_update_events, &match_state, &mut result);

                field.write_match_positions(&mut result, current_time);

                current_time += MATCH_TIME_INCREMENT_MS;
            }
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
