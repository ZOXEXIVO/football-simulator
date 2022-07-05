use crate::academy::AcademyPlayer;
use crate::{SimulatorData};

pub struct ClubAcademyResult {
    pub players: Vec<AcademyPlayer>,
}

impl ClubAcademyResult {
    pub fn new() -> Self {
        ClubAcademyResult {
            players: Vec::new(),
        }
    }

    pub fn process(&self, _: &mut SimulatorData) {}
}
