use crate::common::NeuralNetwork;
use crate::{PlayerFieldPositionGroup, PlayerPositionType};
use nalgebra::Vector3;
use std::collections::HashMap;

use crate::r#match::position::PlayerFieldPosition;

use crate::r#match::{
    BallContext, GameSituationInput, GameTickContext, MatchContext, MatchObjectsPositions,
    MatchPlayer, PlayerState, PlayerTickContext, PlayerUpdateEvent, StateChangeResult,
    SteeringBehavior,
};

lazy_static! {
    static ref PLAYER_STANDING_STATE_NETWORK: NeuralNetwork = PlayerStandingStateNetLoader::load();
}

pub struct GoalkeeperStandingState {}

impl GoalkeeperStandingState {
    pub fn process(
        player: &MatchPlayer,
        context: &mut MatchContext,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
        in_state_time: u64,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        // Analyze the game situation using the neural network
        let nn_results = PLAYER_STANDING_STATE_NETWORK
            .run(&GameSituationInput::from_contexts(context, player, tick_context).to_input());

        // Make decisions based on the analysis
        if let Some(decision) = Self::analyze(player, nn_results, tick_context, player_tick_context)
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

    fn should_rush_out(ball_context: &BallContext) -> bool {
        ball_context.ball_distance < 50.0
    }

    fn is_big_opponents_concentration(
        player: &MatchPlayer,
        objects_positions: &MatchObjectsPositions,
    ) -> bool {
        let max_distance = 150.0;

        let (nearest_teammates_count, nearest_opponents_count) = objects_positions
            .player_distances
            .players_within_distance_count(player, max_distance);

        ((nearest_teammates_count as f32) + 1.0) / ((nearest_opponents_count as f32) + 1.0) < 1.0
    }

    fn should_sweep(player: &MatchPlayer, objects_positions: &MatchObjectsPositions) -> bool {
        let ball_behind_defense = objects_positions.ball_position.y.abs() > 30.0;

        if !ball_behind_defense {
            return false;
        }

        let (teammates, opponents) = objects_positions
            .player_distances
            .players_within_distance(player, 30.0);

        let teammate_closer_to_ball = teammates.iter().any(|(_, distance)| *distance < 5.0);

        !teammate_closer_to_ball
    }

    fn organize_defense(player: &MatchPlayer, objects_positions: &MatchObjectsPositions) {
        // Retrieve defensive players from the same team
        let defensive_players = objects_positions
            .players_positions
            .iter()
            .filter(|p| p.is_home == player.is_home && p.player_id != player.player_id);

        // Group defensive players based on their positions (e.g., left-back, center-back, right-back)
        let mut position_groups: HashMap<PlayerPositionType, Vec<&PlayerFieldPosition>> =
            HashMap::new();

        for defensive_player in defensive_players {
            position_groups
                .entry(player.tactics_position)
                .or_insert_with(Vec::new)
                .push(defensive_player);
        }

        // Analyze defensive positions and adjust if needed
        for (_, players) in &position_groups {
            // Example: Ensure proper spacing and cover key areas of the field
            match players.len() {
                3 => {
                    // Three defenders: Left-back, Center-back, Right-back
                    // Maintain triangular shape, cover central areas, etc.
                    //Self::adjust_triangle_positions(players);
                }
                4 => {
                    // Four defenders: Two center-backs, Left-back, Right-back
                    // Coordinate movements to cover wide areas and central zones
                    //Self::adjust_square_positions(players);
                }
                _ => {
                    // Handle other group sizes if needed
                    // Implement custom logic based on the specific group size
                }
            }
        }
    }

    fn adjust_square_positions(players: &[&PlayerFieldPosition]) {
        // Example: Coordinate movements to cover wide areas and central zones
        // Adjust positions to maintain balance and cover opposing players
        // Add your logic here...
    }

    fn determine_position_group(player: &PlayerFieldPosition) -> PlayerPositionType {
        // Determine the position group based on the player's x-coordinate
        if player.position.x < -30.0 {
            // Player is on the left side
            PlayerPositionType::DefenderLeft
        } else if player.position.x > 30.0 {
            // Player is on the right side
            PlayerPositionType::DefenderRight
        } else {
            // Player is in the center
            PlayerPositionType::DefenderCenter
        }
    }

    fn analyze(
        player: &MatchPlayer,
        analysis: Vec<f64>,
        tick_context: &GameTickContext,
        player_tick_context: PlayerTickContext,
    ) -> Option<GoalkeeperDecision> {
        if Self::is_big_opponents_concentration(player, &tick_context.objects_positions) {
            return Some(GoalkeeperDecision::Run);
        }

        if player_tick_context.ball_context.ball_distance < 100.0 {
            if let Some((_, opponent_distance)) = tick_context
                .objects_positions
                .player_distances
                .find_closest_opponent(player)
            {
                if opponent_distance < 50.0 {
                    return Some(GoalkeeperDecision::Run);
                }
            }
        }

        if Self::should_sweep(player, &tick_context.objects_positions) {
            // Perform sweeping action
            // Add sweeping logic here...
        }

        // Use the neural network analysis and game context to determine the appropriate decision
        // Example implementation:
        // if analysis.is_goal_scoring_opportunity {
        //     GoalkeeperDecision::RushOut
        // } else if analysis.is_defensive_reorganization_needed {
        //     GoalkeeperDecision::OrganizeDefense
        // } else if player_tick_context.ball_context.ball_distance < 20.0 {
        //     GoalkeeperDecision::Tackle
        // } else {
        //     GoalkeeperDecision::PositionYourself
        // }

        None
    }

    fn execute_decision(
        player: &MatchPlayer,
        context: &mut MatchContext,
        match_objects_positions: &MatchObjectsPositions,
        decision: GoalkeeperDecision,
        result: &mut Vec<PlayerUpdateEvent>,
    ) -> StateChangeResult {
        match decision {
            GoalkeeperDecision::RushOut => {
                // Rush out of the goal to intercept the ball
                let velocity = SteeringBehavior::Arrive {
                    target: match_objects_positions.ball_position,
                    slowing_distance: 5.0,
                }
                .calculate(player)
                .velocity;

                return StateChangeResult::with(PlayerState::Running, velocity);
            }
            GoalkeeperDecision::OrganizeDefense => {
                // Self::organize_defensive_line(player, context);
                return StateChangeResult::with_state(PlayerState::Walking);
            }
            GoalkeeperDecision::Tackle => {
                return StateChangeResult::with_state(PlayerState::Tackling);
            }
            GoalkeeperDecision::PositionYourself => {
                // Position yourself appropriately based on the ball's position
                let target_position = match_objects_positions.ball_position;

                let velocity = SteeringBehavior::Arrive {
                    target: target_position,
                    slowing_distance: 5.0,
                }
                .calculate(player)
                .velocity;

                return StateChangeResult::with(PlayerState::Walking, velocity);
            }
            GoalkeeperDecision::Run => {
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

    fn calculate_goalkeeper_position(
        player: &MatchPlayer,
        objects_positions: &MatchObjectsPositions,
        ball_context: &BallContext,
    ) -> Vector3<f32> {
        // Calculate the ideal position for the goalkeeper based on the ball's position and trajectory
        let mut ideal_position = player.start_position;

        // Adjust the position based on the ball's trajectory
        if ball_context.is_heading_towards_goal {
            // If the ball is heading towards the goal, position the goalkeeper closer to the goal line
            ideal_position.y = player.start_position.y.max(-40.0);
        } else {
            // If the ball is not heading towards the goal, position the goalkeeper closer to the center
            ideal_position.y = player.start_position.y.min(0.0);
        }

        // Adjust the position based on the ball's position
        let ball_position = objects_positions.ball_position;
        if ball_position.x < -20.0 {
            // If the ball is on the left side, shift the goalkeeper towards the left post
            ideal_position.x = player.start_position.x.max(-20.0);
        } else if ball_position.x > 20.0 {
            // If the ball is on the right side, shift the goalkeeper towards the right post
            ideal_position.x = player.start_position.x.min(20.0);
        } else {
            // If the ball is in the center, keep the goalkeeper in the center
            ideal_position.x = player.start_position.x;
        }

        // Adjust the position based on the positions of other players
        let mut closest_opponent_distance = std::f32::MAX;
        let mut closest_opponent_position = Vector3::<f32>::zeros();

        for opponent in objects_positions
            .players_positions
            .iter()
            .filter(|p| p.is_home != player.is_home)
        {
            let distance = (opponent.position - ideal_position).magnitude();
            if distance < closest_opponent_distance {
                closest_opponent_distance = distance;
                closest_opponent_position = opponent.position;
            }
        }

        if closest_opponent_distance < 15.0 {
            // If an opponent is very close, shift the goalkeeper towards the opponent
            ideal_position = (ideal_position + closest_opponent_position) / 2.0;
        }

        ideal_position
    }

    fn communicate_with_defenders(
        player: &MatchPlayer,
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: &BallContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) {
        // Get the list of defenders
        let defenders: Vec<&MatchPlayer> = objects_positions
            .players_positions
            .iter()
            .filter(|p| {
                p.is_home == player.is_home
                    && player.tactics_position.position_group()
                        == PlayerFieldPositionGroup::Defender
            })
            .map(|p| context.players.get(p.player_id))
            .flatten()
            .collect();

        for defender in defenders {
            if Self::is_opponent_near_goal(defender, objects_positions) {
                // Communicate to the defender to mark the opponent near the goal
                result.push(PlayerUpdateEvent::CommunicateMessage(
                    defender.player_id,
                    "Mark the opponent near the goal!",
                ));
            } else {
                // Communicate to the defender to maintain defensive position
                result.push(PlayerUpdateEvent::CommunicateMessage(
                    defender.player_id,
                    "Maintain defensive position!",
                ));
            }
        }
    }

    fn is_opponent_near_goal(
        defender: &MatchPlayer,
        objects_positions: &MatchObjectsPositions,
    ) -> bool {
        let max_distance_to_goal = 20.0;
        objects_positions.players_positions.iter().any(|p| {
            p.is_home != defender.is_home && p.position.y.abs() > (50.0 - max_distance_to_goal)
        })
    }

    fn make_critical_decisions(
        player: &MatchPlayer,
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: &BallContext,
        result: &mut Vec<PlayerUpdateEvent>,
    ) {
        if ball_metadata.ball_distance > 10.0 {
            return;
        }

        if ball_metadata.is_heading_towards_goal {
            result.push(PlayerUpdateEvent::RushOut(player.player_id));
        } else {
            result.push(PlayerUpdateEvent::StayInGoal(player.player_id));
        }
    }
}

enum GoalkeeperDecision {
    Run,
    RushOut,
    OrganizeDefense,
    Tackle,
    PositionYourself,
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_standing_data.json");

#[derive(Debug)]
pub struct PlayerStandingStateNetLoader;

impl PlayerStandingStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}

// To improve the goalkeeper behavior in your football simulator, you can make the following changes and additions to the code:
//
// Positioning and Decision Making:
//
// Implement a more sophisticated positioning system that takes into account the position of the ball, opposing players, and the goalkeeper's own team's defensive line.
// Use a decision-making algorithm or a set of rules to determine when the goalkeeper should stay on the goal line, come off the line to intercept a pass or shot, or rush out to challenge an opposing player.
// Consider factors such as the distance and angle of the ball from the goal, the speed and direction of the ball, and the presence of opposing players in dangerous positions.
// Communication and Organization:
//
// Enhance the goalkeeper's communication with the defensive players to organize the defense effectively.
// Implement a system where the goalkeeper can issue commands or signals to the defenders to mark opponents, maintain defensive shape, or cover specific areas of the field.
// Use the communicate_with_defenders function to send appropriate messages or instructions based on the game situation.
// Sweeper Keeper Role:
//
// Improve the should_sweep function to make more intelligent decisions about when the goalkeeper should act as a sweeper.
// Consider factors such as the position of the ball, the presence of opposing players, and the availability of defensive cover before deciding to sweep.
// Implement the actual sweeping action, where the goalkeeper moves out of the goal area to clear the ball or intercept a pass.
// Shot Stopping and Distribution:
//
// Enhance the goalkeeper's ability to react to shots on goal by considering factors such as the speed, placement, and trajectory of the shot.
// Implement a system to determine the goalkeeper's diving or jumping direction and timing based on the characteristics of the shot.
// Improve the goalkeeper's distribution skills, such as accurately passing the ball to teammates or clearing it to safety when under pressure.
// Handling Crosses and High Balls:
//
// Develop a system for the goalkeeper to assess the flight and trajectory of crosses and high balls into the penalty area.
// Implement decision-making logic to determine whether the goalkeeper should come out to claim the ball or stay on the line.
// Consider factors such as the position of opposing players, the presence of defensive cover, and the goalkeeper's own physical attributes (e.g., height, jumping ability).
// Learning and Adaptation:
//
// Utilize machine learning techniques to train the goalkeeper's decision-making model based on historical data or simulated scenarios.
// Continuously update and refine the model based on the goalkeeper's performance and the outcomes of different game situations.
// Adapt the goalkeeper's behavior and strategies based on the specific characteristics and tendencies of the opposing team and individual players.
// Integration with Team Tactics:
//
// Ensure that the goalkeeper's actions and decisions align with the overall team tactics and strategy.
// Collaborate with the defensive players to maintain a cohesive defensive structure and minimize gaps or weaknesses.
// Adapt the goalkeeper's positioning and decision-making based on the team's formation, pressing intensity, and defensive approach.
// Remember to thoroughly test and fine-tune the goalkeeper's behavior through extensive simulations and iterations to achieve realistic and effective performance.
//
// Additionally, it's important to strike a balance between realistic behavior and computational efficiency, as simulating complex decision-making processes for the goalkeeper can be resource-intensive. Optimize the code and algorithms where possible to ensure smooth performance of the football simulator.
