use crate::r#match::{
    BallState, MatchObjectsPositions, MatchPlayer, MatchState, PlayerUpdateEvent, SteeringBehavior,
};
use nalgebra::Vector2;

pub struct DefenderStrategies {}

impl DefenderStrategies {
    pub fn detect_velocity(
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
        state: &MatchState,
    ) -> Vector2<f32> {
        match state.ball_state {
            Some(ball_state) => match ball_state {
                BallState::HomeSide => {
                    if player.is_home {
                        // defent when i'm defender and ball on my side of field
                        DefenderStrategies::defend(player, result, objects_positions, state)
                    } else {
                        DefenderStrategies::support(player, result, objects_positions, state)
                    }
                }
                BallState::AwaySide => {
                    if player.is_home {
                        DefenderStrategies::support(player, result, objects_positions, state)
                    } else {
                        DefenderStrategies::defend(player, result, objects_positions, state)
                    }
                }
            },
            None => Vector2::new(0.0, 0.0),
        }
    }

    fn defend(
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
        state: &MatchState,
    ) -> Vector2<f32> {
        let steering_output = SteeringBehavior::Seek {
            target: objects_positions.ball_positions,
        }
        .calculate(player);

        Vector2::new(steering_output.velocity.x, steering_output.velocity.y)
    }

    fn support(
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
        state: &MatchState,
    ) -> Vector2<f32> {
        let steering_output = SteeringBehavior::Seek {
            target: player.start_position,
        }
        .calculate(player);

        Vector2::new(steering_output.velocity.x, steering_output.velocity.y)
    }
}
