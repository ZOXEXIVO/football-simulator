use crate::r#match::position::FieldPosition;
use crate::r#match::FootballMatchDetails;
use nalgebra::Vector2;
use rand::{thread_rng, Rng};
use rand_distr::num_traits::Pow;

pub struct Ball {
    pub start_position: FieldPosition,
    pub position: FieldPosition,
    pub velocity: Vector2<f32>,
    pub direction: FieldPosition,
    pub owner: Option<BallOwner>,
}

impl Ball {
    pub fn with_coord(x: f32, y: f32) -> Self {
        Ball {
            position: FieldPosition { x, y },
            start_position: FieldPosition { x, y },
            velocity: Vector2::new(0.0, 0.0),
            direction: FieldPosition { x: 0.0, y: 0.0 },
            owner: None,
        }
    }

    pub fn update(&mut self) -> Vec<BallUpdateEvent> {
        let mut result = Vec::new();

        self.update_velocity(&mut result);
        self.move_to(&mut result);
        self.check_boundary_collision(&mut result);
        self.check_goal(&mut result);

        result
    }

    pub fn handle_events(events: &Vec<BallUpdateEvent>, match_details: &mut FootballMatchDetails) {
        for event in events {
            match event {
                BallUpdateEvent::AwayGoal => {
                    match_details.score.away += 1;
                }
                BallUpdateEvent::HomeGoal => {
                    match_details.score.home += 1;
                }
            }
        }
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
                    result.push(BallUpdateEvent::AwayGoal);
                } else {
                    result.push(BallUpdateEvent::HomeGoal);
                }

                self.reset();
            }
        }
    }

    fn update_velocity(&mut self, result: &mut Vec<BallUpdateEvent>) {
        let mut rng = thread_rng();

        let random_x_val: f32 = rng.gen_range(-0.1..0.1);
        let random_y_val: f32 = rng.gen_range(-0.1..0.1);

        self.velocity = Vector2::new(random_x_val, random_y_val);
    }

    fn move_to(&mut self, result: &mut Vec<BallUpdateEvent>) {
        self.position.x += self.velocity.x;
        if self.position.x > 140.0 {
            self.position.x = 140.0;
        }

        self.position.y += self.velocity.y;
        if self.position.y > 90.0 {
            self.position.y = 90.0;
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
    HomeGoal,
    AwayGoal,
}

pub enum BallOwner {
    Home,
    Away,
}
