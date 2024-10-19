use crate::training::skills::determine_base_value_to_skill_increase;
use crate::{MentalFocusType, Player, Staff};
use chrono::NaiveDate;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MentalSkill {
    Aggression,
    Anticipation,
    Bravery,
    Composure,
    Concentration,
    Decisions,
    Determination,
    Flair,
    Leadership,
    OffTheBall,
    Positioning,
    Teamwork,
    Vision,
    WorkRate,
}

pub static MENTAL_SKILL_INCREASE_SPEED_MAP: LazyLock<HashMap<MentalSkill, f32>> =
    LazyLock::new(|| {
        vec![
            (MentalSkill::Aggression, 0.1),
            (MentalSkill::Anticipation, 0.15),
            (MentalSkill::Bravery, 0.12),
            (MentalSkill::Composure, 0.13),
            (MentalSkill::Concentration, 0.14),
            (MentalSkill::Decisions, 0.1),
            (MentalSkill::Determination, 0.11),
            (MentalSkill::Flair, 0.09),
            (MentalSkill::Leadership, 0.08),
            (MentalSkill::OffTheBall, 0.07),
            (MentalSkill::Positioning, 0.06),
            (MentalSkill::Teamwork, 0.05),
            (MentalSkill::Vision, 0.04),
            (MentalSkill::WorkRate, 0.03),
        ]
        .into_iter()
        .collect()
    });

pub fn determine_mental_skills_to_increase(
    now: NaiveDate,
    weeks_since_last_training: u32,
    coach: &Staff,
    player: &Player,
) -> Vec<(MentalSkill, f32)> {
    let mut skills_to_increase = Vec::with_capacity(14);

    let base_increase_amount =
        determine_mental_skills_increase_amount(now, weeks_since_last_training, player, coach);

    if let Some(focus) = &coach.focus {
        for skill in focus.mental_focus.iter() {
            match skill {
                MentalFocusType::Aggression => {
                    if player.skills.mental.aggression < 20.0 {
                        let increase_amount = base_increase_amount
                            * MENTAL_SKILL_INCREASE_SPEED_MAP[&MentalSkill::Aggression];

                        skills_to_increase.push((MentalSkill::Aggression, increase_amount));
                    }
                }
                MentalFocusType::Anticipation => {
                    if player.skills.mental.anticipation < 20.0 {
                        let increase_amount = base_increase_amount
                            * MENTAL_SKILL_INCREASE_SPEED_MAP[&MentalSkill::Anticipation];

                        skills_to_increase.push((MentalSkill::Anticipation, increase_amount));
                    }
                }
                MentalFocusType::Bravery => {
                    if player.skills.mental.bravery < 20.0 {
                        let increase_amount = base_increase_amount
                            * MENTAL_SKILL_INCREASE_SPEED_MAP[&MentalSkill::Bravery];

                        skills_to_increase.push((MentalSkill::Bravery, increase_amount));
                    }
                }
                MentalFocusType::Composure => {
                    if player.skills.mental.composure < 20.0 {
                        let increase_amount = base_increase_amount
                            * MENTAL_SKILL_INCREASE_SPEED_MAP[&MentalSkill::Composure];

                        skills_to_increase.push((MentalSkill::Composure, increase_amount));
                    }
                }
                MentalFocusType::Concentration => {
                    if player.skills.mental.concentration < 20.0 {
                        let increase_amount = base_increase_amount
                            * MENTAL_SKILL_INCREASE_SPEED_MAP[&MentalSkill::Concentration];

                        skills_to_increase.push((MentalSkill::Concentration, increase_amount));
                    }
                }
                MentalFocusType::Decisions => {
                    if player.skills.mental.decisions < 20.0 {
                        let increase_amount = base_increase_amount
                            * MENTAL_SKILL_INCREASE_SPEED_MAP[&MentalSkill::Decisions];

                        skills_to_increase.push((MentalSkill::Decisions, increase_amount));
                    }
                }
                MentalFocusType::Determination => {
                    if player.skills.mental.determination < 20.0 {
                        let increase_amount = base_increase_amount
                            * MENTAL_SKILL_INCREASE_SPEED_MAP[&MentalSkill::Determination];

                        skills_to_increase.push((MentalSkill::Determination, increase_amount));
                    }
                }
                MentalFocusType::Flair => {
                    if player.skills.mental.flair < 20.0 {
                        let increase_amount = base_increase_amount
                            * MENTAL_SKILL_INCREASE_SPEED_MAP[&MentalSkill::Flair];

                        skills_to_increase.push((MentalSkill::Flair, increase_amount));
                    }
                }
                MentalFocusType::Leadership => {
                    if player.skills.mental.leadership < 20.0 {
                        let increase_amount = base_increase_amount
                            * MENTAL_SKILL_INCREASE_SPEED_MAP[&MentalSkill::Leadership];

                        skills_to_increase.push((MentalSkill::Leadership, increase_amount));
                    }
                }
                MentalFocusType::OffTheBall => {}
                MentalFocusType::Positioning => {}
                MentalFocusType::Teamwork => {}
                MentalFocusType::Vision => {}
                MentalFocusType::WorkRate => {}
            }
        }
    }

    skills_to_increase
}

pub fn determine_mental_skills_increase_amount(
    now: NaiveDate,
    weeks_since_last_training: u32,
    player: &Player,
    coach: &Staff,
) -> f32 {
    let base_increase =
        determine_base_value_to_skill_increase(now, weeks_since_last_training, player, coach);

    let weeks_since_last_training_factor = weeks_since_last_training as f32 * 0.1;

    let increase_amount = base_increase * weeks_since_last_training_factor;

    let coaching_skills = [coach.staff_attributes.coaching.mental];

    let coaching_skills_average =
        coaching_skills.iter().sum::<u8>() as f32 / coaching_skills.len() as f32;

    let skill_increase_amount =
        ((player.skills.mental.determination + coaching_skills_average) / 2.0) / 20.0;

    increase_amount + skill_increase_amount * 0.2
}
