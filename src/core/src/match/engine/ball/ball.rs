use crate::r#match::position::FieldPosition;
use crate::r#match::{BallState, GoalDetail, MatchContext, MatchState};
use nalgebra::{Vector2, Vector3};
use rand::{thread_rng, Rng};
use rand_distr::num_traits::Pow;

pub struct Ball {
    pub start_position: FieldPosition,
    pub position: FieldPosition,
    pub velocity: Vector3<f32>,
    pub owner: Option<BallOwner>,
    pub ball_position: BallPosition,
    pub center_field_position: f32,
    pub height: f32,
}

impl Ball {
    pub fn with_coord(x: f32, y: f32) -> Self {
        Ball {
            position: FieldPosition { x, y, z: 0.0 },
            start_position: FieldPosition { x, y, z: 0.0 },
            velocity: Vector3::new(0.0, 0.0, 0.0),
            owner: None,
            ball_position: BallPosition::Home,
            center_field_position: x, // initial ball position = center field
            height: 0.0,
        }
    }

    pub fn update(&mut self, state: &MatchState) -> Vec<BallUpdateEvent> {
        let mut result = Vec::with_capacity(10);

        self.update_velocity(&mut result);
        self.move_to(&mut result);
        self.check_boundary_collision(&mut result);
        self.check_goal(&mut result);

        result
    }

    pub fn handle_events(
        current_time: u64,
        events: Vec<BallUpdateEvent>,
        context: &mut MatchContext,
    ) {
        for event in events {
            match event {
                BallUpdateEvent::AwayGoal(goal_scorer, goal_assistant) => {
                    context.result.score.away += 1;
                    context.result.score.details.push(GoalDetail {
                        player_id: goal_scorer,
                        assistant: goal_assistant,
                        minute: (current_time / 1000 / 60) as u8,
                    })
                }
                BallUpdateEvent::HomeGoal(goal_scorer, goal_assistant) => {
                    context.result.score.home += 1;
                    context.result.score.details.push(GoalDetail {
                        player_id: goal_scorer,
                        assistant: goal_assistant,
                        minute: (current_time / 1000 / 60) as u8,
                    })
                }
                BallUpdateEvent::ChangeBallSide(position) => {
                    let ball_state = match position {
                        BallPosition::Home => BallState::HomeSide,
                        BallPosition::Away => BallState::AwaySide,
                    };

                    context.state.set_ball_state(ball_state)
                }
            }
        }
    }

    pub fn calculate_velocity(pass_direction: Vector2<f32>, pass_power: f32) -> Vector2<f32> {
        // The mass of a standard football is around 0.43 kg
        let ball_mass = 0.43;
        // The coefficient of friction between the ball and grass is around 0.1
        let friction_coefficient = 0.1;
        // The acceleration due to gravity is approximately 9.81 m/s^2
        let gravity = Vector2::new(0.0, -9.81);

        // Calculate the direction and magnitude of the pass velocity
        let pass_velocity = pass_direction.normalize() * pass_power;

        // Calculate the net force acting on the ball, taking into account friction and gravity
        let net_force = pass_velocity * ball_mass * -friction_coefficient + ball_mass * gravity;

        // Calculate the acceleration of the ball based on the net force
        let acceleration = net_force / ball_mass;

        // Calculate the final velocity of the ball after a certain amount of time has passed
        let time_elapsed = 0.5; // 0.5 seconds for the sake of example
        let final_velocity = pass_velocity + acceleration * time_elapsed;

        final_velocity
    }

    fn check_boundary_collision(&mut self, result: &mut Vec<BallUpdateEvent>) {
        // Check if ball hits the boundary and reverse its velocity if it does
        if self.position.x <= 0.0 || self.position.x >= 150.0 {
            self.velocity.x = -self.velocity.x;
        }

        if self.position.y <= 0.0 || self.position.y >= 100.0 {
            self.velocity.y = -self.velocity.y;
        }
    }

    fn check_goal(&mut self, result: &mut Vec<BallUpdateEvent>) {
        let goal_post_width = 6.0;
        let goal_line_x = 140.0;

        if self.position.x > goal_line_x - goal_post_width
            && self.position.x < goal_line_x + goal_post_width
        {
            let goal_line_y = 90.0 / 2.0;

            if (self.start_position.y < goal_line_y && self.position.y >= goal_line_y)
                || (self.start_position.y > goal_line_y && self.position.y <= goal_line_y)
            {
                if self.start_position.x < goal_line_x {
                    //result.push(BallUpdateEvent::AwayGoal);
                } else {
                    //result.push(BallUpdateEvent::HomeGoal);
                }

                self.reset();
            }
        }
    }

    fn update_velocity(&mut self, result: &mut Vec<BallUpdateEvent>) {
        let mut rng = thread_rng();

        let random_x_val: f32 = rng.gen_range(-1.0..1.0);
        let random_y_val: f32 = rng.gen_range(-1.0..1.0);
        let random_z_val: f32 = rng.gen_range(-1.0..1.0);

        self.velocity = Vector3::new(random_x_val, random_y_val, random_z_val);
    }

    fn move_to(&mut self, result: &mut Vec<BallUpdateEvent>) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        let position = self.position();
        if position != self.ball_position {
            result.push(BallUpdateEvent::ChangeBallSide(position))
        }
    }

    fn position(&self) -> BallPosition {
        if self.position.x <= self.center_field_position {
            BallPosition::Home
        } else {
            BallPosition::Away
        }
    }

    pub fn move_towards_player(&mut self, player_pos: &FieldPosition) {
        let position_diff = *player_pos - self.position;

        let distance = (position_diff.x.pow(2.0) + position_diff.y.pow(2.0)).sqrt();

        self.position.x += (position_diff.x / distance) * self.velocity.x;
        self.position.y += (position_diff.y / distance) * self.velocity.y;
    }

    pub fn reset(&mut self) {
        self.position.x = self.start_position.x;
        self.position.y = self.start_position.y;
    }
}

pub enum BallUpdateEvent {
    HomeGoal(u32, Option<u32>),
    AwayGoal(u32, Option<u32>),
    ChangeBallSide(BallPosition),
}

pub enum BallOwner {
    Home,
    Away,
}

#[derive(Eq, PartialEq)]
pub enum BallPosition {
    Home,
    Away,
}
