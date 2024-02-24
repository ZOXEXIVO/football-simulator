use crate::common::NeuralNetwork;
use crate::PlayerPositionType;
use std::collections::HashMap;

use crate::r#match::position::{PlayerFieldPosition, VectorExtensions};
use crate::r#match::strategies::common::MatchPlayerLogic;
use crate::r#match::{BallContext, GameTickContext, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerState, PlayerTickContext, PlayerUpdateEvent, StateChangeResult};

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
        if player_tick_context.ball_context.ball_is_on_player_home_side
            && Self::is_dangerous(player, &tick_context.objects_positions)
        {
            return StateChangeResult::with_state(PlayerState::Running);
        }

        // if let Some(nearest_opponent) =
        //     MatchPlayerLogic::closest_opponent(player, &tick_context.objects_positions.players_positions)
        // {
        //     let distance_to_opponent = nearest_opponent.position.distance_to(&player.position);
        //     if distance_to_opponent < 50.0 {
        //         return StateChangeResult::with_state(PlayerState::Running);
        //     }
        // }

        if player_tick_context.ball_context.ball_distance > 100.0 {
            return StateChangeResult::none();
        }

        if player_tick_context.ball_context.ball_distance < 20.0 {
            return StateChangeResult::with_state(PlayerState::Tackling);
        }

        if Self::should_sweep(player, &tick_context.objects_positions, &player_tick_context.ball_context) {
            // Perform sweeping action
            // Add sweeping logic here...
        }

        // 2. Organize defense based on player positions
        Self::organize_defense(player, &tick_context.objects_positions);

        // 3. Communicate with defenders
        Self::communicate_with_defenders(player, context, &tick_context.objects_positions, &player_tick_context.ball_context);

        // 4. Make critical decisions
        Self::make_critical_decisions(player, context, &tick_context.objects_positions, &player_tick_context.ball_context);

        return if player_tick_context.ball_context.is_ball_heading_towards_goal {
            if Self::should_rush_out(player, &tick_context.objects_positions, &player_tick_context.ball_context) {
                StateChangeResult::with_state(PlayerState::Running)
            } else {
                StateChangeResult::with_state(PlayerState::Walking)
            }
        } else {
            StateChangeResult::none()
        };
    }

    fn should_rush_out(
        player: &MatchPlayer,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: &BallContext,
    ) -> bool {
        objects_positions.ball_position.y.abs() < 10.0
            && objects_positions
            .ball_position
            .distance_to(&player.position)
            < 50.0
    }

    fn is_dangerous(player: &MatchPlayer, objects_positions: &MatchObjectsPositions) -> bool {
        let (nearest_home_count, nearest_away_count) = objects_positions
            .players_positions
            .iter()
            .filter(|p| p.position.distance_to(&player.position) < 100.0)
            .map(|p| p.is_home)
            .partition::<Vec<_>, _>(|&is_home| is_home);

        let nearest_home_count = nearest_home_count.len() as f32;
        let nearest_away_count = nearest_away_count.len() as f32;

        (nearest_home_count + 1.0) / (nearest_away_count + 1.0) < 0.5
    }

    fn should_sweep(
        player: &MatchPlayer,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: &BallContext,
    ) -> bool {
        // Check if the ball is behind the defensive line
        let ball_behind_defense = objects_positions.ball_position.y.abs() > 30.0; // Adjust the threshold as needed

        if !ball_behind_defense {
            return false; // Ball is not behind the defense, no need to sweep
        }

        // Check if the goalkeeper is closest to the ball
        let goalkeeper_closest_to_ball = objects_positions
            .players_positions
            .iter()
            .filter(|p| p.is_home == player.is_home) // Consider only players from the same team
            .all(|p| {
                // Check if any player (except the goalkeeper) is closer to the ball
                p.position.distance_to(&objects_positions.ball_position)
                    >= player
                    .position
                    .distance_to(&objects_positions.ball_position)
            });

        // Return true if the goalkeeper is closest to the ball and the ball is behind the defense
        goalkeeper_closest_to_ball
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
                    Self::adjust_triangle_positions(players);
                }
                4 => {
                    // Four defenders: Two center-backs, Left-back, Right-back
                    // Coordinate movements to cover wide areas and central zones
                    Self::adjust_square_positions(players);
                }
                _ => {
                    // Handle other group sizes if needed
                    // Implement custom logic based on the specific group size
                }
            }
        }
    }

    fn adjust_triangle_positions(players: &[&PlayerFieldPosition]) {
        // Example: Maintain triangular shape with center-backs closer to the goal
        // if let Some(center_backs) = players
        //     .iter()
        //     .filter(|p| Self::is_center_back(**p))
        //     .collect::<Vec<_>>()
        //     .as_slice()
        // {
        //     if let Some(left_back) = players.iter().find(|p| Self::is_left_back(**p)) {
        //         // Adjust left-back position
        //         // Example: Shift left-back slightly to cover wide areas
        //         // Adjust position based on specific defensive strategy
        //     }
        //     if let Some(right_back) = players.iter().find(|p| Self::is_right_back(**p)) {
        //         // Adjust right-back position
        //         // Example: Shift right-back slightly to cover wide areas
        //         // Adjust position based on specific defensive strategy
        //     }
        // }
    }

    // Helper functions to identify specific player positions
    fn is_center_back(player: &PlayerFieldPosition) -> bool {
        // Implement logic to determine if a player is a center-back
        // Add your logic here...
        false
    }

    fn is_left_back(player: &PlayerFieldPosition) -> bool {
        // Implement logic to determine if a player is a left-back
        // Add your logic here...
        false
    }

    fn is_right_back(player: &PlayerFieldPosition) -> bool {
        // Implement logic to determine if a player is a right-back
        // Add your logic here...
        false
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

    fn communicate_with_defenders(
        player: &MatchPlayer,
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: &BallContext,
    ) {
        // Logic to communicate with defenders based on game situations
        // Add communication logic here...
    }

    fn make_critical_decisions(
        player: &MatchPlayer,
        context: &mut MatchContext,
        objects_positions: &MatchObjectsPositions,
        ball_metadata: &BallContext,
    ) {
        // Logic to make critical decisions (e.g., leaving goal line, claiming crosses)
        // Add decision-making logic here...
    }
}

const NEURAL_NETWORK_DATA: &'static str = include_str!("nn_standing_data.json");

#[derive(Debug)]
pub struct PlayerStandingStateNetLoader;

impl PlayerStandingStateNetLoader {
    pub fn load() -> NeuralNetwork {
        NeuralNetwork::load_json(NEURAL_NETWORK_DATA)
    }
}
