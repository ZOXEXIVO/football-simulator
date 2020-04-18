pub use chrono::prelude::*;

use crate::club::{BoardContext, ClubContext};
use crate::continent::ContinentContext;
use crate::country::CountryContext;
use crate::league::LeagueContext;
use crate::people::{PlayerContext, StaffContext};

#[derive(Clone)]
pub struct GlobalContext {
    pub simulation: SimulationContext,
    pub continent: Option<ContinentContext>,
    pub country: Option<CountryContext>,
    pub league: Option<LeagueContext>,
    pub club: Option<ClubContext>,
    pub board: Option<BoardContext>,
    pub player: Option<PlayerContext>,
    pub staff: Option<StaffContext>
}

impl GlobalContext {
    pub fn new(simulation_ctx: SimulationContext) -> Self {
        GlobalContext {
            simulation: simulation_ctx,
            continent: None,
            country: None,
            league: None,
            club: None,
            board: None,
            player: None,
            staff: None,
        }
    }

    pub fn with_continent(&self, continent_id: u32) -> Self {
        let mut ctx = self.clone();
        ctx.continent = Some(ContinentContext::new(continent_id));
        ctx
    }

    pub fn with_country(&self, country_id: u32) -> Self {
        let mut ctx = self.clone();

        ctx.country = Some(CountryContext::new(country_id));
        ctx
    }

    pub fn with_league(&self, league_id: u32) -> Self {
        let mut ctx = self.clone();
        ctx.league = Some(LeagueContext::new(league_id));
        ctx
    }

    pub fn with_club(&self, club_id: u32) -> Self {
        let mut ctx = self.clone();
        ctx.club = Some(ClubContext::new(club_id));
        ctx
    }

    pub fn with_board(&self) -> Self {
        let mut ctx = self.clone();
        ctx.board = Some(BoardContext::new());
        ctx
    }

    pub fn with_player(&self, player_id: Option<u32>) -> Self {
        let mut ctx = self.clone();
        ctx.player = Some(PlayerContext::new(player_id));
        ctx
    }

    pub fn with_staff(&self) -> Self {
        let mut ctx = self.clone();
        ctx.staff = Some(StaffContext::new());
        ctx
    }
}

#[derive(Clone)]
pub struct SimulationContext {
    pub date: NaiveDateTime,
    pub day: u8,
    pub hour: u8,
}

impl SimulationContext {
    pub fn new(date: NaiveDateTime) -> Self {
        SimulationContext {
            date,
            day: 0,
            hour: 0,
        }
    }

    pub fn check_contract_expiration(&self) -> bool {
        self.hour == 0
    }
}
