pub enum PlayerMessage {
    Greeting
}

#[derive(Debug, Clone)]
pub struct PlayerMailbox {
    messages: Vec<PlayerMessage>
}

impl PlayerMailbox{
    pub fn new() -> Self{
        PlayerMailbox{
            messages: Vec::new()
        }
    }
}
