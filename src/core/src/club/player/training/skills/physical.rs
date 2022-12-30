use crate::{
    MentalFocusType, MentalSkill, PhysicalFocusType, PhysicalSkill, Player, Staff,
    TechnicalFocusType, TechnicalSkill,
};

pub fn determine_physical_skills_to_increase(
    weeks_since_last_training: u32,
    coach: &Staff,
    player: &Player,
) -> Vec<(PhysicalSkill, f32)> {
    let mut skills_to_increase = Vec::new();

    if let Some(focus) = &coach.focus {
        for skill in focus.physical_focus.iter() {
            match skill {
                PhysicalFocusType::Acceleration => {
                    if player.skills.physical.acceleration < 20.0 {
                        skills_to_increase.push((PhysicalSkill::Acceleration, 0.0f32));
                    }
                }
                PhysicalFocusType::Agility => {
                    if player.skills.physical.agility < 20.0 {
                        skills_to_increase.push((PhysicalSkill::Agility, 0.0f32));
                    }
                }
                PhysicalFocusType::Balance => {
                    if player.skills.physical.balance < 20.0 {
                        skills_to_increase.push((PhysicalSkill::Balance, 0.0f32));
                    }
                }
                PhysicalFocusType::Jumping => {
                    if player.skills.physical.jumping < 20.0 {
                        skills_to_increase.push((PhysicalSkill::Jumping, 0.0f32));
                    }
                }
                PhysicalFocusType::NaturalFitness => {
                    if player.skills.physical.natural_fitness < 20.0 {
                        skills_to_increase.push((PhysicalSkill::NaturalFitness, 0.0f32));
                    }
                }
                PhysicalFocusType::Pace => {
                    if player.skills.physical.pace < 20.0 {
                        skills_to_increase.push((PhysicalSkill::Pace, 0.0f32));
                    }
                }
                PhysicalFocusType::Stamina => {
                    if player.skills.physical.stamina < 20.0 {
                        skills_to_increase.push((PhysicalSkill::Stamina, 0.0f32));
                    }
                }
                PhysicalFocusType::Strength => {
                    if player.skills.physical.strength < 20.0 {
                        skills_to_increase.push((PhysicalSkill::Strength, 0.0f32));
                    }
                }
                PhysicalFocusType::MatchReadiness => {}
            }
        }
    }

    skills_to_increase
}
