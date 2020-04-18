pub struct StaffResult {
    pub transfer_requests: Vec<u32>
}

impl StaffResult{
    pub fn new() -> Self {
        StaffResult {
            transfer_requests: Vec::new()
        }
    }
    
    pub fn request_transfer(&mut self, player_id: u32) {
        self.transfer_requests.push(player_id);
    }
}