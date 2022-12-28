use crate::SimulatorData;

pub struct TeamTrainingResult {
    pub player_results: Vec<PlayerTrainingResult>,
}

impl TeamTrainingResult {
    pub fn new() -> Self {
        TeamTrainingResult {
            player_results: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        TeamTrainingResult {
            player_results: Vec::new(),
        }
    }

    pub fn process(&self, data: &mut SimulatorData) {
        for player_result in &self.player_results {
            player_result.process(data);
        }
    }
}

pub struct PlayerTrainingMentalResult {
    pub diff: f64,
}

pub struct PlayerTrainingPhysicalResult {}
pub struct PlayerTrainingTechnicalResult {}

pub struct PlayerTrainingResult {
    pub mental: PlayerTrainingMentalResult,
    pub physical: PlayerTrainingPhysicalResult,
    pub technical: PlayerTrainingTechnicalResult,
}

impl PlayerTrainingResult {
    pub fn new() -> Self {
        PlayerTrainingResult {
            mental: PlayerTrainingMentalResult::new(),
            physical: PlayerTrainingPhysicalResult::new(),
            technical: PlayerTrainingTechnicalResult::new(),
        }
    }

    pub fn set_mental(&mut self, result: PlayerTrainingMentalResult) {
        self.mental = result;
    }

    pub fn set_physical(&mut self, result: PlayerTrainingPhysicalResult) {
        self.physical = result;
    }

    pub fn set_technical(&mut self, result: PlayerTrainingTechnicalResult) {
        self.technical = result;
    }

    pub fn process(&self, _: &mut SimulatorData) {}
}

impl PlayerTrainingMentalResult {
    pub fn new() -> Self {
        PlayerTrainingMentalResult { diff: 0f64 }
    }
}

impl PlayerTrainingPhysicalResult {
    pub fn new() -> Self {
        PlayerTrainingPhysicalResult {}
    }
}

impl PlayerTrainingTechnicalResult {
    pub fn new() -> Self {
        PlayerTrainingTechnicalResult {}
    }
}
