use nalgebra::Vector3;

pub struct MatchBallLogic;

impl MatchBallLogic {
    pub fn is_heading_towards_goal(ball_position: &Vector3<f32>, goal_position: &Vector3<f32>) -> bool {
        let ball_to_goal = goal_position - ball_position;

        let ball_forward = Vector3::new(1.0, 0.0, 0.0);

        let dot_product = ball_to_goal.normalize().dot(&ball_forward);

        dot_product > 0.8
    }

}
