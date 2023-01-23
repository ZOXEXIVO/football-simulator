use crate::r#match::position::FieldPosition;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

pub struct Ball {
    pub start_position: FieldPosition,
    pub position: FieldPosition,
    pub velocity: i16,
    pub direction: FieldPosition,
    rnd: ThreadRng,
}

impl Ball {
    pub fn new(x: i16, y: i16) -> Self {
        Ball {
            position: FieldPosition { x, y },
            start_position: FieldPosition { x, y },
            velocity: 0,
            direction: FieldPosition { x: 0, y: 0 },
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
        let speed = self.rnd.gen_range(-2..2) as i16;
        let speed2 = self.rnd.gen_range(-2..2) as i16;

        self.position.x += speed * speed2;
        self.position.y += speed * speed2;
    }

    pub fn move_towards_player(&mut self, player_pos: &FieldPosition) {
        let dx = (player_pos.x - self.position.x) as f64;
        let dy = (player_pos.y - self.position.y) as f64;

        let distance = (dx.powi(2) + dy.powi(2)).sqrt();

        self.position.x += ((dx / distance) * self.velocity as f64) as i16;
        self.position.y += ((dy / distance) * self.velocity as f64) as i16;
    }

    pub fn reset(&mut self) {
        self.position.x = self.start_position.x;
        self.position.y = self.start_position.y;
    }
}

pub enum BallUpdateEvent {
    Goal,
}
