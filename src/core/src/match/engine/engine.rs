use crate::r#match::ball::Ball;
use crate::r#match::field::MatchField;
use crate::r#match::position::FieldPosition;
use crate::r#match::squad::TeamSquad;
use crate::r#match::{FootballMatchResult, GameState, MatchPlayer, MatchState};

pub struct FootballEngine<const W: usize, const H: usize> {}

impl<const W: usize, const H: usize> FootballEngine<W, H> {
    pub fn play(home_squad: TeamSquad, away_squad: TeamSquad) -> FootballMatchResult {
        let mut context = MatchContext::new();

        let mut field = MatchField::new(W, H);

        field.setup(home_squad, away_squad);

        context.result.write_team_players(
            field.home_players.as_ref().unwrap(),
            field.away_players.as_ref().unwrap(),
        );

        // First half
        context.state.set_game_state(GameState::FirstHalf);
        Self::play_inner(&mut field, &mut context);

        {
            field.swap_squads();
            Self::play_rest_time(&mut field);
        }

        // Second half
        context.state.set_game_state(GameState::SecondHalf);
        Self::play_inner(&mut field, &mut context);

        if context.result.additinal_time_ms > 0 {
            // additional time
            context.state.set_game_state(GameState::ExtraTime);

            Self::play_inner(&mut field, &mut context);
        }

        context.result
    }

    fn play_rest_time(field: &mut MatchField) {
        field.players.iter_mut().for_each(|p| {
            p.player_attributes.rest(1000);
        })
    }

    fn play_inner(field: &mut MatchField, context: &mut MatchContext) -> u64 {
        let mut additional_time: u64 = 0;

        while context.time.time <= MATCH_HALF_TIME_MS {
            let ball_update_events = field.ball.update(&context.state);

            // handle ball
            Ball::handle_events(ball_update_events, context);

            let player_positions: Vec<FieldPosition> =
                field.players.iter().map(|p| p.position).collect();

            let player_update_events = field
                .players
                .iter_mut()
                .flat_map(|player| {
                    player.update(&context.state, &field.ball.position, &player_positions)
                })
                .collect();

            // handle player
            MatchPlayer::handle_events(player_update_events, context);

            field.write_match_positions(&mut context.result, context.time.time);

            context.increment_time();
        }

        additional_time
    }
}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}

pub struct MatchContext {
    pub state: MatchState,
    time: MatchTime,
    pub result: FootballMatchResult,
}

impl MatchContext {
    pub fn new() -> Self {
        MatchContext {
            state: MatchState::new(),
            time: MatchTime::new(),
            result: FootballMatchResult::with_match_time(MATCH_HALF_TIME_MS),
        }
    }

    pub fn increment_time(&mut self) {
        self.time.inc(MATCH_TIME_INCREMENT_MS);
    }
}

const MATCH_TIME_INCREMENT_MS: u64 = 10;
const MATCH_HALF_TIME_MS: u64 = 1 * 60 * 1000;

pub struct MatchTime {
    pub time: u64,
}

impl MatchTime {
    pub fn new() -> Self {
        MatchTime { time: 0 }
    }

    pub fn inc(&mut self, val: u64) {
        self.time += val;
    }
}
