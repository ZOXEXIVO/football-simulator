use crate::r#match::position::FieldPosition;
use crate::r#match::{
    MatchObjectsPositions, MatchPlayer, MatchState, PlayerUpdateEvent, SteeringBehavior,
};
use nalgebra::Vector3;

pub struct GoalkeeperStrategies {}

impl GoalkeeperStrategies {
    pub fn detect_velocity(
        current_time: u64,
        player: &MatchPlayer,
        result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
        state: &MatchState,
    ) -> Vector3<f32> {
        let x_position = match player.is_home {
            true => 30.0,
            false => -30.0,
        };

        let output = SteeringBehavior::Wander {
            target: FieldPosition::new(
                player.start_position.x + x_position,
                player.start_position.y,
                0.0,
            ),
            radius: 50.0,
            jitter: 5.0,
            distance: 30.0,
            angle: 54.0,
        }
        .calculate(player);

        Vector3::new(output.velocity.x, output.velocity.y, 0.0)
    }
}
