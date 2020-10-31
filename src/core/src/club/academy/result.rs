use crate::{SimulatorData, Player};

pub struct ClubAcademyResult {
    players: Vec<Player>
}

impl ClubAcademyResult {
    pub fn new() -> Self {
        ClubAcademyResult {
            players: Vec::new()
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        
    }
}
