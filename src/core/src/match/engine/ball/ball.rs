use crate::r#match::MatchContext;
use nalgebra::Vector3;
use rand_distr::num_traits::Pow;
use crate::r#match::ball::events::BallUpdateEvent;

pub struct Ball {
    pub start_position: Vector3<f32>,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub ball_position: BallPosition,
    pub center_field_position: f32,
    pub height: f32,

    pub last_owner: Option<u32>,
    pub owned: bool
}

impl Ball {
    pub fn with_coord(x: f32, y: f32) -> Self {
        Ball {
            position: Vector3::new(x, y, 0.0),
            start_position: Vector3::new(x, y, 0.0),
            velocity: Vector3::zeros(),
            ball_position: BallPosition::Home,
            center_field_position: x, // initial ball position = center field
            height: 0.0,
            last_owner: None,
            owned: false
        }
    }

    pub fn update(&mut self, context: &MatchContext) -> Vec<BallUpdateEvent> {
        let mut result = Vec::with_capacity(10);

        self.update_velocity(&mut result);
        self.move_to(&mut result);
        self.check_goal(context, &mut result);
        self.check_boundary_collision(&mut result, context);

        result
    }

    fn check_boundary_collision(
        &mut self,
        _result: &mut Vec<BallUpdateEvent>,
        context: &MatchContext,
    ) {
        let field_width = context.field_size.width as f32 + 1.0;
        let field_height = context.field_size.height as f32 + 1.0;

        // Check if ball hits the boundary and reverse its velocity if it does
        if self.position.x <= 0.0 || self.position.x >= field_width {
            self.velocity.x = -self.velocity.x;
        }

        if self.position.y <= 0.0 || self.position.y >= field_height {
            self.velocity.y =- self.velocity.y;
        }
    }

    fn check_goal(&mut self, context: &MatchContext, result: &mut Vec<BallUpdateEvent>) {
        if let Some(goal_side) = context.goal_positions.is_goal(self.position) {
            result.push(BallUpdateEvent::Goal(goal_side, self.last_owner));
            self.reset();
        }
    }

    fn update_velocity(&mut self, _result: &mut Vec<BallUpdateEvent>) {
        let gravity = Vector3::new(0.0, 0.0, -9.81);

        const FRICTION_COEFFICIENT: f32 = 1.5;
        const BALL_MASS: f32 = 0.43;
        const STOPPING_THRESHOLD: f32 = 0.01;

        let velocity_norm = self.velocity.norm();
        let friction = if velocity_norm > 0.0 {
            -self.velocity.normalize() * FRICTION_COEFFICIENT
        } else {
            Vector3::zeros()
        };

        let total_force = gravity * BALL_MASS - friction;
        let acceleration = total_force / BALL_MASS;

        const TIME_STEP: f32 = 0.01;

        self.velocity += acceleration * TIME_STEP;

        if self.velocity.norm() < STOPPING_THRESHOLD {
            self.velocity = Vector3::zeros();
        }
    }

    fn move_to(&mut self, result: &mut Vec<BallUpdateEvent>) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn position(&self) -> BallPosition {
        if self.position.x <= self.center_field_position {
            BallPosition::Home
        } else {
            BallPosition::Away
        }
    }

    pub fn move_towards_player(&mut self, player_pos: &Vector3<f32>) {
        let position_diff = *player_pos - self.position;

        let distance = (position_diff.x.pow(2.0) + position_diff.y.pow(2.0)).sqrt();

        self.position.x += (position_diff.x / distance) * self.velocity.x;
        self.position.y += (position_diff.y / distance) * self.velocity.y;
    }

    pub fn reset(&mut self) {
        self.position.x = self.start_position.x;
        self.position.y = self.start_position.y;

        self.velocity = Vector3::zeros();
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BallPosition {
    Home,
    Away,
}
