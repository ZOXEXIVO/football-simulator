use crate::{Team, TeamTrainingResult};
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

        for player in &team.players.players {
            let training_result = player.train(coach, date);
            result.player_results.push(training_result);
        }

        result
    }
}
