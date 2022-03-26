use crate::training::history::PlayerTrainingHistory;
use crate::utils::IntegerUtils;
use crate::{PersonBehaviourState, Player, PlayerTeamTrainingResult, Staff};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct PlayerTraining {
    pub history: PlayerTrainingHistory,
}

impl PlayerTraining {
    pub fn new() -> Self {
        PlayerTraining {
            history: PlayerTrainingHistory::new(),
        }
    }

    pub fn train(
        player: &mut Player,
        coach: &Staff,
        date: NaiveDateTime,
    ) -> PlayerTeamTrainingResult {
        let result = PlayerTeamTrainingResult::new();

        match coach.behaviour.state {
            PersonBehaviourState::Good => {
                player
                    .skills
                    .mental
                    .train(IntegerUtils::random(-1, 1) as i8);
                player
                    .skills
                    .technical
                    .train(IntegerUtils::random(-1, 1) as i8)
            }
            PersonBehaviourState::Normal => player.skills.train(IntegerUtils::random(-1, 1) as i8),
            PersonBehaviourState::Poor => player
                .skills
                .physical
                .train(IntegerUtils::random(-1, 1) as i8),
        }

        result
    }
}
