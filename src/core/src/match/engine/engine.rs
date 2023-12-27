use crate::r#match::ball::Ball;
use crate::r#match::field::MatchField;
use crate::r#match::position::{PlayerFieldPosition, VectorExtensions};
use crate::r#match::squad::TeamSquad;
use crate::r#match::{GameState, MatchPlayer, MatchResultRaw, MatchState, StateManager};
use nalgebra::Vector3;

pub struct FootballEngine<const W: usize, const H: usize> {}

impl<const W: usize, const H: usize> FootballEngine<W, H> {
    pub fn new() -> Self {
        FootballEngine {}
    }

    pub fn play(home_squad: TeamSquad, away_squad: TeamSquad) -> MatchResultRaw {
        let mut field = MatchField::new(W, H, home_squad, away_squad);

        let mut context = MatchContext::new(&field.size);

        let mut state_manager = StateManager::new();

        while let Some(state) = state_manager.next() {
            context.state.set(state);

            let play_state_result = Self::play_inner(&mut field, &mut context);

            StateManager::handle_state_finish(&mut context, &mut field, play_state_result);
        }

        // TODO
        context.result.home_players = field.home_players.unwrap();
        context.result.away_players = field.away_players.unwrap();

        context.result
    }

    fn play_inner(field: &mut MatchField, context: &mut MatchContext) -> PlayMatchStateResult {
        let result = PlayMatchStateResult::new();

        while context.increment_time() {
            Self::game_tick(field, context);
        }

        result
    }

    pub fn game_tick(field: &mut MatchField, context: &mut MatchContext){
        let ball_update_events = field.ball.update(context);

        // handle ball
        Ball::handle_events(context.time.time, ball_update_events, context);

        Self::play_players(field, context, MatchObjectsPositions::from(&field));

        field.write_match_positions(&mut context.result, context.time.time);
    }

    fn play_players(
        field: &mut MatchField,
        context: &mut MatchContext,
        objects_positions: MatchObjectsPositions,
    ) {
        let player_update_events = field
            .players
            .iter_mut()
            .flat_map(|player| player.update(context, &objects_positions))
            .collect();

        // handle player
        MatchPlayer::handle_events(player_update_events, &mut field.ball, context);
    }
}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}

pub struct MatchContext {
    pub state: GameState,
    pub time: MatchTime,
    pub result: MatchResultRaw,
    pub field_size: MatchFieldSize,
}

impl MatchContext {
    pub fn new(field_size: &MatchFieldSize) -> Self {
        MatchContext {
            state: GameState::new(),
            time: MatchTime::new(),
            result: MatchResultRaw::with_match_time(MATCH_HALF_TIME_MS),
            field_size: MatchFieldSize::clone(&field_size),
        }
    }

    pub fn increment_time(&mut self) -> bool {
        self.time.increment(MATCH_TIME_INCREMENT_MS) < MATCH_HALF_TIME_MS
    }

    pub fn add_time(&mut self, time: u64) {
        self.time.increment(time);
    }
}

#[derive(Clone)]
pub struct MatchFieldSize {
    pub width: usize,
    pub height: usize,
}

impl MatchFieldSize {
    pub fn new(width: usize, height: usize) -> Self {
        MatchFieldSize { width, height }
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

    pub fn increment(&mut self, val: u64) -> u64 {
        self.time += val;
        self.time
    }
}

pub struct MatchObjectsPositions {
    pub ball_position: Vector3<f32>,
    pub ball_velocity: Vector3<f32>,
    pub players_positions: Vec<PlayerFieldPosition>,
}

impl MatchObjectsPositions {
    pub fn from(field: &MatchField) -> Self {
        MatchObjectsPositions {
            ball_position: field.ball.position,
            ball_velocity: field.ball.velocity,
            players_positions: field
                .players
                .iter()
                .map(|p| PlayerFieldPosition {
                    player_id: p.player_id,
                    is_home: p.is_home,
                    position: p.position,
                })
                .collect(),
        }
    }

    pub fn find_closest_teammate(
        &self,
        current_player: &MatchPlayer,
        _state: &MatchState,
    ) -> Option<Vector3<f32>> {
        let max_pass_distance = 30.0;

        let mut closest_teammate = None;
        let mut closest_distance = f32::MAX;

        for teammate_player_position in self.players_positions.iter() {
            if teammate_player_position.player_id == current_player.player_id {
                continue;
            }

            if teammate_player_position.is_home != current_player.is_home {
                continue;
            }

            let distance = current_player
                .position
                .distance_to(&teammate_player_position.position);

            if distance < closest_distance && distance < max_pass_distance {
                closest_teammate = Some(teammate_player_position.position);
                closest_distance = distance;
            }
        }

        closest_teammate
    }
}

pub struct PlayMatchStateResult {
    pub additional_time: u64,
}

impl PlayMatchStateResult {
    pub fn new() -> Self {
        PlayMatchStateResult { additional_time: 0 }
    }
}
