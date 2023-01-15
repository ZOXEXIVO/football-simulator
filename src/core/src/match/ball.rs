use crate::r#match::position::FieldPosition;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

pub struct Ball {
    pub position: FieldPosition,
    pub speed: i16,
    pub direction: FieldPosition,
    rnd: ThreadRng,
}

impl Ball {
    pub fn new(x: i16, y: i16) -> Self {
        Ball {
            position: FieldPosition { x, y },
            speed: 0,
            direction: FieldPosition { x: 0, y: 0 },
            rnd: thread_rng(),
        }
    }

    pub fn move_ball(&mut self) {
        let speed = self.rnd.gen_range(-2..2) as i16;
        let speed2 = self.rnd.gen_range(-2..2) as i16;

        self.position.x += speed * speed2;
        self.position.y += speed * speed2;
    }

    pub fn move_towards_player(&mut self, player_pos: &FieldPosition) {
        let dx = (player_pos.x - self.position.x) as f64;
        let dy = (player_pos.y - self.position.y) as f64;

        let distance = (dx.powi(2) + dy.powi(2)).sqrt();

        self.position.x += ((dx / distance) * self.speed as f64) as i16;
        self.position.y += ((dy / distance) * self.speed as f64) as i16;
    }
}
