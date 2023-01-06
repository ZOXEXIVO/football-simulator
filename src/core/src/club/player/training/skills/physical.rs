use crate::training::skills::determine_base_value_to_skill_increase;
use crate::{Person, PhysicalFocusType, Player, Staff};
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PhysicalSkill {
    Acceleration,
    Agility,
    Balance,
    Jumping,
    NaturalFitness,
    Pace,
    Stamina,
    Strength,
}

lazy_static! {
    pub static ref PHYSICAL_SKILL_INCREASE_SPEED_MAP: HashMap<PhysicalSkill, f32> = vec![
        (PhysicalSkill::Acceleration, 0.005),
        (PhysicalSkill::Agility, 0.07),
        (PhysicalSkill::Balance, 0.05),
        (PhysicalSkill::Jumping, 0.05),
        (PhysicalSkill::NaturalFitness, 0.004),
        (PhysicalSkill::Pace, 0.06),
        (PhysicalSkill::Stamina, 0.05),
        (PhysicalSkill::Strength, 0.06)
    ]
    .into_iter()
    .collect();
}

pub fn determine_physical_skills_to_increase(
    now: NaiveDate,
    weeks_since_last_training: u32,
    coach: &Staff,
    player: &Player,
) -> Vec<(PhysicalSkill, f32)> {
    if player.age(now) > 30 {
        return Vec::new();
    }

    let mut skills_to_increase = Vec::with_capacity(8);

    let base_increase_amount =
        determine_physical_skills_increase_amount(now, weeks_since_last_training, player, coach);

    if let Some(focus) = &coach.focus {
        for skill in focus.physical_focus.iter() {
            match skill {
                PhysicalFocusType::Acceleration => {
                    if player.skills.physical.acceleration < 20.0 {
                        let increase_amount = base_increase_amount
                            * PHYSICAL_SKILL_INCREASE_SPEED_MAP[&PhysicalSkill::Acceleration];

                        skills_to_increase.push((PhysicalSkill::Acceleration, increase_amount));
                    }
                }
                PhysicalFocusType::Agility => {
                    if player.skills.physical.agility < 20.0 {
                        let increase_amount = base_increase_amount
                            * PHYSICAL_SKILL_INCREASE_SPEED_MAP[&PhysicalSkill::Agility];

                        skills_to_increase.push((PhysicalSkill::Agility, increase_amount));
                    }
                }
                PhysicalFocusType::Balance => {
                    if player.skills.physical.balance < 20.0 {
                        let increase_amount = base_increase_amount
                            * PHYSICAL_SKILL_INCREASE_SPEED_MAP[&PhysicalSkill::Balance];

                        skills_to_increase.push((PhysicalSkill::Balance, increase_amount));
                    }
                }
                PhysicalFocusType::Jumping => {
                    if player.skills.physical.jumping < 20.0 {
                        let increase_amount = base_increase_amount
                            * PHYSICAL_SKILL_INCREASE_SPEED_MAP[&PhysicalSkill::Jumping];

                        skills_to_increase.push((PhysicalSkill::Jumping, increase_amount));
                    }
                }
                PhysicalFocusType::NaturalFitness => {
                    if player.skills.physical.natural_fitness < 20.0 {
                        let increase_amount = base_increase_amount
                            * PHYSICAL_SKILL_INCREASE_SPEED_MAP[&PhysicalSkill::NaturalFitness];

                        skills_to_increase.push((PhysicalSkill::NaturalFitness, increase_amount));
                    }
                }
                PhysicalFocusType::Pace => {
                    if player.skills.physical.pace < 20.0 {
                        let increase_amount = base_increase_amount
                            * PHYSICAL_SKILL_INCREASE_SPEED_MAP[&PhysicalSkill::Pace];

                        skills_to_increase.push((PhysicalSkill::Pace, increase_amount));
                    }
                }
                PhysicalFocusType::Stamina => {
                    if player.skills.physical.stamina < 20.0 {
                        let increase_amount = base_increase_amount
                            * PHYSICAL_SKILL_INCREASE_SPEED_MAP[&PhysicalSkill::Stamina];

                        skills_to_increase.push((PhysicalSkill::Stamina, increase_amount));
                    }
                }
                PhysicalFocusType::Strength => {
                    if player.skills.physical.strength < 20.0 {
                        let increase_amount = base_increase_amount
                            * PHYSICAL_SKILL_INCREASE_SPEED_MAP[&PhysicalSkill::Strength];

                        skills_to_increase.push((PhysicalSkill::Strength, increase_amount));
                    }
                }
                PhysicalFocusType::MatchReadiness => {}
            }
        }
    }

    skills_to_increase
}

pub fn determine_physical_skills_increase_amount(
    now: NaiveDate,
    weeks_since_last_training: u32,
    player: &Player,
    coach: &Staff,
) -> f32 {
    let base_increase =
        determine_base_value_to_skill_increase(now, weeks_since_last_training, player, coach);

    let weeks_since_last_training_factor = weeks_since_last_training as f32 * 0.1;

    let increase_amount = base_increase * weeks_since_last_training_factor;

    let coaching_skills = [
        coach.staff_attributes.coaching.fitness,
        coach.staff_attributes.coaching.attacking,
    ];

    let coaching_skills_average =
        coaching_skills.iter().sum::<u8>() as f32 / coaching_skills.len() as f32;

    let skill_increase_amount =
        ((player.skills.mental.determination + coaching_skills_average) / 2.0) / 20.0;

    increase_amount + skill_increase_amount * 0.1
}
