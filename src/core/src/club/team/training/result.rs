use crate::SimulatorData;

pub struct TeamTrainingResult {
    trained: bool
}

impl TeamTrainingResult {
    pub fn new() -> Self {
        TeamTrainingResult {
            trained: true
        }
    }

    pub fn empty() -> Self {
        TeamTrainingResult {
            trained: false
        }
    }

    pub fn process(&self, data: &mut SimulatorData) {
        if !self.trained {
            
        }
    }
}
