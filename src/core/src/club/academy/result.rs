use crate::academy::AcademyPlayer;
use crate::{Player, SimulatorData};

pub struct ClubAcademyResult {
    pub players: Vec<AcademyPlayer>,
}

impl ClubAcademyResult {
    pub fn new() -> Self {
        ClubAcademyResult {
            players: Vec::new(),
        }
    }

    pub fn process(&self, data: &mut SimulatorData) {}
}
