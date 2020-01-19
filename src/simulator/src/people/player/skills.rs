use crate::PlayerPositionType;

const SKILL_MIN_VALUE: u8 = 1;
const SKILL_MAX_VALUE: u8 = 20;

#[derive(Debug, Clone)]
pub struct PlayerSkills {
    pub technical: Technical,
    pub mental: Mental,
    pub physical: Physical,
}

impl PlayerSkills {
    pub fn get_for_position(&self, position: &PlayerPositionType) -> u32 {
        self.technical.get_for_position(position)
            + self.mental.get_for_position(position)
            + self.physical.get_for_position(position)
    }

    pub fn train(&mut self, val: i8) {
        self.technical.train(val);
        self.mental.train(val);
        self.physical.train(val);
    }

    pub fn rest(&mut self) {
        self.technical.rest();
        self.mental.rest();
        self.physical.rest();
    }
}

#[derive(Debug, Clone)]
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
    pub fn get_for_position(&self, position: &PlayerPositionType) -> u32 {
        return match position {
            PlayerPositionType::Goalkeeper => {
                return (self.penalty_taking + self.first_touch + self.free_kick_taking) as u32;
            }

            PlayerPositionType::Defender => {
                return (self.dribbling + self.heading + self.marking + self.passing + self.tackling)
                    as u32;
            }

            PlayerPositionType::Midfielder => {
                return (self.dribbling
                    + self.crossing
                    + self.marking
                    + self.passing
                    + self.tackling
                    + self.technique
                    + self.long_shots) as u32;
            }

            PlayerPositionType::Forward => {
                return (self.dribbling + self.first_touch + self.finishing + self.passing) as u32;
            }

            _ => 0,
        };
    }

    pub fn train(&mut self, val: i8) {
        safe_modify(&mut self.corners, val);
        safe_modify(&mut self.crossing, val);
        safe_modify(&mut self.dribbling, val);
        safe_modify(&mut self.finishing, val);
        safe_modify(&mut self.first_touch, val);
        safe_modify(&mut self.free_kick_taking, val);
        safe_modify(&mut self.heading, val);
        safe_modify(&mut self.long_shots, val);
        safe_modify(&mut self.long_throws, val);
        safe_modify(&mut self.marking, val);
        safe_modify(&mut self.passing, val);
        safe_modify(&mut self.penalty_taking, val);
        safe_modify(&mut self.tackling, val);
        safe_modify(&mut self.technique, val);
    }

    pub fn rest(&mut self) {}
}

#[derive(Debug, Clone)]
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
    pub fn get_for_position(&self, position: &PlayerPositionType) -> u32 {
        return match position {
            PlayerPositionType::Goalkeeper => {
                return (self.vision + self.off_the_ball + self.leadership) as u32;
            }

            PlayerPositionType::Defender => {
                return (self.aggression + self.positioning + self.off_the_ball + self.anticipation)
                    as u32;
            }

            PlayerPositionType::Midfielder => {
                return (self.work_rate
                    + self.teamwork
                    + self.positioning
                    + self.decisions
                    + self.vision
                    + self.off_the_ball) as u32;
            }

            PlayerPositionType::Forward => {
                return (self.concentration + self.vision + self.positioning) as u32;
            }

            _ => 0,
        };
    }

    pub fn train(&mut self, val: i8) {
        safe_modify(&mut self.aggression, val);
        safe_modify(&mut self.anticipation, val);
        safe_modify(&mut self.bravery, val);
        safe_modify(&mut self.composure, val);
        safe_modify(&mut self.concentration, val);
        safe_modify(&mut self.decisions, val);
        safe_modify(&mut self.determination, val);
        safe_modify(&mut self.flair, val);
        safe_modify(&mut self.leadership, val);
        safe_modify(&mut self.off_the_ball, val);
        safe_modify(&mut self.positioning, val);
        safe_modify(&mut self.teamwork, val);
        safe_modify(&mut self.vision, val);
        safe_modify(&mut self.work_rate, val);
    }

    pub fn rest(&mut self) {}
}

#[derive(Debug, Clone)]
pub struct Physical {
    pub acceleration: u8,
    pub agility: u8,
    pub balance: u8,
    pub jumping_reach: u8,
    pub natural_fitness: u8,
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,
}

impl Physical {
    pub fn get_for_position(&self, position: &PlayerPositionType) -> u32 {
        return match position {
            PlayerPositionType::Goalkeeper => {
                return (self.agility + self.balance + self.pace + self.jumping_reach) as u32;
            }

            PlayerPositionType::Defender => {
                return (self.agility + self.natural_fitness + self.stamina + self.pace) as u32;
            }

            PlayerPositionType::Midfielder => {
                return (self.acceleration
                    + self.natural_fitness
                    + self.pace
                    + self.stamina
                    + self.strength) as u32;
            }

            PlayerPositionType::Forward => {
                return (self.acceleration + self.stamina) as u32;
            }
            _ => 0,
        };
    }

    pub fn train(&mut self, val: i8) {
        safe_modify(&mut self.acceleration, val);
        safe_modify(&mut self.agility, val);
        safe_modify(&mut self.balance, val);
        safe_modify(&mut self.jumping_reach, val);
        safe_modify(&mut self.natural_fitness, val);
        safe_modify(&mut self.pace, val);
        safe_modify(&mut self.stamina, val);
        safe_modify(&mut self.strength, val);
    }

    pub fn rest(&mut self) {}
}

#[inline]
fn safe_modify(skill: &mut u8, val: i8) {
    if val < 0 {
        let abs_val = (val * -1) as u8;

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
