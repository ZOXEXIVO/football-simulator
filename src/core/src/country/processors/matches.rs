use crate::league::{LeagueMatchResultResult, LeagueResult};
use crate::r#match::{Match, MatchResult};
use crate::utils::Logging;
use crate::{Country, Team};

pub struct CountryMatchProcessor;

impl CountryMatchProcessor {
    pub fn process(country: &mut Country, results: &mut Vec<LeagueResult>) -> Vec<MatchResult> {
        let mut result = Vec::new(); //TODO capacity

        for league_result in results {
            for scheduled_match in &mut league_result.scheduled_matches {
                let match_to_play = Match::make(
                    scheduled_match.league_id,
                    &scheduled_match.id,
                    Self::get_team(country, scheduled_match.home_team_id),
                    Self::get_team(country, scheduled_match.away_team_id),
                );

                let message = &format!(
                    "play match: {} - {}",
                    &match_to_play.home_team.name, &match_to_play.away_team.name
                );

                let match_result = Logging::estimate_result(|| match_to_play.play(), message);

                scheduled_match.result = Some(LeagueMatchResultResult {
                    home_goals: match_result.home_goals,
                    away_goals: match_result.away_goals,
                });

                result.push(match_result);
            }
        }

        result
    }

    pub fn get_team(country: &Country, id: u32) -> &Team {
        country
            .clubs
            .iter()
            .flat_map(|c| &c.teams)
            .find(|team| team.id == id)
            .unwrap()
    }
}
