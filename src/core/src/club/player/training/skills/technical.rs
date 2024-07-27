use crate::training::skills::determine_base_value_to_skill_increase;
use crate::{Player, Staff, TechnicalFocusType};
use chrono::NaiveDate;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TechnicalSkill {
    Corners,
    Crossing,
    Dribbling,
    Finishing,
    FirstTouch,
    FreeKicks,
    Heading,
    LongShots,
    LongThrows,
    Marking,
    Passing,
    PenaltyTaking,
    Tackling,
    Technique,
}


pub static TECHNICAL_SKILL_INCREASE_SPEED_MAP: LazyLock<HashMap<TechnicalSkill, f32>> = LazyLock::new(|| {
    vec![
        (TechnicalSkill::Corners, 0.1),
        (TechnicalSkill::Crossing, 0.2),
        (TechnicalSkill::Dribbling, 0.3),
        (TechnicalSkill::Finishing, 0.4),
        (TechnicalSkill::FirstTouch, 0.3),
        (TechnicalSkill::FreeKicks, 0.4),
        (TechnicalSkill::Heading, 0.3),
        (TechnicalSkill::LongShots, 0.4),
        (TechnicalSkill::LongThrows, 0.2),
        (TechnicalSkill::Marking, 0.1),
        (TechnicalSkill::Passing, 0.3),
        (TechnicalSkill::PenaltyTaking, 0.4),
        (TechnicalSkill::Tackling, 0.2),
        (TechnicalSkill::Technique, 0.3)
    ]
        .into_iter()
        .collect()
});


pub fn determine_technical_skills_to_increase(
    now: NaiveDate,
    weeks_since_last_training: u32,
    coach: &Staff,
    player: &Player,
) -> Vec<(TechnicalSkill, f32)> {
    let mut skills_to_increase = Vec::with_capacity(14);

    let base_increase_amount =
        determine_technical_skills_increase_amount(now, weeks_since_last_training, player, coach);

    if let Some(focus) = &coach.focus {
        for skill in focus.technical_focus.iter() {
            match skill {
                TechnicalFocusType::Corners => {
                    if player.skills.technical.corners < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::Corners];

                        skills_to_increase.push((TechnicalSkill::Corners, increase_amount));
                    }
                }
                TechnicalFocusType::Crossing => {
                    if player.skills.technical.crossing < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::Crossing];

                        skills_to_increase.push((TechnicalSkill::Crossing, increase_amount));
                    }
                }
                TechnicalFocusType::Dribbling => {
                    if player.skills.technical.dribbling < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::Dribbling];

                        skills_to_increase.push((TechnicalSkill::Dribbling, increase_amount));
                    }
                }
                TechnicalFocusType::Finishing => {
                    if player.skills.technical.finishing < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::Finishing];

                        skills_to_increase.push((TechnicalSkill::Finishing, increase_amount));
                    }
                }
                TechnicalFocusType::FirstTouch => {
                    if player.skills.technical.first_touch < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::FirstTouch];

                        skills_to_increase.push((TechnicalSkill::FirstTouch, increase_amount));
                    }
                }
                TechnicalFocusType::FreeKicks => {
                    if player.skills.technical.free_kicks < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::FreeKicks];

                        skills_to_increase.push((TechnicalSkill::FreeKicks, increase_amount));
                    }
                }
                TechnicalFocusType::Heading => {
                    if player.skills.technical.heading < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::Heading];

                        skills_to_increase.push((TechnicalSkill::Heading, increase_amount));
                    }
                }
                TechnicalFocusType::LongShots => {
                    if player.skills.technical.long_shots < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::LongShots];

                        skills_to_increase.push((TechnicalSkill::LongShots, increase_amount));
                    }
                }
                TechnicalFocusType::LongThrows => {
                    if player.skills.technical.long_throws < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::LongThrows];

                        skills_to_increase.push((TechnicalSkill::LongThrows, increase_amount));
                    }
                }
                TechnicalFocusType::Marking => {
                    if player.skills.technical.marking < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::Marking];

                        skills_to_increase.push((TechnicalSkill::Marking, increase_amount));
                    }
                }
                TechnicalFocusType::Passing => {
                    if player.skills.technical.passing < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::Passing];

                        skills_to_increase.push((TechnicalSkill::Passing, increase_amount));
                    }
                }
                TechnicalFocusType::PenaltyTaking => {
                    if player.skills.technical.penalty_taking < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::PenaltyTaking];

                        skills_to_increase.push((TechnicalSkill::PenaltyTaking, increase_amount));
                    }
                }
                TechnicalFocusType::Tackling => {
                    if player.skills.technical.tackling < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::Tackling];

                        skills_to_increase.push((TechnicalSkill::Tackling, increase_amount));
                    }
                }
                TechnicalFocusType::Technique => {
                    if player.skills.technical.technique < 20.0 {
                        let increase_amount = base_increase_amount
                            * TECHNICAL_SKILL_INCREASE_SPEED_MAP[&TechnicalSkill::Technique];

                        skills_to_increase.push((TechnicalSkill::Technique, increase_amount));
                    }
                }
            }
        }
    }

    skills_to_increase
}

pub fn determine_technical_skills_increase_amount(
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
        coach.staff_attributes.coaching.attacking,
        coach.staff_attributes.coaching.defending,
        coach.staff_attributes.coaching.tactical,
        coach.staff_attributes.coaching.technical,
    ];

    let coaching_skills_average =
        coaching_skills.iter().sum::<u8>() as f32 / coaching_skills.len() as f32;

    let skill_increase_amount =
        ((player.skills.mental.determination + coaching_skills_average) / 2.0) / 20.0;

    increase_amount + skill_increase_amount * 0.2
}
