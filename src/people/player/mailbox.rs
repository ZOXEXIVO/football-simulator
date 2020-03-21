#[derive(Debug)]
pub enum PlayerMessage {
    Greeting,
}

#[derive(Debug)]
pub struct PlayerMailbox {
    messages: Vec<PlayerMessage>,
}

impl PlayerMailbox {
    pub fn new() -> Self {
        PlayerMailbox {
            messages: Vec::new(),
        }
    }
}
