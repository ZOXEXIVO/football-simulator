use crate::common::NeuralNetwork;
use crate::utils::IntegerUtils;
use crate::{
    PersonBehaviourState, Physical, Player, PlayerSkills, PlayerTrainingHistory,
    PlayerTrainingResult, Staff, TrainingNetLoader, TrainingRecord,
};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
pub struct PlayerTraining {
    training_net: NeuralNetwork,
}

impl PlayerTraining {
    pub fn new() -> Self {
        PlayerTraining {
            training_net: TrainingNetLoader::load(),
        }
    }

    pub fn train(
        &self,
        player: &Player,
        coach: &Staff,
        now: NaiveDateTime,
    ) -> PlayerTrainingResult {
        let mut result = PlayerTrainingResult::new();

        result
        //
        // let training_history = &player.training_history;
        // let skills = &mut player.skills;
        //
        // // Вычисляем среднее значение скиллов на которые рассчитывает тренер
        // let technical_mean = determine_mean_skill(coach.technical_focus, &skills.technical);
        // let mental_mean = determine_mean_skill(coach.mental_focus, &skills.mental);
        // let physical_mean = determine_mean_skill(coach.physical_focus, &skills.physical);
        //
        // // Рассчитываем какие скиллы увеличивать
        // let technical_skills_to_increase =
        //     determine_skills_to_increase(technical_mean, &skills.technical);
        // let mental_skills_to_increase = determine_skills_to_increase(mental_mean, &skills.mental);
        // let physical_skills_to_increase =
        //     determine_skills_to_increase(physical_mean, &skills.physical);
        //
        // // Увеличиваем скиллы
        // increase_skills(technical_skills_to_increase, &mut skills.technical);
        // increase_skills(mental_skills_to_increase, &mut skills.mental);
        // increase_skills(physical_skills_to_increase, &mut skills.physical);
        //
        // // Сохраняем историю тренировки
        // let training_record = TrainingRecord {
        //     date: now,
        //     technical_skills_to_increase,
        //     mental_skills_to_increase,
        //     physical_skills_to_increase,
        // };
        //
        // training_history.records.push(training_record);
        //
        // result
    }
}

// fn determine_mean_skill(focus: Focus, skills: &Skills) -> f64 {
//     let mut sum = 0;
//     let mut count = 0;
//
//     for (_, skill_value) in skills {
//         sum += *skill_value as f64;
//         count += 1;
//     }
//
//     let mean = sum / count as f64;
//
//     match focus {
//         Focus::Low => mean / 2.0,
//         Focus::Medium => mean,
//         Focus::High => mean * 2.0,
//     }
// }
//
// fn determine_skills_to_increase(
//     player: &Player,
//     coach: &Staff,
//     training_history: &PlayerTrainingHistory,
//     current_date: NaiveDate,
// ) -> Vec<SkillToIncrease> {
//     let mut skills_to_increase = Vec::new();
//
//     let physical_form = player
//         .training
//         .determine_current_physical_form(training_history, current_date);
//
//     let mental_form = player
//         .training
//         .determine_current_mental_form(training_history, current_date);
//
//     if player.positions.is_technical() {
//         skills_to_increase.extend(determine_technical_skills_to_increase(
//             player,
//             coach,
//             physical_form,
//             mental_form,
//         ));
//     }
//
//     if player.positions.is_physical() {
//         skills_to_increase.extend(determine_physical_skills_to_increase(
//             player,
//             coach,
//             physical_form,
//             mental_form,
//         ));
//     }
//
//     if player.positions.is_mental() {
//         skills_to_increase.extend(determine_mental_skills_to_increase(
//             player,
//             coach,
//             physical_form,
//             mental_form,
//         ));
//     }
//
//     skills_to_increase
// }
//
// fn increase_skills(skills: &mut PlayerSkills, skills_to_increase: &[SkillToIncrease]) {
//     for skill in skills_to_increase {
//         match skill.area {
//             SkillArea::Technical => {
//                 skills.technical[skill.name] += skill.amount;
//             }
//             SkillArea::Mental => {
//                 skills.mental[skill.name] += skill.amount;
//             }
//             SkillArea::Physical => {
//                 skills.physical[skill.name] += skill.amount;
//             }
//         }
//     }
// }
//
// fn determine_physical_skills_to_increase(coach: &Coach, player: &Player) -> Vec<PhysicalSkill> {
//     let mut skills_to_increase = Vec::new();
//
//     if coach.physical_focus.acceleration {
//         skills_to_increase.push(PhysicalSkill::Acceleration);
//     }
//     if coach.physical_focus.agility {
//         skills_to_increase.push(PhysicalSkill::Agility);
//     }
//     // и так далее для остальных скиллов физической составляющей
//
//     skills_to_increase
// }
//
// fn determine_technical_skills_to_increase(coach: &Staff, player: &Player) -> Vec<TechnicalSkill> {
//     let mut skills_to_increase = Vec::new();
//
//     let coach_focus = coach.technical_focus;
//     let player_skills = &player.skills.technical;
//
//     let mut skill_names = Vec::new();
//     skill_names.extend(TechnicalSkill::iter());
//
//     // Сортируем список скиллов в соответствии с фокусом тренера
//     skill_names.sort_by_key(|skill| match coach_focus {
//         TechnicalFocus::Corners => skill.corner_factor(),
//         TechnicalFocus::Crossing => skill.crossing_factor(),
//         TechnicalFocus::Dribbling => skill.dribbling_factor(),
//         TechnicalFocus::Finishing => skill.finishing_factor(),
//         TechnicalFocus::FirstTouch => skill.first_touch_factor(),
//         TechnicalFocus::FreeKickTaking => skill.free_kick_taking_factor(),
//         TechnicalFocus::Heading => skill.heading_factor(),
//         TechnicalFocus::LongShots => skill.long_shots_factor(),
//         TechnicalFocus::LongThrows => skill.long_throws_factor(),
//         TechnicalFocus::Marking => skill.marking_factor(),
//         TechnicalFocus::Passing => skill.passing_factor(),
//         TechnicalFocus::PenaltyTaking => skill.penalty_taking_factor(),
//         TechnicalFocus::Tackling => skill.tackling_factor(),
//         TechnicalFocus::Technique => skill.technique_factor(),
//     });
//
//     // Добавляем скиллы в список увеличения только если у игрока они ниже среднего уровня
//     for skill in skill_names {
//         let skill_value = player_skills[skill];
//         if skill_value < AVERAGE_SKILL_LEVEL {
//             skills_to_increase.push(skill);
//         }
//     }
//
//     skills_to_increase
// }
//
// fn determine_mental_skills_to_increase(coach: &Staff, player: &Player) -> Vec<MentalSkill> {
//     let mut result = Vec::new();
//
//     let coach_mental_focus = coach.mental_focus;
//
//     for skill in MentalSkill::iter() {
//         if coach_mental_focus[*skill] {
//             result.push(*skill);
//         }
//     }
//
//     result
// }
//
// #[derive(Debug, PartialEq)]
// pub enum MentalSkill {
//     Aggression,
//     Anticipation,
//     Bravery,
//     Composure,
//     Concentration,
//     Decisions,
//     Determination,
//     Flair,
//     Leadership,
//     OffTheBall,
//     Positioning,
//     Teamwork,
//     Vision,
//     WorkRate,
// }
//
// struct SkillToIncrease {
//     area: SkillArea,
//     name: &'static str,
//     amount: u8,
// }
//
// enum SkillArea {
//     Technical,
//     Mental,
//     Physical,
// }
//
// #[derive(Debug, Clone, Copy)]
// pub enum TechnicalSkill {
//     Corners,
//     Crossing,
//     Dribbling,
//     Finishing,
//     FirstTouch,
//     FreeKickTaking,
//     Heading,
//     LongShots,
//     LongThrows,
//     Marking,
//     Passing,
//     PenaltyTaking,
//     Tackling,
//     Technique,
// }
//
// #[derive(Debug, PartialEq)]
// pub enum PhysicalSkill {
//     Acceleration,
//     Agility,
//     Balance,
//     JumpingReach,
//     NaturalFitness,
//     Pace,
//     Stamina,
//     Strength,
// }
