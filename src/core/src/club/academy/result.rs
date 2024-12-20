use crate::{PlayerCollectionResult, PlayerResult, SimulatorData};

pub struct ClubAcademyResult {
    pub players: PlayerCollectionResult,
    pub produce_result: ProduceYouthPlayersResult,
}

impl ClubAcademyResult {
    pub fn new(players: PlayerCollectionResult, produce_result: ProduceYouthPlayersResult) -> Self {
        ClubAcademyResult {
            players,
            produce_result,
        }
    }

    pub fn process(&self, _: &mut SimulatorData) {}
}

pub struct ProduceYouthPlayersResult {
    pub players: Vec<PlayerResult>,
}

impl ProduceYouthPlayersResult {
    pub fn new(players: Vec<PlayerResult>) -> Self {
        ProduceYouthPlayersResult { players }
    }

    pub fn process(&self, _: &mut SimulatorData) {}
}
