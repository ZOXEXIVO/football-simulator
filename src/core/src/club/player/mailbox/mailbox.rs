use crate::handlers::ProcessContractHandler;
use crate::{Player, PlayerMailboxResult, PlayerResult};
use chrono::NaiveDate;
use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(Debug)]
pub struct PlayerMessage {
    pub message_type: PlayerMessageType,
}

#[derive(Debug)]
pub enum PlayerMessageType {
    Greeting,
    ContractProposal(PlayerContractProposal),
}

#[derive(Debug)]
pub struct PlayerContractProposal {
    pub salary: u32,
    pub years: u8,
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

    pub fn process(
        player: &mut Player,
        player_result: &mut PlayerResult,
        now: NaiveDate,
    ) -> PlayerMailboxResult {
        let result = PlayerMailboxResult::new();

        for message in player.mailbox.get() {
            match message.message_type {
                PlayerMessageType::Greeting => {}
                PlayerMessageType::ContractProposal(proposal) => {
                    ProcessContractHandler::process(player, proposal, now, player_result);
                }
            }
        }

        result
    }

    pub fn push(&self, message: PlayerMessage) {
        let mut messages = self.messages.lock().unwrap();
        messages.push_back(message);
    }

    pub fn get(&self) -> Vec<PlayerMessage> {
        let mut messages = self.messages.lock().unwrap();
        messages.drain(..).collect()
    }
}
