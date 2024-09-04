use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;
use crate::r#match::{
    CommonInjuredState, CommonReturningState, CommonRunningState, CommonShootingState,
    CommonTacklingState, GameTickContext, GoalkeeperStrategies, MatchContext, MatchPlayer,
    PlayerTickContext,
};
use crate::PlayerFieldPositionGroup;
use nalgebra::Vector3;

pub trait StateProcessingHandler {
    fn try_fast(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: &PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> Option<StateChangeResult>;

    fn process_slow(
        &self,
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: &PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult;
}

pub type StateHandler = fn(
    in_state_time: u64,
    player: &mut MatchPlayer,
    context: &mut MatchContext,
    tick_context: &GameTickContext,
    player_context: PlayerTickContext,
    result: &mut Vec<PlayerUpdateEvent>,
) -> StateChangeResult;

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
            PlayerState::Goalkeeper(goalkeeper_state) => {
                GoalkeeperStrategies::process(goalkeeper_state, &mut state_processor)
            }
            // PlayerState::Defender(_) => DefenderStrategies::process,
            // PlayerState::Midfielder(_) => MidfielderStrategies::process,
            // PlayerState::Forward(_) => ForwardStrategies::process,
            _ => StateChangeResult::none(),
        }
    }
}

pub struct StateProcessor<'p> {
    in_state_time: u64,
    player: &'p mut MatchPlayer,
    context: &'p mut MatchContext,
    tick_context: &'p GameTickContext,
    player_context: &'p PlayerTickContext,
    result: &'p mut Vec<PlayerUpdateEvent>,
}

impl<'p> StateProcessor<'p> {
    pub fn new(
        in_state_time: u64,
        player: &'p mut MatchPlayer,
        context: &'p mut MatchContext,
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
    pub fn process<H: StateProcessingHandler>(&mut self, handler: H) -> StateChangeResult {
        if let Some(fast_result) = handler.try_fast(
            self.in_state_time,
            self.player,
            self.context,
            self.tick_context,
            self.player_context,
            self.result,
        ) {
            return fast_result;
        }

        handler.process_slow(
            self.in_state_time,
            self.player,
            self.context,
            self.tick_context,
            self.player_context,
            self.result,
        )
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

    pub fn none() -> Self {
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

    pub fn with_velocity(velocity: Vector3<f32>) -> Self {
        StateChangeResult {
            state: None,
            velocity: Some(velocity),
        }
    }
}
