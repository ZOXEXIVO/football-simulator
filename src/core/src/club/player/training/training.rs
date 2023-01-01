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
        let now = now.date();

        let mut result = PlayerTrainingResult::new(player.id);

        let training_history = &player.training_history;

        let weeks_since_last_training = training_history.weeks_since_last_training(now);

        // technical
        result
            .technical
            .skill_increase
            .extend(determine_technical_skills_to_increase(
                now,
                weeks_since_last_training,
                coach,
                player,
            ));

        result
            .mental
            .skill_increase
            .extend(determine_mental_skills_to_increase(
                now,
                weeks_since_last_training,
                coach,
                player,
            ));

        result
            .physical
            .skill_increase
            .extend(determine_physical_skills_to_increase(
                now,
                weeks_since_last_training,
                coach,
                player,
            ));

        result
    }
}
