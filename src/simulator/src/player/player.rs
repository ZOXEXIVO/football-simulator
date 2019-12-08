use crate::core::{SimulationContext};
use crate::shared::fullname::FullName;
use crate::player::skills::*;
use crate::utils::{DateUtils, IntegerUtils};
use std::fmt::{Display, Formatter, Result};

use chrono::NaiveDate;
use crate::PlayerAttributes;

#[derive(Debug, Clone)]
pub struct Player {
      pub id: u32,
      pub full_name: FullName,
      pub birth_date: NaiveDate,
      pub skills: PlayerSkills,
      pub preferred_foot: PlayerFoot,
      pub attributes: PlayerAttributes
}

impl Player {
      pub fn new(
            id: u32,
            full_name: FullName,
            birth_date: NaiveDate,
            skills: PlayerSkills,
      ) -> Self {
            Player {
                  id,                  
                  full_name,
                  birth_date,
                  skills,
                  preferred_foot: PlayerFoot::Right,
                  attributes: PlayerAttributes::new()
            }
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) -> Vec<PlayerEvent>{
            let mut result_events = Vec::new();

            if DateUtils::is_birthday(self.birth_date, context.date.date()) {
                  result_events.push(PlayerEvent::Birthday(self.id));
            }

            let change_val = IntegerUtils::random(-3,3) as i8;

            self.skills.train(change_val);

            result_events
      }
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
      Birthday(u32),
      ContractExpired(i32)
}


#[derive(Debug, Clone)]
pub enum PlayerFoot{
      Left,
      Right,
      Both
}

#[derive(Debug, Clone)]
pub enum PlayerPosition{
      Goalkeeper
}


//DISPLAY
impl Display for Player {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}, {}", self.full_name, self.birth_date)
      }
}
