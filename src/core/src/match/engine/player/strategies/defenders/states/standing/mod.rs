use crate::common::NeuralNetwork;

use crate::r#match::decision::DefenderDecision;
use crate::r#match::strategies::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{
    GameSituationInput, GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer,
    PlayerState, PlayerTickContext, PlayerUpdateEvent, StateChangeResult, SteeringBehavior,
};

lazy_static! {
    static ref DEFENDER_STANDING_STATE_NETWORK: NeuralNetwork =
        DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json"));
}

pub struct DefenderStandingState {}

impl DefenderStandingState {
    pub fn process(
        player: &MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        // Analyze the game situation using the neural network
        let nn_input = GameSituationInput::from_contexts(context, player, tick_context).to_input();
        let nn_result = DEFENDER_STANDING_STATE_NETWORK.run(&nn_input);

        // Make decisions based on the analysis
        if let Some(decision) =
            Self::analyze_results(nn_result, player, tick_context, player_tick_context)
        {
            return Self::execute_decision(
                player,
                context,
                &tick_context.objects_positions,
                decision,
                result,
            );
        }

        StateChangeResult::none()
    }

    fn analyze_results(
        nn_analysis: Vec<f64>,
        player: &MatchPlayer,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
    ) -> Option<DefenderDecision> {
        if player_tick_context.ball_context.ball_distance < 100.0 {
            if let Some((_, opponent_distance)) = tick_context
                .objects_positions
                .player_distances
                .find_closest_opponent(player)
            {
                if opponent_distance < 50.0 {
                    return Some(DefenderDecision::Run);
                }
            }
        }

        None
    }

    fn execute_decision(
        player: &MatchPlayer,
        context: &mut MatchContext,
        match_objects_positions: &MatchObjectsPositions,
        decision: DefenderDecision,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        match decision {
            DefenderDecision::Run => {
                {
                    // go to own goals
                    let velocity = SteeringBehavior::Arrive {
                        target: player.start_position,
                        slowing_distance: 5.0,
                    }
                    .calculate(player)
                    .velocity;

                    return StateChangeResult::with(PlayerState::Running, velocity);
                }
            }
        }
    }
}
