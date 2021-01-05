use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Debug)]
pub struct PlayerMessage {
    pub message_type: PlayerMessageType
}

#[derive(Debug)]
pub enum PlayerMessageType {
    Greeting,
    ContractProposal(PlayerContractProposal)
}

#[derive(Debug)]
pub struct PlayerContractProposal{
    pub salary: u32,
    pub years: u8
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
    
    pub fn push(&self, message: PlayerMessage) {
        let mut messages = self.messages.lock().unwrap();
        messages.push_back(message);
    }
    
    pub fn get(&self) -> Vec<PlayerMessage>{
        let mut messages = self.messages.lock().unwrap();
        messages.drain(..).collect()
    }
}

