use crate::common::NeuralNetwork;
use nalgebra::Vector3;
use std::sync::LazyLock;

use crate::r#match::decision::DefenderDecision;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::common::loader::DefaultNeuralNetworkLoader;
use crate::r#match::{
    GameFieldContextInput, GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer,
    PlayerState, PlayerTickContext, StateChangeResult, SteeringBehavior,
};

static DEFENDER_STANDING_STATE_NETWORK: LazyLock<NeuralNetwork> =
    LazyLock::new(|| DefaultNeuralNetworkLoader::load(include_str!("nn_standing_data.json")));

pub struct DefenderStandingState {}

impl DefenderStandingState {
    pub fn process(
        in_state_time: u64,
        player: &mut MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_context: PlayerTickContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        let test = SteeringBehavior::Arrive {
            target: tick_context.objects_positions.ball_position,
            slowing_distance: 10.0,
        }
        .calculate(player)
        .velocity;

        return StateChangeResult::with_velocity(test);
        //
        // // Analyze the game situation using the neural network
        // let nn_input = GameFieldContextInput::from_contexts(context, player, tick_context).to_input();
        // let nn_result = DEFENDER_STANDING_STATE_NETWORK.run(&nn_input);
        //
        // // Make decisions based on the analysis
        // if let Some(decision) =
        //     Self::analyze_results(nn_result, player, tick_context, player_tick_context)
        // {
        //     return Self::execute_decision(
        //         player,
        //         context,
        //         &tick_context.objects_positions,
        //         decision,
        //         result,
        //     );
        // }

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

        // If no immediate threat, analyze the neural network output
        if nn_analysis[0] > 0.7 {
            // If the neural network suggests a high probability of maintaining position,
            // stand still and wait for the next opportunity
            return Some(DefenderDecision::StandStill);
        } else if nn_analysis[1] > 0.6 {
            // If the neural network suggests a moderate probability of needing to move,
            // adjust position to a more strategic location
            return Some(DefenderDecision::AdjustPosition);
        } else if nn_analysis[2] > 0.5 {
            // If the neural network suggests a moderate probability of needing to run,
            // run towards the goal to defend
            return Some(DefenderDecision::RunTowardsGoal);
        } else if nn_analysis[3] > 0.4 {
            // If the neural network suggests a moderate probability of needing to mark an opponent,
            // mark the closest opponent
            return Some(DefenderDecision::MarkOpponent);
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
            DefenderDecision::RunTowardsBall => {
                let ball_position = match_objects_positions.ball_position;
                let velocity = SteeringBehavior::Arrive {
                    target: ball_position,
                    slowing_distance: 10.0,
                }
                .calculate(player)
                .velocity;

                StateChangeResult::with(PlayerState::Running, velocity)
            }
            DefenderDecision::StandStill => StateChangeResult::none(),
            DefenderDecision::AdjustPosition => {
                let velocity = SteeringBehavior::Arrive {
                    target: player.start_position,
                    slowing_distance: 5.0,
                }
                .calculate(player)
                .velocity;

                StateChangeResult::with(PlayerState::Walking, velocity)
            }
            DefenderDecision::RunTowardsGoal => {
                // Run towards the goal to defend
                let goal_position = calculate_goal_position(player, context);
                let velocity = SteeringBehavior::Arrive {
                    target: goal_position,
                    slowing_distance: 10.0,
                }
                .calculate(player)
                .velocity;

                StateChangeResult::with(PlayerState::Running, velocity)
            }
            DefenderDecision::MarkOpponent => {
                // Mark the closest opponent
                let opponent_position = find_closest_opponent_position(player, context);
                let velocity = SteeringBehavior::Arrive {
                    target: opponent_position,
                    slowing_distance: 5.0,
                }
                .calculate(player)
                .velocity;

                StateChangeResult::with(PlayerState::Walking, velocity)
            }
            _ => StateChangeResult::none(),
        }
    }
}

// Helper function to calculate the goal position based on the player and game context
fn calculate_goal_position(player: &MatchPlayer, context: &MatchContext) -> Vector3<f32> {
    // Implement your goal position calculation logic here
    // This could involve determining the position of the player's own goal
    // based on the player's team and the field dimensions

    // For simplicity, this example returns the player's starting position
    player.start_position
}

// Helper function to find the position of the closest opponent
fn find_closest_opponent_position(player: &MatchPlayer, context: &MatchContext) -> Vector3<f32> {
    // Implement your logic to find the position of the closest opponent
    // This could involve iterating through the positions of all opponents
    // and finding the closest one to the player

    // For simplicity, this example returns the player's starting position
    player.start_position
}
