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
        for player in &self.players {
            player.process(data);
        }
    }
}

pub struct PlayerResult {
    pub want_new_contract: bool,
    pub is_transfer_requested: bool,
    pub player_id: u32,
    pub transfer_requests: Vec<u32>
}

impl PlayerResult{
    pub fn new(player_id: u32) -> Self {
        PlayerResult {
            want_new_contract: false,
            is_transfer_requested: false,
            player_id,
            transfer_requests: Vec::new()
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        if self.is_transfer_requested {
            
        }
    }
    
    pub fn request_transfer(&mut self, player_id: u32) {
        self.transfer_requests.push(player_id);
    }
}
