use crate::r#match::position::FieldPosition;
use nalgebra::Vector2;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use rand_distr::num_traits::Pow;

pub struct Ball {
    pub start_position: FieldPosition,
    pub position: FieldPosition,
    pub velocity: Vector2<f32>,
    pub direction: FieldPosition,
    rnd: ThreadRng,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Ball {
            position: FieldPosition { x, y },
            start_position: FieldPosition { x, y },
            velocity: Vector2::new(0.0, 0.0),
            direction: FieldPosition { x: 0.0, y: 0.0 },
            rnd: thread_rng(),
        }
    }

    pub fn update(&mut self) -> Vec<BallUpdateEvent> {
        let mut result = Vec::new();

        self.move_to(&mut result);
        self.check_goal(&mut result);

        result
    }

    fn check_goal(&mut self, result: &mut Vec<BallUpdateEvent>) {
        // if self.position.x >= self.width as i16 {
        //     match_details.score.home += 1;
        // } else if self.position.x <= 0 {
        //     match_details.score.away += 1;
        // }
    }

    fn move_to(&mut self, result: &mut Vec<BallUpdateEvent>) {
        let speed = self.rnd.gen_range(-1..1) as f32;
        let speed2 = self.rnd.gen_range(-1..1) as f32;

        self.position.x += speed * speed2;
        self.position.y += speed * speed2;
    }

    pub fn move_towards_player(&mut self, player_pos: &FieldPosition) {
        let dx = (player_pos.x - self.position.x) as f32;
        let dy = (player_pos.y - self.position.y) as f32;

        let distance = (dx.pow(2.0) + dy.pow(2.0)).sqrt();

        self.position.x += (dx / distance) * self.velocity.x;
        self.position.y += (dy / distance) * self.velocity.y;
    }

    pub fn reset(&mut self) {
        self.position.x = self.start_position.x;
        self.position.y = self.start_position.y;
    }
}

pub enum BallUpdateEvent {
    Goal,
}
