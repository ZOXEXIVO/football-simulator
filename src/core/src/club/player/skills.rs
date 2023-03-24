use nalgebra::{Vector2, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct PlayerSkills {
    pub technical: Technical,
    pub mental: Mental,
    pub physical: Physical,
}

impl PlayerSkills {
    pub fn max_speed(&self) -> f32 {
        (self.physical.acceleration
            + self.physical.agility
            + self.physical.balance
            + self.physical.pace)
            / (4.0 * 20.0)
    }

    pub fn walking_speed(&self) -> Vector3<f32> {
        let walking_speed = (self.physical.acceleration + self.physical.stamina) / 2.0 * 0.1;
        Vector3::new(walking_speed, walking_speed, 0.0)
    }

    pub fn running_speed(&self) -> Vector3<f32> {
        let running_speed = (self.physical.acceleration + self.physical.stamina) / 2.0 * 0.15;
        Vector3::new(running_speed, running_speed, 0.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Technical {
    pub corners: f32,
    pub crossing: f32,
    pub dribbling: f32,
    pub finishing: f32,
    pub first_touch: f32,
    pub free_kicks: f32,
    pub heading: f32,
    pub long_shots: f32,
    pub long_throws: f32,
    pub marking: f32,
    pub passing: f32,
    pub penalty_taking: f32,
    pub tackling: f32,
    pub technique: f32,
}

impl Technical {
    pub fn average(&self) -> f32 {
        (self.corners
            + self.crossing
            + self.dribbling
            + self.finishing
            + self.first_touch
            + self.free_kicks
            + self.heading
            + self.long_shots
            + self.long_throws
            + self.marking
            + self.passing
            + self.penalty_taking
            + self.tackling
            + self.technique) as f32
            / 14.0
    }

    pub fn rest(&mut self) {}
}

#[derive(Debug, Copy, Clone)]
pub struct Mental {
    pub aggression: f32,
    pub anticipation: f32,
    pub bravery: f32,
    pub composure: f32,
    pub concentration: f32,
    pub decisions: f32,
    pub determination: f32,
    pub flair: f32,
    pub leadership: f32,
    pub off_the_ball: f32,
    pub positioning: f32,
    pub teamwork: f32,
    pub vision: f32,
    pub work_rate: f32,
}

impl Mental {
    pub fn average(&self) -> f32 {
        (self.aggression
            + self.anticipation
            + self.bravery
            + self.composure
            + self.concentration
            + self.decisions
            + self.determination
            + self.flair
            + self.leadership
            + self.off_the_ball
            + self.positioning
            + self.teamwork
            + self.vision
            + self.work_rate) as f32
            / 14.0
    }

    pub fn rest(&mut self) {}
}

#[derive(Debug, Copy, Clone)]
pub struct Physical {
    pub acceleration: f32,
    pub agility: f32,
    pub balance: f32,
    pub jumping: f32,
    pub natural_fitness: f32,
    pub pace: f32,
    pub stamina: f32,
    pub strength: f32,

    pub match_readiness: f32,
}

impl Physical {
    pub fn average(&self) -> f32 {
        (self.acceleration
            + self.agility
            + self.balance
            + self.jumping
            + self.natural_fitness
            + self.pace
            + self.stamina
            + self.strength) as f32
            / 8.0
    }

    pub fn rest(&mut self) {}
}
