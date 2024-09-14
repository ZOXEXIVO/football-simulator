use crate::r#match::field::MatchField;
use crate::r#match::squad::TeamSquad;
use crate::r#match::{GameState, GameTickContext, MatchObjectsPositions, MatchPlayer, MatchResultRaw, StateManager};
use std::collections::HashMap;
use crate::r#match::ball::events::{BallEvents, BallUpdateEvent};
use crate::r#match::engine::collisions::ObjectCollisionsDetector;
use crate::r#match::player::events::{PlayerEvents, PlayerUpdateEvent};

pub struct FootballEngine<const W: usize, const H: usize> {}

impl<const W: usize, const H: usize> FootballEngine<W, H> {
    pub fn new() -> Self {
        FootballEngine {}
    }

    pub fn play(home_squad: TeamSquad, away_squad: TeamSquad) -> MatchResultRaw {
        let team_a_id = home_squad.team_id;
        let team_b_id = away_squad.team_id;

        let players = MatchPlayerCollection::from_squads(&home_squad, &away_squad);

        let mut field = MatchField::new(W, H, home_squad, away_squad);

        let mut context = MatchContext::new(&field.size, players, team_a_id, team_b_id);

        let mut state_manager = StateManager::new();

        while let Some(state) = state_manager.next() {
            context.state.set(state);

            let play_state_result = Self::play_inner(&mut field, &mut context);

            StateManager::handle_state_finish(&mut context, &mut field, play_state_result);
        }

        // TODO
        context.result.home_players = field.home_players.unwrap();
        context.result.away_players = field.away_players.unwrap();

        context.result.fill_details(context.players.raw_players());

        context.result
    }

    fn play_inner(field: &mut MatchField, context: &mut MatchContext) -> PlayMatchStateResult {
        let result = PlayMatchStateResult::new();

        while context.increment_time() {
            Self::game_tick(field, context);
        }

        result
    }

    pub fn game_tick(field: &mut MatchField, context: &mut MatchContext) {
        let game_tick_context = GameTickContext {
            objects_positions: MatchObjectsPositions::from(&field),
        };

        let (collision_ball_events, collision_player_events) =
            ObjectCollisionsDetector::process(&game_tick_context.objects_positions);

        Self::play_ball(field, context, collision_ball_events);
        Self::play_players(field, context, &game_tick_context, collision_player_events);

        field.write_match_positions(&mut context.result, context.time.time);
    }

    fn play_ball(
        field: &mut MatchField,
        context: &mut MatchContext,
        ball_collision_events: Vec<BallUpdateEvent>
    ) {
        let ball_events = field.ball.update(context);

        let all_ball_events =  ball_events.iter().chain(&ball_collision_events);

        BallEvents::handle_events(context.time.time, &mut field.ball, all_ball_events, context);
    }

    fn play_players(
        field: &mut MatchField,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_collision_events: Vec<PlayerUpdateEvent>
    ){
        let mut all_player_events = {
            let player_events: Vec<PlayerUpdateEvent> = field.players
                .iter_mut()
                .flat_map(|player| player.update(context, tick_context))
                .collect();

             player_events.into_iter().chain(player_collision_events).into_iter()
        };

        PlayerEvents::process(&mut all_player_events, &mut field.ball, context);
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
    pub players: MatchPlayerCollection,
}

impl MatchContext {
    pub fn new(field_size: &MatchFieldSize, players: MatchPlayerCollection, team_a_id: u32, team_b_id: u32) -> Self {
        MatchContext {
            state: GameState::new(),
            time: MatchTime::new(),
            result: MatchResultRaw::with_match_time(MATCH_HALF_TIME_MS, team_a_id, team_b_id),
            field_size: MatchFieldSize::clone(&field_size),
            players,
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

pub struct MatchPlayerCollection {
    players: HashMap<u32, MatchPlayer>,
}

impl MatchPlayerCollection {
    pub fn from_squads(home_squad: &TeamSquad, away_squad: &TeamSquad) -> Self {
        let mut result = HashMap::new();

        // home_main
        for hs_m in &home_squad.main_squad {
            result.insert(hs_m.player_id, hs_m.clone());
        }

        // home_subs
        for hs_s in &home_squad.substitutes {
            result.insert(hs_s.player_id, hs_s.clone());
        }

        // home_main
        for as_m in &away_squad.main_squad {
            result.insert(as_m.player_id, as_m.clone());
        }

        // home_subs
        for as_s in &away_squad.substitutes {
            result.insert(as_s.player_id, as_s.clone());
        }

        MatchPlayerCollection { players: result }
    }

    pub fn get(&self, player_id: u32) -> Option<&MatchPlayer> {
        self.players.get(&player_id)
    }

    pub fn get_mut(&mut self, player_id: u32) -> Option<&mut MatchPlayer> {
        self.players.get_mut(&player_id)
    }

    pub fn raw_players(&self) -> Vec<&MatchPlayer> {
        self.players.values().collect()
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

pub struct PlayMatchStateResult {
    pub additional_time: u64,
}

impl PlayMatchStateResult {
    pub fn new() -> Self {
        PlayMatchStateResult { additional_time: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let match_time = MatchTime::new();
        assert_eq!(match_time.time, 0);
    }

    #[test]
    fn test_increment() {
        let mut match_time = MatchTime::new();

        let incremented_time = match_time.increment(10);
        assert_eq!(match_time.time, 10);
        assert_eq!(incremented_time, 10);

        let incremented_time_again = match_time.increment(5);
        assert_eq!(match_time.time, 15);
        assert_eq!(incremented_time_again, 15);
    }
}