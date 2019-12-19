use crate::core::SimulationContext;
use crate::shared::fullname::FullName;
use crate::player::skills::*;
use crate::utils::{DateUtils, IntegerUtils};
use std::fmt::{Display, Formatter, Result};

use chrono::NaiveDate;
use crate::{PlayerAttributes, PlayerMailbox};

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub full_name: FullName,
    pub birth_date: NaiveDate,
    pub skills: PlayerSkills,
    pub positions: Vec<PlayerPosition>,
    pub preferred_foot: PlayerFoot,
    pub attributes: PlayerAttributes,
    pub mailbox: PlayerMailbox
}

impl Player {
    pub fn new(
        id: u32,
        full_name: FullName,
        birth_date: NaiveDate,
        skills: PlayerSkills,
        positions: Vec<PlayerPosition>
    ) -> Self {
        Player {
            id,
            full_name,
            birth_date,
            skills,
            positions,
            preferred_foot: PlayerFoot::Right,
            attributes: PlayerAttributes::new(),
            mailbox: PlayerMailbox::new()
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) -> Vec<PlayerEvent> {
        let mut result_events = Vec::new();

        if DateUtils::is_birthday(self.birth_date, context.date.date()) {
            result_events.push(
                PlayerEvent::Birthday(self.id)
            );
        }

        self.train();

        result_events
    }

    pub fn get_skill(&self) -> u32 {
        self.positions.iter().map(|position| {
            self.skills.get_for_position(position)
        }).sum()
    } 
    
    pub fn train(&mut self) {
        let change_val = IntegerUtils::random(-3, 3) as i8;

        self.skills.train(change_val);
    }
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    Birthday(u32),
    ContractExpired(u32),
}


#[derive(Debug, Clone)]
pub enum PlayerFoot {
    Left,
    Right,
    Both,
}

#[derive(Debug, Clone)]
pub enum PlayerPosition {
    //defenders
    Goalkeeper,
    Libero,
    Sweeper,
    Wingerback,
    RightLeftBack,
    LimitedDefender,
    BallPlayingDefender,
    CentralDefender,
}

//DISPLAY
impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.full_name, self.birth_date)
    }
}
