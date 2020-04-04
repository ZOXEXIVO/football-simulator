pub struct PlayerContext {
    pub contract_improve_requests: Vec<u32>,
    pub transfer_requests: Vec<u32>,
}

impl PlayerContext {
    pub fn new() -> Self {
        PlayerContext {
            contract_improve_requests: Vec::new(),
            transfer_requests: Vec::new(),
        }
    }
    
    pub fn request_contract_improvement(&mut self, player_id: u32) {
        self.contract_improve_requests.push(player_id);
    }

    pub fn request_transfer(&mut self, player_id: u32) {
        self.transfer_requests.push(player_id);
    }
}