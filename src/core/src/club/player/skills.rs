use crate::club::PlayerPositionType;
use crate::{
    Player, PlayerTrainingHistory, PlayerTrainingMentalResult, PlayerTrainingPhysicalResult,
    PlayerTrainingTechnicalResult,
};

const SKILL_MIN_VALUE: u8 = 1;
const SKILL_MAX_VALUE: u8 = 20;

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
    pub corners: u8,
    pub crossing: u8,
    pub dribbling: u8,
    pub finishing: u8,
    pub first_touch: u8,
    pub free_kick_taking: u8,
    pub heading: u8,
    pub long_shots: u8,
    pub long_throws: u8,
    pub marking: u8,
    pub passing: u8,
    pub penalty_taking: u8,
    pub tackling: u8,
    pub technique: u8,
}

impl Technical {
    pub fn get_for_position(&self, position: PlayerPositionType) -> u32 {
        match position {
            PlayerPositionType::Goalkeeper => {
                (self.penalty_taking + self.first_touch + self.free_kick_taking) as u32
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

    pub fn train(
        &self,
        player: &Player,
        training_history: &PlayerTrainingHistory,
    ) -> PlayerTrainingTechnicalResult {
        let mut result = PlayerTrainingTechnicalResult::new();

        result
    }

    pub fn rest(&mut self) {}
}

#[derive(Debug)]
pub struct Mental {
    pub aggression: u8,
    pub anticipation: u8,
    pub bravery: u8,
    pub composure: u8,
    pub concentration: u8,
    pub decisions: u8,
    pub determination: u8,
    pub flair: u8,
    pub leadership: u8,
    pub off_the_ball: u8,
    pub positioning: u8,
    pub teamwork: u8,
    pub vision: u8,
    pub work_rate: u8,
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

    pub fn train(
        &self,
        player: &Player,
        training_history: &PlayerTrainingHistory,
    ) -> PlayerTrainingMentalResult {
         let mut result = PlayerTrainingMentalResult::new();
        
         result
    }

    pub fn rest(&mut self) {}
}

#[derive(Debug)]
pub struct Physical {
    pub acceleration: u8,
    pub agility: u8,
    pub balance: u8,
    pub jumping_reach: u8,
    pub natural_fitness: u8,
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,

    pub match_readiness: u8,
}

impl Physical {
    pub fn get_for_position(&self, position: PlayerPositionType) -> u32 {
        match position {
            PlayerPositionType::Goalkeeper => {
                (self.agility + self.balance + self.pace + self.jumping_reach) as u32
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

    pub fn train(
        &self,
        player: &Player,
        training_history: &PlayerTrainingHistory,
    ) -> PlayerTrainingPhysicalResult {
        let mut result = PlayerTrainingPhysicalResult::new();

        result
    }

    pub fn rest(&mut self) {}
}

#[inline]
fn safe_modify(skill: &mut u8, val: i8) {
    if val < 0 {
        let abs_val = -val as u8;

        if *skill <= abs_val {
            *skill = SKILL_MIN_VALUE;
        } else {
            *skill -= abs_val;
        }
    } else {
        let abs_val = val as u8;

        if *skill + abs_val > SKILL_MAX_VALUE {
            *skill = SKILL_MAX_VALUE;
        } else {
            *skill += abs_val;
        }
    }
}
