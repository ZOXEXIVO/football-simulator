use crate::r#match::ball::Ball;
use crate::r#match::field::MatchField;
use crate::r#match::position::{PlayerFieldPosition, VectorExtensions};
use crate::r#match::squad::TeamSquad;
use crate::r#match::{MatchGameState, MatchPlayer, MatchResultRaw, MatchState, StateManager};
use nalgebra::Vector3;

pub struct FootballEngine<const W: usize, const H: usize> {}

impl<const W: usize, const H: usize> FootballEngine<W, H> {
    pub fn new() -> Self {
        FootballEngine {}
    }

    pub fn play(home_squad: TeamSquad, away_squad: TeamSquad) -> MatchResultRaw {
        let mut field = MatchField::new(W, H, home_squad, away_squad);

        let mut context = MatchContext::new(&field);

        let mut state_manager = StateManager::new();
        let mut state: MatchState = MatchState::Initial;

        while !state.is_end_state() {
            state = state_manager.next();

            context.state.set(state);

            Self::play_inner(&mut field, &mut context);

            field.swap_squads();
            field.swap_player_positions();
        }

        context.result
    }

    fn play_inner(field: &mut MatchField, context: &mut MatchContext) -> u64 {
        let additional_time: u64 = 0;

        while context.increment_time() {
            let ball_update_events = field.ball.update(context);

            // handle ball
            Ball::handle_events(context.time.time, ball_update_events, context);

            // setup positions
            let objects_positions = MatchObjectsPositions::from(&field);

            let player_update_events = field
                .players
                .iter_mut()
                .flat_map(|player| {
                    player.update(context.time.time, &context.state, &objects_positions)
                })
                .collect();

            // handle player
            MatchPlayer::handle_events(player_update_events, context);

            field.write_match_positions(&mut context.result, context.time.time);
        }

        additional_time
    }

    fn play_rest_time(field: &mut MatchField) {
        field.players.iter_mut().for_each(|p| {
            p.player_attributes.rest(1000);
        })
    }
}

pub enum MatchEvent {
    MatchPlayed(u32, bool, u8),
    Goal(u32),
    Assist(u32),
    Injury(u32),
}

pub struct MatchContext {
    pub state: MatchGameState,
    time: MatchTime,
    pub result: MatchResultRaw,
    pub field_size: MatchFieldSize,
}

impl MatchContext {
    pub fn new(field: &MatchField) -> Self {
        MatchContext {
            state: MatchGameState::new(),
            time: MatchTime::new(),
            result: MatchResultRaw::with_match_time(MATCH_HALF_TIME_MS),
            field_size: MatchFieldSize::clone(&field.size),
        }
    }

    pub fn increment_time(&mut self) -> bool {
        self.time.increment(MATCH_TIME_INCREMENT_MS) < MATCH_HALF_TIME_MS
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
    pub ball_positions: Vector3<f32>,
    pub ball_velocity: Vector3<f32>,
    pub players_positions: Vec<PlayerFieldPosition>,
}

impl MatchObjectsPositions {
    pub fn from(field: &MatchField) -> Self {
        MatchObjectsPositions {
            ball_positions: field.ball.position,
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

    fn find_closest_teammate(
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
