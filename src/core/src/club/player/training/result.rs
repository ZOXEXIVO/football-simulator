use crate::training::skills::result::{
    PlayerTrainingMentalResult, PlayerTrainingPhysicalResult, PlayerTrainingTechnicalResult,
};
use crate::training::skills::{MentalSkill, PhysicalSkill, TechnicalSkill};
use crate::SimulatorData;

pub struct PlayerTrainingResult {
    pub player_id: u32,
    pub mental: PlayerTrainingMentalResult,
    pub physical: PlayerTrainingPhysicalResult,
    pub technical: PlayerTrainingTechnicalResult,
}

impl PlayerTrainingResult {
    pub fn new(player_id: u32) -> Self {
        PlayerTrainingResult {
            player_id,
            mental: PlayerTrainingMentalResult::new(),
            physical: PlayerTrainingPhysicalResult::new(),
            technical: PlayerTrainingTechnicalResult::new(),
        }
    }

    pub fn process(&self, data: &mut SimulatorData) {
        self.mental.process(data, self.player_id);
        self.physical.process(data, self.player_id);
        self.technical.process(data, self.player_id);
    }
}
