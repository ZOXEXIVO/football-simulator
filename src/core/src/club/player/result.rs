use crate::simulator::SimulatorData;
use crate::club::Player;

pub struct PlayerCollectionResult {
    pub players: Vec<PlayerResult>,
    pub outgoing_players: Vec<Player>
}

impl PlayerCollectionResult{
    pub fn new(players: Vec<PlayerResult>, outgoing_players: Vec<Player>) -> Self {
        PlayerCollectionResult {
            players,
            outgoing_players
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        
    }
}

pub struct PlayerResult {
    pub player_id: u32,
    pub transfer_requests: Vec<u32>
}

impl PlayerResult{
    pub fn new(player_id: u32) -> Self {
        PlayerResult {
            player_id,
            transfer_requests: Vec::new()
        }
    }
    
    pub fn request_transfer(&mut self, player_id: u32) {
        self.transfer_requests.push(player_id);
    }
}
