use crate::{MentalFocusType, MentalSkill, Player, Staff, TechnicalFocusType, TechnicalSkill};

pub fn determine_technical_skills_to_increase(
    weeks_since_last_training: u32,
    coach: &Staff,
    player: &Player,
) -> Vec<(TechnicalSkill, f32)> {
    let mut skills_to_increase = Vec::new();

    if let Some(focus) = &coach.focus {
        for skill in focus.technical_focus.iter() {
            match skill {
                TechnicalFocusType::Corners => {
                    if player.skills.technical.corners < 20.0 {
                        skills_to_increase.push((TechnicalSkill::Corners, 0.0f32));
                    }
                }
                TechnicalFocusType::Crossing => {
                    if player.skills.technical.crossing < 20.0 {
                        skills_to_increase.push((TechnicalSkill::Crossing, 0.0f32));
                    }
                }
                TechnicalFocusType::Dribbling => {
                    if player.skills.technical.dribbling < 20.0 {
                        skills_to_increase.push((TechnicalSkill::Dribbling, 0.0f32));
                    }
                }
                TechnicalFocusType::Finishing => {
                    if player.skills.technical.finishing < 20.0 {
                        skills_to_increase.push((TechnicalSkill::Finishing, 0.0f32));
                    }
                }
                TechnicalFocusType::FirstTouch => {
                    if player.skills.technical.first_touch < 20.0 {
                        skills_to_increase.push((TechnicalSkill::FirstTouch, 0.0f32));
                    }
                }
                TechnicalFocusType::FreeKicks => {
                    if player.skills.technical.free_kicks < 20.0 {
                        skills_to_increase.push((TechnicalSkill::FreeKicks, 0.0f32));
                    }
                }
                TechnicalFocusType::Heading => {
                    if player.skills.technical.heading < 20.0 {
                        skills_to_increase.push((TechnicalSkill::Heading, 0.0f32));
                    }
                }
                TechnicalFocusType::LongShots => {
                    if player.skills.technical.long_shots < 20.0 {
                        skills_to_increase.push((TechnicalSkill::LongShots, 0.0f32));
                    }
                }
                TechnicalFocusType::LongThrows => {
                    if player.skills.technical.long_throws < 20.0 {
                        skills_to_increase.push((TechnicalSkill::LongThrows, 0.0f32));
                    }
                }
                TechnicalFocusType::Marking => {
                    if player.skills.technical.marking < 20.0 {
                        skills_to_increase.push((TechnicalSkill::Marking, 0.0f32));
                    }
                }
                TechnicalFocusType::Passing => {
                    if player.skills.technical.passing < 20.0 {
                        skills_to_increase.push((TechnicalSkill::Passing, 0.0f32));
                    }
                }
                TechnicalFocusType::PenaltyTaking => {
                    if player.skills.technical.penalty_taking < 20.0 {
                        skills_to_increase.push((TechnicalSkill::PenaltyTaking, 0.0f32));
                    }
                }
                TechnicalFocusType::Tackling => {
                    if player.skills.technical.tackling < 20.0 {
                        skills_to_increase.push((TechnicalSkill::Tackling, 0.0f32));
                    }
                }
                TechnicalFocusType::Technique => {
                    if player.skills.technical.technique < 20.0 {
                        skills_to_increase.push((TechnicalSkill::Technique, 0.0f32));
                    }
                }
            }
        }
    }

    skills_to_increase
}

// pub fn determine_mental_skills_to_increase(coach: &Staff, player: &Player) -> Vec<MentalSkill> {
//     let mut skills_to_increase = Vec::new(, 0.0f32));
//
//     if let Some(focus) = &coach.focus {
//         for skill in focus.mental_focus.iter() {
//             match skill {
//                 MentalFocusType::WorkRate => {
//                     if player.skills.mental.work_rate < 20.0 {
//                         skills_to_increase.push((MentalSkill::WorkRate, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Determination => {
//                     if player.skills.mental.determination < 20.0 {
//                         skills_to_increase.push((MentalSkill::Determination, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Flair => {
//                     if player.skills.mental.flair < 20.0 {
//                         skills_to_increase.push((MentalSkill::Flair, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Leadership => {
//                     if player.skills.mental.leadership < 20.0 {
//                         skills_to_increase.push((MentalSkill::Leadership, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::OffTheBall => {
//                     if player.skills.mental.off_the_ball < 20.0 {
//                         skills_to_increase.push((MentalSkill::OffTheBall, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Positioning => {
//                     if player.skills.mental.positioning < 20.0 {
//                         skills_to_increase.push((MentalSkill::Positioning, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Teamwork => {
//                     if player.skills.mental.teamwork < 20.0 {
//                         skills_to_increase.push((MentalSkill::Teamwork, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Vision => {
//                     if player.skills.mental.vision < 20.0 {
//                         skills_to_increase.push((MentalSkill::Vision, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Anticipation => {
//                     if player.skills.mental.anticipation < 20.0 {
//                         skills_to_increase.push((MentalSkill::Anticipation, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Bravery => {
//                     if player.skills.mental.bravery < 20.0 {
//                         skills_to_increase.push((MentalSkill::Bravery, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Composure => {
//                     if player.skills.mental.composure < 20.0 {
//                         skills_to_increase.push((MentalSkill::Composure, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Concentration => {
//                     if player.skills.mental.concentration < 20.0 {
//                         skills_to_increase.push((MentalSkill::Concentration, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Decisions => {
//                     if player.skills.mental.decisions < 20.0 {
//                         skills_to_increase.push((MentalSkill::Decisions, 0.0f32));
//                     }
//                 }
//                 MentalFocusType::Aggression => {
//                     if player.skills.mental.aggression < 20.0 {
//                         skills_to_increase.push((MentalSkill::Aggression, 0.0f32));
//                     }
//                 }
//             }
//         }
//     }
//
//     skills_to_increase
// }

pub fn determine_technical_skills_increase_amount(
    weeks_since_last_training: u32,
    player: &Player,
    coach: &Staff,
) -> f32 {
    let base_increase = 0.1;
    let weeks_since_last_training_factor = 1.0 + weeks_since_last_training as f32 * 0.1;

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
        (player.skills.mental.determination + coaching_skills_average) / 2.0;

    let potential_ability_factor = player.player_attributes.potential_ability as f32 / 200.0;
    let current_ability_factor = player.player_attributes.current_ability as f32 / 200.0;

    increase_amount + skill_increase_amount * potential_ability_factor * current_ability_factor
}

// fn increase_technical_skills(
//     weeks_since_last_training: u32,
//     player: &Player,
//     coach: &Staff,
//     skills_to_improve: Vec<TechnicalSkill>,
// ) {
//     let improvement_amount =
//         determine_skills_increase_amount(weeks_since_last_training, player, coach, 0.0f32));
//
//     for skill in skills_to_improve {
//         match skill {
//             TechnicalSkill::Corners => {
//                 player.skills.technical.corners += improvement_amount;
//             }
//             TechnicalSkill::Crossing => {
//                 player.skills.technical.crossing += improvement_amount;
//             }
//             TechnicalSkill::Dribbling => {
//                 player.skills.technical.dribbling += improvement_amount;
//             }
//             TechnicalSkill::Finishing => {
//                 player.skills.technical.finishing += improvement_amount;
//             }
//             TechnicalSkill::FirstTouch => {
//                 player.skills.technical.first_touch += improvement_amount;
//             }
//             TechnicalSkill::FreeKicks => {
//                 player.skills.technical.free_kicks += improvement_amount;
//             }
//             TechnicalSkill::Heading => {
//                 player.skills.technical.heading += improvement_amount;
//             }
//             TechnicalSkill::LongShots => {
//                 player.skills.technical.long_shots += improvement_amount;
//             }
//             TechnicalSkill::LongThrows => {
//                 player.skills.technical.long_throws += improvement_amount;
//             }
//             TechnicalSkill::Marking => {
//                 player.skills.technical.marking += improvement_amount;
//             }
//             TechnicalSkill::Passing => {
//                 player.skills.technical.passing += improvement_amount;
//             }
//             TechnicalSkill::PenaltyTaking => {
//                 player.skills.technical.penalty_taking += improvement_amount;
//             }
//             TechnicalSkill::Tackling => {
//                 player.skills.technical.tackling += improvement_amount;
//             }
//             TechnicalSkill::Technique => {
//                 player.skills.technical.technique += improvement_amount;
//             }
//         }
//     }
// }
