use crate::context::GlobalContext;
use crate::league::LeagueResult;
use crate::utils::Logging;
use crate::Country;

pub struct CountryLeagueProcessor;

impl CountryLeagueProcessor {
    pub fn process(country: &mut Country, ctx: &GlobalContext<'_>) -> Vec<LeagueResult> {
        let teams_ids: Vec<(u32, u32)> = country
            .clubs
            .iter()
            .flat_map(|c| &c.teams)
            .map(|c| (c.id, c.league_id))
            .collect();

        country
            .leagues
            .iter_mut()
            .map(|league| {
                let league_team_ids: Vec<u32> = teams_ids
                    .iter()
                    .filter(|(_, league_id)| *league_id == league.id)
                    .map(|(id, _)| *id)
                    .collect();
                {
                    let message = &format!("simulate league: {}", &league.name);
                    Logging::estimate_result(
                        || league.simulate(ctx.with_league(league.id, &league_team_ids)),
                        message,
                    )
                }
            })
            .collect()
    }
}
