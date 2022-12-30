use crate::club::PlayerPositionType;
use crate::{Player, PlayerTrainingHistory};
use half::f16;

const SKILL_MIN_VALUE: f32 = 1.0;
const SKILL_MAX_VALUE: f32 = 20.0;

#[derive(Debug)]
pub struct PlayerSkills {
    pub technical: Technical,
    pub mental: Mental,
    pub physical: Physical,
}

impl PlayerSkills {
    pub fn get_for_position(&self, position: PlayerPositionType) -> u32 {
        self.technical.get_for_position(position)
            + self.mental.get_for_position(position)
            + self.physical.get_for_position(position)
    }
}

#[derive(Debug)]
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
    pub fn get_for_position(&self, position: PlayerPositionType) -> u32 {
        match position {
            PlayerPositionType::Goalkeeper => {
                (self.penalty_taking + self.first_touch + self.free_kicks) as u32
            }

            PlayerPositionType::Sweeper
            | PlayerPositionType::DefenderLeft
            | PlayerPositionType::DefenderCenter
            | PlayerPositionType::DefenderRight => {
                (self.dribbling + self.heading + self.marking + self.passing + self.tackling) as u32
            }

            PlayerPositionType::MidfielderLeft
            | PlayerPositionType::MidfielderCenter
            | PlayerPositionType::MidfielderRight => {
                (self.dribbling
                    + self.crossing
                    + self.marking
                    + self.passing
                    + self.tackling
                    + self.technique
                    + self.long_shots) as u32
            }

            PlayerPositionType::WingbackLeft
            | PlayerPositionType::Striker
            | PlayerPositionType::WingbackRight => {
                (self.dribbling + self.first_touch + self.finishing + self.passing) as u32
            }

            _ => 0,
        }
    }

    pub fn rest(&mut self) {}
}

#[derive(Debug)]
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
    pub fn get_for_position(&self, position: PlayerPositionType) -> u32 {
        match position {
            PlayerPositionType::Goalkeeper => {
                (self.vision + self.off_the_ball + self.leadership) as u32
            }

            PlayerPositionType::Sweeper
            | PlayerPositionType::DefenderLeft
            | PlayerPositionType::DefenderCenter
            | PlayerPositionType::DefenderRight => {
                (self.aggression + self.positioning + self.off_the_ball + self.anticipation) as u32
            }

            PlayerPositionType::MidfielderLeft
            | PlayerPositionType::MidfielderCenter
            | PlayerPositionType::MidfielderRight => {
                (self.work_rate
                    + self.teamwork
                    + self.positioning
                    + self.decisions
                    + self.vision
                    + self.off_the_ball) as u32
            }

            PlayerPositionType::WingbackLeft
            | PlayerPositionType::Striker
            | PlayerPositionType::WingbackRight => {
                (self.concentration + self.vision + self.positioning) as u32
            }

            _ => 0,
        }
    }

    pub fn rest(&mut self) {}
}

#[derive(Debug)]
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
    pub fn get_for_position(&self, position: PlayerPositionType) -> u32 {
        match position {
            PlayerPositionType::Goalkeeper => {
                (self.agility + self.balance + self.pace + self.jumping) as u32
            }

            PlayerPositionType::Sweeper
            | PlayerPositionType::DefenderLeft
            | PlayerPositionType::DefenderCenter
            | PlayerPositionType::DefenderRight => {
                (self.agility + self.natural_fitness + self.stamina + self.pace) as u32
            }

            PlayerPositionType::MidfielderLeft
            | PlayerPositionType::MidfielderCenter
            | PlayerPositionType::MidfielderRight => {
                (self.acceleration
                    + self.natural_fitness
                    + self.pace
                    + self.stamina
                    + self.strength) as u32
            }

            PlayerPositionType::WingbackLeft
            | PlayerPositionType::Striker
            | PlayerPositionType::WingbackRight => (self.acceleration + self.stamina) as u32,
            _ => 0,
        }
    }

    pub fn rest(&mut self) {}
}

#[inline]
fn safe_modify(skill: &mut f32, val: i8) {
    if val < 0 {
        let abs_val = -val as f32;

        if *skill <= abs_val {
            *skill = SKILL_MIN_VALUE;
        } else {
            *skill -= abs_val;
        }
    } else {
        let abs_val = val as f32;

        if *skill + abs_val > SKILL_MAX_VALUE {
            *skill = SKILL_MAX_VALUE;
        } else {
            *skill += abs_val;
        }
    }
}
