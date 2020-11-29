use chrono::{NaiveDate, Datelike, Duration};
use crate::shared::FullName;
use std::ops::Add;

pub trait Person {
    fn id(&self) -> u32;
    fn fullname(&self) -> &FullName;
    fn birthday(&self) -> NaiveDate;
    
    fn age(&self, now: NaiveDate) -> u8 { 
        let mut age = now.year() - self.birthday().year();
 
        if now.month() < self.birthday().month() || 
            (now.month() == self.birthday().month() && now.day() < self.birthday().day()) {
            age -= 1;
        }
        
        age as u8
    }
    
    fn behaviour(&self) -> &PersonBehaviour;
    fn attributes(&self) -> &PersonAttributes;
}

#[derive(Debug)]
pub struct PersonAttributes{
    pub adaptability: u8,
    pub ambition: u8,
    pub controversy: u8,
    pub loyalty: u8,
    pub pressure: u8,
    pub professionalism: u8,
    pub sportsmanship: u8,
    pub temperament: u8
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
