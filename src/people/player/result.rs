pub struct PlayerResult {
    pub transfer_requests: Vec<u32>
}

impl PlayerResult{
    pub fn new() -> Self {
        PlayerResult {
            transfer_requests: Vec::new()
        }
    }
    
    pub fn request_transfer(&mut self, player_id: u32) {
        self.transfer_requests.push(player_id);
    }
}