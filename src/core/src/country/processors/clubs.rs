use crate::context::GlobalContext;
use crate::league::LeagueResult;
use crate::utils::Logging;
use crate::{ClubResult, Country};

pub struct CountryClubProcessor;

impl CountryClubProcessor {
    pub fn process(country: &mut Country, ctx: &GlobalContext<'_>) -> Vec<ClubResult> {
        country
            .clubs
            .iter_mut()
            .map(|club| {
                let message = &format!("simulate club: {}", &club.name);
                Logging::estimate_result(
                    || club.simulate(ctx.with_club(club.id, &club.name.clone())),
                    message,
                )
            })
            .collect()
    }
}
