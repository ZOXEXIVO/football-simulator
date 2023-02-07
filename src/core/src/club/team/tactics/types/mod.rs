use crate::training::skills::{MentalSkill, PhysicalSkill, TechnicalSkill};
use crate::PlayerPositionType;
use std::collections::HashMap;

pub mod tactic_442;

pub trait Tactic {
    fn get_player_positions(&self) -> Vec<PlayerPositionType>;
    fn get_skill_priority(&self) -> SkillPriority;
}

pub struct SkillPriority {
    pub technical: HashMap<TechnicalSkill, f32>,
    pub mental: HashMap<MentalSkill, f32>,
    pub physical: HashMap<PhysicalSkill, f32>,
}

pub struct TechnicalSkillPriority {
    pub technical_skill: HashMap<TechnicalSkill, f32>,
}

pub struct MentalSkillPriority {
    pub mental_skill: HashMap<MentalSkill, f32>,
}

pub struct PhysicalSkillPriority {
    pub physical_skill: HashMap<PhysicalSkill, f32>,
}
