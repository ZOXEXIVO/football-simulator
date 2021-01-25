use crate::league::{
    LeagueSettings, Schedule, ScheduleError, ScheduleGenerator, ScheduleItem, ScheduleItemResult,
    ScheduleTour, Season,
};
use crate::utils::DateUtils;
use chrono::prelude::*;
use chrono::Duration;
use chrono::NaiveDate;
use log::{debug, warn};

// const DAY_PLAYING_TIMES: [(u8, u8); 4] = [(13, 0), (14, 0), (16, 0), (18, 0)];

pub struct RoundSchedule;

impl RoundSchedule {
    pub fn new() -> Self {
        RoundSchedule {}
    }
}

impl ScheduleGenerator for RoundSchedule {
    fn generate(
        &self,
        league_id: u32,
        season: Season,
        teams: &[u32],
        league_settings: &LeagueSettings,
    ) -> Result<Schedule, ScheduleError> {
        let teams_len = teams.len();

        if teams_len == 0 {
            warn!("schedule: team_len is empty. skip generation");
            ScheduleError::from_str("team_len is empty");
        }

        let tours_count = (teams_len * teams_len - teams_len) / (teams_len / 2);

        debug!(
            "schedule: team_len = {}, tours_count = {}",
            teams_len, tours_count
        );

        let (season_year_start, season_year_end) = match season {
            Season::OneYear(year) => (year, year),
            Season::TwoYear(start_year, end_year) => (start_year, end_year),
        };

        let current_date = DateUtils::get_next_saturday(NaiveDate::from_ymd(
            season_year_start as i32,
            league_settings.season_starting_half.from_month as u32,
            league_settings.season_starting_half.from_day as u32,
        ));

        let mut result = Schedule::with_tours_capacity((teams_len / 2) * tours_count);

        for item in generate_tours(league_id, teams, current_date) {
            result.tours.push(item);
        }

        Ok(result)
    }
}

fn generate_tours(
    league_id: u32,
    teams: &[u32],
    mut current_date: NaiveDateTime,
) -> Vec<ScheduleTour> {
    let team_len = teams.len() as u32;
    let games_count = (team_len / 2) as usize;

    let tours_count = ((team_len * team_len - team_len) / (team_len / 2)) as usize;

    let mut result = Vec::with_capacity(tours_count);

    let mut games_offset = 0;

    let games = generate_game_pairs(teams, tours_count);

    for tour in 1..tours_count {
        let mut tour = ScheduleTour::new(tour as u8, games_count);

        for game_idx in 0..games_count {
            let (home_team_id, away_team_id) = games[games_offset + game_idx as usize];

            tour.items.push(ScheduleItem::new(
                league_id,
                home_team_id,
                away_team_id,
                current_date,
            ));

            debug!(
                "date = {}, home_team_id = {}, away_team_id = {}",
                current_date, home_team_id, away_team_id
            );
        }

        games_offset += games_count;
        current_date += Duration::days(7);

        result.push(tour);
    }

    result
}

fn generate_game_pairs(teams: &[u32], tours_count: usize) -> Vec<(u32, u32)> {
    let mut result = Vec::new();

    let mut temp_vec = Vec::new();

    let team_len = teams.len() as u32;
    let team_len_half = team_len / 2 as u32;

    for team in 1..team_len_half + 1 {
        temp_vec.push((teams[team as usize], teams[(team_len - team) as usize]))
    }

    for team in &temp_vec {
        result.push((team.0, team.1));
    }

    for _ in 0..tours_count {
        rotate(&mut temp_vec);

        for team in &temp_vec {
            result.push((team.0, team.1));
        }
    }

    result
}

fn rotate(clubs: &mut Vec<(u32, u32)>) {
    let teams_len = clubs.len();

    let right_top = clubs[0].1;
    let left_bottom = clubs[teams_len - 1].0;

    for i in 0..teams_len - 1 {
        clubs[i].1 = clubs[i + 1].1;
        clubs[teams_len - i - 1].0 = clubs[teams_len - i - 2].0;
    }

    clubs[0].0 = right_top;
    clubs[teams_len - 1].1 = left_bottom;
}
