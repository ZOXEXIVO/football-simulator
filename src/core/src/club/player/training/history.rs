#[derive(Debug)]
pub struct PlayerTrainingHistoryItem {
    
}

#[derive(Debug)]
pub struct PlayerTrainingHistory {
    data: Vec<PlayerTrainingHistoryItem>,
}

impl PlayerTrainingHistory {
    pub fn new() -> Self {
        PlayerTrainingHistory { data: Vec::new() }
    }
}
