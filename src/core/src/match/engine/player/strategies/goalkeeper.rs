use crate::r#match::position::VectorExtensions;
use crate::r#match::{GameState, MatchContext, MatchObjectsPositions, MatchPlayer, PlayerUpdateEvent, SteeringBehavior};
use nalgebra::Vector3;
use crate::FloatUtils;

pub struct GoalkeeperStrategies {}

impl GoalkeeperStrategies {
    pub fn detect_velocity(
        context: &mut MatchContext,
        player: &MatchPlayer,
        _result: &mut Vec<PlayerUpdateEvent>,
        objects_positions: &MatchObjectsPositions,
    ) -> Vector3<f32> {
        let is_ball_moving =
            objects_positions.ball_velocity.x > 0.0 && objects_positions.ball_velocity.y > 0.0;

        if !is_ball_moving {
            return Vector3::new(FloatUtils::random(-0.4, 0.3), FloatUtils::random(-0.4, 0.3), FloatUtils::random(-0.4, 0.3))
            //return Vector3::zeros();
        }

        let _ball_distance = objects_positions
            .ball_positions
            .distance_to(&player.position);

        let test = SteeringBehavior::Arrive {
            target: objects_positions.ball_positions,
            slowing_distance: 10.0,
        }
        .calculate(player)
        .velocity;

        return test;

        // if ball_distance < 300.0 {
        //     return SteeringBehavior::Arrive {
        //         target: objects_positions.ball_positions,
        //         slowing_distance: 10.0,
        //     }
        //     .calculate(player)
        //     .velocity;
        // } else {
        //     return Vector3::zeros();
        //
        //     let x_position = match player.is_home {
        //         true => 30.0,
        //         false => -30.0,
        //     };
        //
        //     let output = SteeringBehavior::Wander {
        //         target: Vector3::new(
        //             player.start_position.x + x_position,
        //             player.start_position.y,
        //             0.0,
        //         ),
        //         radius: 50.0,
        //         jitter: 5.0,
        //         distance: 30.0,
        //         angle: 54.0,
        //     }
        //     .calculate(player);
        //
        //     Vector3::new(output.velocity.x, output.velocity.y, 0.0)
        // }
    }
}
