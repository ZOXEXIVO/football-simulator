use crate::r#match::defenders::states::{DefenderState, DefenderStrategies};
use crate::r#match::forwarders::states::ForwardStrategies;
use crate::r#match::goalkeepers::states::state::GoalkeeperStrategies;
use crate::r#match::midfielders::states::MidfielderStrategies;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;
use crate::r#match::{CommonInjuredState, CommonReturningState, CommonRunningState, CommonShootingState, CommonTacklingState, GameTickContext, MatchContext, MatchPlayer, PlayerDistanceFromStartPosition, PlayerTickContext};
use crate::PlayerFieldPositionGroup;
use nalgebra::Vector3;
use crate::r#match::player::state::PlayerState::Defender;

pub trait StateProcessingHandler {
    // Try fast processing
    fn try_fast(&self, context: &mut StateProcessingContext) -> Option<StateChangeResult>;
    // Try slow processing with neural network
    fn process_slow(&self, context: &mut StateProcessingContext) -> StateChangeResult;
    // Calculate velocity
    fn velocity(&self) -> Vector3<f32>;
}

impl PlayerFieldPositionGroup {
    pub fn process(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: &PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let player_state = player.state;

        let mut state_processor = StateProcessor::new(
            in_state_time,
            player,
            context,
            tick_context,
            player_context,
            result,
        );

        match player_state {
            // Common states
            PlayerState::Running => state_processor.process(CommonRunningState::default()),
            PlayerState::Tackling => state_processor.process(CommonTacklingState::default()),
            PlayerState::Shooting => state_processor.process(CommonShootingState::default()),
            PlayerState::Returning => state_processor.process(CommonReturningState::default()),
            PlayerState::Injured => state_processor.process(CommonInjuredState::default()),
            // // Specific states
            PlayerState::Goalkeeper(state) => GoalkeeperStrategies::process(state, state_processor),
            PlayerState::Defender(state) => DefenderStrategies::process(state, state_processor),
            PlayerState::Midfielder(state) => MidfielderStrategies::process(state, state_processor),
            PlayerState::Forward(state) => ForwardStrategies::process(state, state_processor),
        }
    }
}

pub struct StateProcessor<'p> {
    in_state_time: u64,
    player: &'p MatchPlayer,
    context: &'p MatchContext,
    tick_context: &'p GameTickContext,
    player_context: &'p PlayerTickContext,
    result: &'p mut Vec<PlayerUpdateEvent>,
}

impl<'p> StateProcessor<'p> {
    pub fn new(
        in_state_time: u64,
        player: &'p MatchPlayer,
        context: &'p MatchContext,
        tick_context: &'p GameTickContext,
        player_context: &'p PlayerTickContext,
        result: &'p mut Vec<PlayerUpdateEvent>,
    ) -> Self {
        StateProcessor {
            in_state_time,
            player,
            context,
            tick_context,
            player_context,
            result,
        }
    }

    pub fn process<H: StateProcessingHandler>(self, handler: H) -> StateChangeResult {
        let mut processing_ctx = self.into_ctx();

        if let Some(fast_result) = handler.try_fast(&mut processing_ctx) {
            return fast_result;
        }

        let mut result = handler.process_slow(&mut processing_ctx);

        if processing_ctx.in_state_time % 3 == 0 {
            result.velocity = Some(handler.velocity());
        }

        result
    }

    pub fn into_ctx(self) -> StateProcessingContext<'p> {
        StateProcessingContext::from(self)
    }
}

pub struct StateProcessingContext<'sp> {
    pub in_state_time: u64,
    pub player: &'sp MatchPlayer,
    pub context: &'sp MatchContext,
    pub tick_context: &'sp GameTickContext,
    pub player_context: &'sp PlayerTickContext,
    pub result: &'sp mut Vec<PlayerUpdateEvent>,
}

impl<'sp> StateProcessingContext<'sp> {
    pub fn position_to_distance(&self) -> PlayerDistanceFromStartPosition {
        self.player_context
            .player
            .distance_to_start_position
    }

    pub fn ball_on_own_side(&self) -> bool {
        self.player_context.ball.on_own_side
    }

    pub fn ball_distance(&self) -> f32 {
        self.player_context.ball.ball_distance
    }

    pub fn ball_towards_player(&self) -> bool {
        self.player_context.ball.is_heading_towards_player
    }

    pub fn player_distances(&self) -> (usize, usize) {
        self
            .tick_context
            .objects_positions
            .player_distances
            .players_within_distance_count(self.player, 10.0)
    }
}

impl<'sp> From<StateProcessor<'sp>> for StateProcessingContext<'sp> {
    fn from(value: StateProcessor<'sp>) -> Self {
        StateProcessingContext {
            in_state_time: value.in_state_time,
            player: value.player,
            context: value.context,
            tick_context: value.tick_context,
            player_context: value.player_context,
            result: value.result,
        }
    }
}

pub struct StateChangeResult {
    pub state: Option<PlayerState>,
    pub velocity: Option<Vector3<f32>>,
}

impl StateChangeResult {
    pub fn with(state: PlayerState, velocity: Vector3<f32>) -> Self {
        StateChangeResult {
            state: Some(state),
            velocity: Some(velocity),
        }
    }

    pub const fn none() -> Self {
        StateChangeResult {
            state: None,
            velocity: None,
        }
    }

    pub fn with_state(state: PlayerState) -> Self {
        StateChangeResult {
            state: Some(state),
            velocity: None,
        }
    }

    pub fn with_defender_state(state: DefenderState) -> Self {
        StateChangeResult {
            state: Some(Defender(state)),
            velocity: None,
        }
    }

    pub fn with_velocity(velocity: Vector3<f32>) -> Self {
        StateChangeResult {
            state: None,
            velocity: Some(velocity),
        }
    }
}
