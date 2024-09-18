use crate::r#match::defenders::states::{DefenderState, DefenderStrategies};
use crate::r#match::forwarders::states::ForwardStrategies;
use crate::r#match::goalkeepers::states::state::GoalkeeperStrategies;
use crate::r#match::midfielders::states::MidfielderStrategies;
use crate::r#match::player::events::{PlayerUpdateEvent, PlayerUpdateEventCollection};
use crate::r#match::player::state::PlayerState;
use crate::r#match::player::state::PlayerState::Defender;
use crate::r#match::{
    BallOperationsImpl, CommonInjuredState, CommonReturningState, CommonRunningState,
    CommonShootingState, CommonTacklingState, GameTickContext, MatchContext, MatchPlayer,
    PlayerOperationsImpl,
};
use crate::PlayerFieldPositionGroup;
use nalgebra::Vector3;
use std::cell::RefCell;

pub trait StateProcessingHandler {
    // Try fast processing
    fn try_fast(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult>;
    // Try slow processing with neural network
    fn process_slow(&self, ctx: &StateProcessingContext) -> Option<StateChangeResult>;
    // Calculate velocity
    fn velocity(&self, ctx: &StateProcessingContext) -> Option<Vector3<f32>>;
}

impl PlayerFieldPositionGroup {
    pub fn process(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &MatchContext,
        tick_context: &GameTickContext
    ) -> StateProcessingResult {
        let player_state = player.state;

        let mut state_processor =  StateProcessor::new(in_state_time, player, context, tick_context);

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
    tick_context: &'p GameTickContext
}

impl<'p> StateProcessor<'p> {
    pub fn new(
        in_state_time: u64,
        player: &'p MatchPlayer,
        context: &'p MatchContext,
        tick_context: &'p GameTickContext
    ) -> Self {
        StateProcessor {
            in_state_time,
            player,
            context,
            tick_context
        }
    }

    pub fn process<H: StateProcessingHandler>(self, handler: H) -> StateProcessingResult {
        let mut processing_ctx = self.into_ctx();

        let mut result = StateProcessingResult::new();

        if let Some(velocity) = handler.velocity(&processing_ctx) {
            result.velocity = Some(velocity);
        }

        if let Some(fast_result) = handler.try_fast(&processing_ctx) {
            if let Some(state) = fast_result.state {
                result.state = Some(state);
                result.events = fast_result.events;

                return result;
            }

        }

        if let Some(slow_result) = handler.process_slow(&mut processing_ctx) {
            if let Some(state) = slow_result.state {
                result.state = Some(state);
                result.events = slow_result.events;

                return result;
            }
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
    pub tick_context: &'sp GameTickContext
}

impl<'sp> StateProcessingContext<'sp> {
    #[inline]
    pub fn ball(&self) -> BallOperationsImpl<'_> { BallOperationsImpl::new(self) }

    #[inline]
    pub fn player(&self) -> PlayerOperationsImpl<'_> {
        PlayerOperationsImpl::new(self)
    }
}

impl<'sp> From<StateProcessor<'sp>> for StateProcessingContext<'sp> {
    fn from(value: StateProcessor<'sp>) -> Self {
        StateProcessingContext {
            in_state_time: value.in_state_time,
            player: value.player,
            context: value.context,
            tick_context: value.tick_context,
        }
    }
}

pub struct StateProcessingResult {
    pub state: Option<PlayerState>,
    pub velocity: Option<Vector3<f32>>,
    pub events: PlayerUpdateEventCollection
}

impl StateProcessingResult {
    pub fn new() -> Self {
        StateProcessingResult {
            state: None,
            velocity: None,
            events: PlayerUpdateEventCollection::new()
        }
    }
}

pub struct StateChangeResult {
    pub state: Option<PlayerState>,
    pub velocity: Option<Vector3<f32>>,

    pub events: PlayerUpdateEventCollection
}

impl StateChangeResult {
    pub fn new() -> Self {
        StateChangeResult {
            state: None,
            velocity: None,
            events: PlayerUpdateEventCollection::new()
        }
    }

    pub fn set_state(&mut self, state: PlayerState) {
        self.state = Some(state);
    }

    pub fn set_velocity(&mut self, velocity: Vector3<f32>) {
        self.velocity = Some(velocity);
    }

    pub fn with(state: PlayerState) -> Self {
        StateChangeResult {
            state: Some(state),
            velocity: None,
            events: PlayerUpdateEventCollection::new()
        }
    }

    pub fn with_defender_state(state: DefenderState) -> Self {
        StateChangeResult {
            state: Some(Defender(state)),
            velocity: None,
            events: PlayerUpdateEventCollection::new()
        }
    }

    pub fn with_velocity(velocity: Vector3<f32>) -> Self {
        StateChangeResult {
            state: None,
            velocity: Some(velocity),
            events: PlayerUpdateEventCollection::new()
        }
    }

    pub fn with_events(events: PlayerUpdateEventCollection) -> Self {
        StateChangeResult {
            state: None,
            velocity: None,
            events
        }
    }
}
