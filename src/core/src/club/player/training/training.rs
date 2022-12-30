use crate::training::result::PlayerTrainingResult;
use crate::training::skills::{
    determine_mental_skills_to_increase, determine_physical_skills_to_increase,
    determine_technical_skills_to_increase,
};
use crate::{
    MentalFocusType, Player, PlayerTrainingHistory, Staff, TechnicalFocusType, TrainingNetLoader,
    TrainingRecord,
};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct PlayerTraining {}

impl PlayerTraining {
    pub fn new() -> Self {
        PlayerTraining {}
    }

    pub fn train(player: &Player, coach: &Staff, now: NaiveDateTime) -> PlayerTrainingResult {
        let mut result = PlayerTrainingResult::new(player.id);

        let training_history = &player.training_history;

        let weeks_since_last_training = training_history.weeks_since_last_training(now);

        // technical
        result
            .technical
            .skill_increase
            .extend(determine_technical_skills_to_increase(
                weeks_since_last_training,
                coach,
                player,
            ));

        result
            .mental
            .skill_increase
            .extend(determine_mental_skills_to_increase(
                weeks_since_last_training,
                coach,
                player,
            ));

        result
            .physical
            .skill_increase
            .extend(determine_physical_skills_to_increase(
                weeks_since_last_training,
                coach,
                player,
            ));

        result
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
