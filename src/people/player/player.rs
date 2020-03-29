use crate::people::{Behaviour, PlayerAttributes, PlayerContext, PlayerMailbox, PlayerSkills};
use crate::shared::fullname::FullName;
use crate::utils::{DateUtils, IntegerUtils};
use chrono::NaiveDate;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub full_name: FullName,
    pub birth_date: NaiveDate,
    pub behaviour: Behaviour,
    pub skills: PlayerSkills,
    pub positions: Vec<PlayerPosition>,
    pub preferred_foot: PlayerFoot,
    pub attributes: PlayerAttributes,
    pub mailbox: PlayerMailbox,
}

impl Player {
    pub fn new(
        id: u32,
        full_name: FullName,
        birth_date: NaiveDate,
        skills: PlayerSkills,
        attributes: PlayerAttributes,
        mut positions: Vec<PlayerPosition>,
    ) -> Self {
        positions.sort_by_key(|c| c.level);

        Player {
            id,
            full_name,
            birth_date,
            behaviour: Behaviour::default(),
            skills,
            positions,
            preferred_foot: PlayerFoot::Right,
            attributes,
            mailbox: PlayerMailbox::new(),
        }
    }

    pub fn simulate(&mut self, context: &mut PlayerContext) {
        if DateUtils::is_birthday(self.birth_date, context.date.date()) {
            self.behaviour.try_increase();
        }

        context.request_contract_improvement(self.id);
        context.request_transfer(self.id);

        self.train();
    }

    fn request_transfer(&self, context: &PlayerContext) {}

    pub fn position(&self) -> &PlayerPositionType {
        &self.positions.first().unwrap().position
    }

    pub fn is_ready_for_match(&self) -> bool {
        match self.skills.physical.match_readiness {
            0..=10 => false,
            10..=20 => true,
            _ => false,
        }
    }

    pub fn get_skill(&self) -> u32 {
        self.skills.get_for_position(self.position())
    }

    pub fn train(&mut self) {
        let change_val = IntegerUtils::random(-3, 3) as i8;

        self.skills.train(change_val);
    }
}

#[derive(Debug)]
pub enum PlayerFoot {
    Left,
    Right,
    Both,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PlayerPositionType {
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
}

#[derive(Debug)]
pub struct PlayerPosition {
    pub position: PlayerPositionType,
    pub level: u8,
}

//DISPLAY
impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.full_name, self.birth_date)
    }
}
