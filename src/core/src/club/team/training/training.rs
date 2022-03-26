use crate::club::{Player, Staff};
use crate::{PlayerTraining, Team, TeamTrainingResult};
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct TeamTraining {}

impl TeamTraining {
    pub fn train(team: &mut Team, date: NaiveDateTime) -> TeamTrainingResult {
        if team.training_schedule.is_time(date) {
            return TeamTrainingResult::empty();
        }

        let mut result = TeamTrainingResult::new();

        let coach = team.staffs.training_coach(&team.team_type);

        for player in team.players.players.iter_mut() {
            result
                .player_results
                .push(PlayerTraining::train(player, coach, date));
        }

        result
    }
}
