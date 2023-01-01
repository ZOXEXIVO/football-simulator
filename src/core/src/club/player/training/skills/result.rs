use crate::training::skills::{MentalSkill, PhysicalSkill, TechnicalSkill};
use crate::SimulatorData;

pub struct PlayerTrainingTechnicalResult {
    pub skill_increase: Vec<(TechnicalSkill, f32)>,
}

impl PlayerTrainingTechnicalResult {
    pub fn new() -> Self {
        PlayerTrainingTechnicalResult {
            skill_increase: Vec::new(),
        }
    }

    pub fn process(&self, data: &mut SimulatorData, player_id: u32) {
        let player = match data.player_mut(player_id) {
            Some(player) => player,
            None => return,
        };

        for (skill, increase) in &self.skill_increase {
            match skill {
                TechnicalSkill::Corners => player.skills.technical.corners += increase,
                TechnicalSkill::Crossing => player.skills.technical.crossing += increase,
                TechnicalSkill::Dribbling => player.skills.technical.dribbling += increase,
                TechnicalSkill::Finishing => player.skills.technical.finishing += increase,
                TechnicalSkill::FirstTouch => player.skills.technical.first_touch += increase,
                TechnicalSkill::FreeKicks => player.skills.technical.free_kicks += increase,
                TechnicalSkill::Heading => player.skills.technical.heading += increase,
                TechnicalSkill::LongShots => player.skills.technical.long_shots += increase,
                TechnicalSkill::LongThrows => player.skills.technical.long_throws += increase,
                TechnicalSkill::Marking => player.skills.technical.marking += increase,
                TechnicalSkill::Passing => player.skills.technical.passing += increase,
                TechnicalSkill::PenaltyTaking => player.skills.technical.penalty_taking += increase,
                TechnicalSkill::Tackling => player.skills.technical.tackling += increase,
                TechnicalSkill::Technique => player.skills.technical.technique += increase,
            }
        }
    }
}

pub struct PlayerTrainingMentalResult {
    pub skill_increase: Vec<(MentalSkill, f32)>,
}

impl PlayerTrainingMentalResult {
    pub fn new() -> Self {
        PlayerTrainingMentalResult {
            skill_increase: Vec::new(),
        }
    }

    pub fn process(&self, data: &mut SimulatorData, player_id: u32) {
        let player = match data.player_mut(player_id) {
            Some(player) => player,
            None => return,
        };

        for (skill, increase) in &self.skill_increase {
            match skill {
                MentalSkill::Aggression => player.skills.mental.aggression += increase,
                MentalSkill::Anticipation => player.skills.mental.anticipation += increase,
                MentalSkill::Bravery => player.skills.mental.bravery += increase,
                MentalSkill::Composure => player.skills.mental.composure += increase,
                MentalSkill::Concentration => player.skills.mental.concentration += increase,
                MentalSkill::Decisions => player.skills.mental.decisions += increase,
                MentalSkill::Determination => player.skills.mental.determination += increase,
                MentalSkill::Flair => player.skills.mental.flair += increase,
                MentalSkill::Leadership => player.skills.mental.leadership += increase,
                MentalSkill::OffTheBall => player.skills.mental.off_the_ball += increase,
                MentalSkill::Positioning => player.skills.mental.positioning += increase,
                MentalSkill::Teamwork => player.skills.mental.teamwork += increase,
                MentalSkill::Vision => player.skills.mental.vision += increase,
                MentalSkill::WorkRate => player.skills.mental.work_rate += increase,
            }
        }
    }
}

pub struct PlayerTrainingPhysicalResult {
    pub skill_increase: Vec<(PhysicalSkill, f32)>,
}

impl PlayerTrainingPhysicalResult {
    pub fn new() -> Self {
        PlayerTrainingPhysicalResult {
            skill_increase: Vec::new(),
        }
    }

    pub fn process(&self, data: &mut SimulatorData, player_id: u32) {
        let player = match data.player_mut(player_id) {
            Some(player) => player,
            None => return,
        };

        for (skill, increase) in &self.skill_increase {
            match skill {
                PhysicalSkill::Acceleration => player.skills.physical.acceleration += increase,
                PhysicalSkill::Agility => player.skills.physical.agility += increase,
                PhysicalSkill::Balance => player.skills.physical.balance += increase,
                PhysicalSkill::Jumping => player.skills.physical.jumping += increase,
                PhysicalSkill::NaturalFitness => player.skills.physical.natural_fitness += increase,
                PhysicalSkill::Pace => player.skills.physical.pace += increase,
                PhysicalSkill::Stamina => player.skills.physical.stamina += increase,
                PhysicalSkill::Strength => player.skills.physical.strength += increase,
            }
        }
    }
}
