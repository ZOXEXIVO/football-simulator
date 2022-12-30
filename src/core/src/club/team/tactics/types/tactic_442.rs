// use crate::types::{SkillPriority, Tactic};
// use crate::{MentalSkill, PhysicalSkill, PlayerPositionType, TechnicalSkill};
// use std::collections::HashMap;
//
// pub struct Tactic442 {}
//
// impl Tactic for Tactic442 {
//     fn get_player_positions(&self) -> Vec<PlayerPositionType> {
//         vec![
//             PlayerPositionType::Goalkeeper,
//             PlayerPositionType::DefenderLeft,
//             PlayerPositionType::DefenderCenter,
//             PlayerPositionType::DefenderCenter,
//             PlayerPositionType::DefenderRight,
//             PlayerPositionType::MidfielderLeft,
//             PlayerPositionType::MidfielderCenter,
//             PlayerPositionType::MidfielderCenter,
//             PlayerPositionType::MidfielderRight,
//             PlayerPositionType::Striker,
//             PlayerPositionType::Striker,
//         ]
//     }
//
//     fn get_skill_priority(&self) -> SkillPriority {
//         SkillPriority {
//             technical: {
//                 let mut map = HashMap::new();
//
//                 map.insert(TechnicalSkill::Passing, 0.5);
//                 map.insert(TechnicalSkill::Dribbling, 0.5);
//                 map.insert(TechnicalSkill::Shooting, 0.5);
//                 map.insert(TechnicalSkill::Heading, 0.5);
//                 map.insert(TechnicalSkill::Tackling, 0.5);
//                 map.insert(TechnicalSkill::Speed, 0.5);
//                 map.insert(TechnicalSkill::Acceleration, 0.5);
//                 map.insert(TechnicalSkill::Stamina, 0.5);
//                 map.insert(TechnicalSkill::Strength, 0.5);
//                 map.insert(TechnicalSkill::Agility, 0.5);
//                 map.insert(TechnicalSkill::Balance, 0.5);
//                 map.insert(TechnicalSkill::Jumping, 0.5);
//                 map.insert(TechnicalSkill::Reactions, 0.5);
//                 map.insert(TechnicalSkill::Aggression, 0.5);
//                 map.insert(TechnicalSkill::Interceptions, 0.5);
//                 map.insert(TechnicalSkill::Vision, 0.5);
//                 map.insert(TechnicalSkill::Composure, 0.5);
//                 map.insert(TechnicalSkill::Crossing, 0.5);
//                 map.insert(TechnicalSkill::LongShots, 0.5);
//                 map.insert(TechnicalSkill::Finishing, 0.5);
//                 map.insert(TechnicalSkill::LongPassing, 0.5);
//                 map.insert(TechnicalSkill::Marking, 0.0);
//
//                 map
//             },
//             mental: {
//                 let mut map = HashMap::new();
//
//                 map.insert(MentalSkill::Passing, 0.5);
//                 map.insert(MentalSkill::Dribbling, 0.5);
//                 map.insert(MentalSkill::Shooting, 0.5);
//                 map.insert(MentalSkill::Heading, 0.5);
//                 map.insert(MentalSkill::Tackling, 0.5);
//                 map.insert(MentalSkill::Speed, 0.5);
//                 map.insert(MentalSkill::Acceleration, 0.5);
//                 map.insert(MentalSkill::Stamina, 0.5);
//                 map.insert(MentalSkill::Strength, 0.5);
//                 map.insert(MentalSkill::Agility, 0.5);
//                 map.insert(MentalSkill::Balance, 0.5);
//                 map.insert(MentalSkill::Jumping, 0.5);
//                 map.insert(MentalSkill::Reactions, 0.5);
//                 map.insert(MentalSkill::Aggression, 0.5);
//                 map.insert(MentalSkill::Interceptions, 0.5);
//                 map.insert(MentalSkill::Vision, 0.5);
//                 map.insert(MentalSkill::Composure, 0.5);
//                 map.insert(MentalSkill::Crossing, 0.5);
//                 map.insert(MentalSkill::LongShots, 0.5);
//                 map.insert(MentalSkill::Finishing, 0.5);
//                 map.insert(MentalSkill::LongPassing, 0.5);
//                 map.insert(MentalSkill::Marking, 0.0);
//
//                 map
//             },
//             physical: {
//                 let mut map = HashMap::new();
//
//                 map.insert(PhysicalSkill::Passing, 0.5);
//                 map.insert(PhysicalSkill::Dribbling, 0.5);
//                 map.insert(PhysicalSkill::Shooting, 0.5);
//                 map.insert(PhysicalSkill::Heading, 0.5);
//                 map.insert(PhysicalSkill::Tackling, 0.5);
//                 map.insert(PhysicalSkill::Speed, 0.5);
//                 map.insert(PhysicalSkill::Acceleration, 0.5);
//                 map.insert(PhysicalSkill::Stamina, 0.5);
//                 map.insert(PhysicalSkill::Strength, 0.5);
//                 map.insert(PhysicalSkill::Agility, 0.5);
//                 map.insert(PhysicalSkill::Balance, 0.5);
//                 map.insert(PhysicalSkill::Jumping, 0.5);
//                 map.insert(PhysicalSkill::Reactions, 0.5);
//                 map.insert(PhysicalSkill::Aggression, 0.5);
//                 map.insert(PhysicalSkill::Interceptions, 0.5);
//                 map.insert(PhysicalSkill::Vision, 0.5);
//                 map.insert(PhysicalSkill::Composure, 0.5);
//                 map.insert(PhysicalSkill::Crossing, 0.5);
//                 map.insert(PhysicalSkill::LongShots, 0.5);
//                 map.insert(PhysicalSkill::Finishing, 0.5);
//                 map.insert(PhysicalSkill::LongPassing, 0.5);
//                 map.insert(PhysicalSkill::Marking, 0.0);
//
//                 map
//             },
//         }
//     }
// }
