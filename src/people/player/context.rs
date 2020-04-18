#[derive(Clone)]
pub struct PlayerContext {
    pub transfer_requests: Vec<u32>,
}

impl PlayerContext {
    pub fn new() -> Self {
        PlayerContext {
            transfer_requests: Vec::new(),
        }
    }
    
    pub fn request_transfer(&mut self, player_id: u32) {
        self.transfer_requests.push(player_id);
    }
}