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
    pub player_id: u32,
    pub want_new_contract: bool,
    pub no_contract: bool,
    pub is_transfer_requested: bool,
    pub transfer_requests: Vec<u32>
}

impl PlayerResult{
    pub fn new(player_id: u32) -> Self {
        PlayerResult {
            player_id,
            want_new_contract: false,
            no_contract: false,
            is_transfer_requested: false,
            transfer_requests: Vec::new()
        }
    }

    pub fn process(&self, data: &mut SimulatorData){
        if self.is_transfer_requested {
            
        }
        
        if self.want_new_contract {
            
        }
    }
    
    pub fn request_transfer(&mut self, player_id: u32) {
        self.transfer_requests.push(player_id);
    }
}
