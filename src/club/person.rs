use chrono::NaiveDate;
use crate::shared::FullName;

pub trait Person{
    fn id() -> u32;
    fn fullname(&self) -> FullName;
    fn birthday(&self) -> NaiveDate;
    fn behaviour(&self) -> PersonBehaviourState;
}

#[derive(Debug)]
pub struct PersonBehaviour {
    pub state: PersonBehaviourState,
}

impl PersonBehaviour {
    pub fn default() -> Self {
        PersonBehaviour {
            state: PersonBehaviourState::Normal,
        }
    }

    pub fn try_increase(&mut self) {
        match self.state {
            PersonBehaviourState::Poor => {
                self.state = PersonBehaviourState::Normal;
            }
            PersonBehaviourState::Normal => {
                self.state = PersonBehaviourState::Good;
            }
            _ => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PersonBehaviourState {
    Poor,
    Normal,
    Good,
}
