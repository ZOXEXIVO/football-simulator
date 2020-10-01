use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Debug)]
pub enum PlayerMessage {
    Greeting,
}

#[derive(Debug)]
pub struct PlayerMailbox {
    messages: Mutex<VecDeque<PlayerMessage>>,
}

impl PlayerMailbox {
    pub fn new() -> Self {
        PlayerMailbox {
            messages: Mutex::new(VecDeque::new()),
        }
    }
    
    pub fn push(&mut self, message: PlayerMessage) {
        let mut messages = self.messages.lock().unwrap();
        messages.push_back(message);
    }
    
    pub fn get(&mut self) -> Vec<PlayerMessage>{
        let mut messages = self.messages.lock().unwrap();
        messages.drain(..).collect()
    }
}

