pub use chrono::prelude::*;

use crate::club::{BoardContext, ClubContext};
use crate::continent::ContinentContext;
use crate::country::CountryContext;
use crate::league::LeagueContext;
use crate::people::{PlayerContext, StaffContext};

pub struct GlobalContext {
    pub simulation: SimulationContext,
    pub continent: Option<ContinentContext>,
    pub country: Option<CountryContext>,
    pub league: Option<LeagueContext>,
    pub club: Option<ClubContext>,
    pub board: Option<BoardContext>,
    pub player: Option<PlayerContext>,
    pub staff: Option<StaffContext>,
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

    pub fn with_continent(&mut self) -> &mut Self {
        self.continent = Some(ContinentContext::new());
        self
    }

    pub fn with_country(&mut self) -> &mut Self {
        self.country = Some(CountryContext::new());
        self
    }

    pub fn with_league(&mut self) -> &mut Self {
        self.league = Some(LeagueContext::new());
        self
    }

    pub fn with_club(&mut self) -> &mut Self {
        self.club = Some(ClubContext::new());
        self
    }

    pub fn with_board(&mut self) -> &mut Self {
        self.board = Some(BoardContext::new());
        self
    }

    pub fn with_player(&mut self) -> &mut Self {
        self.player = Some(PlayerContext::new());
        self
    }

    pub fn with_staff(&mut self) -> &mut Self {
        self.staff = Some(StaffContext::new());
        self
    }

    pub fn continent(&mut self) -> &mut ContinentContext {
        self.continent.as_mut().unwrap()
    }

    pub fn country(&mut self) -> &mut CountryContext {
        self.country.as_mut().unwrap()
    }

    pub fn league(&mut self) -> &mut LeagueContext {
        self.league.as_mut().unwrap()
    }

    pub fn club(&mut self) -> &mut ClubContext {
        self.club.as_mut().unwrap()
    }

    pub fn board(&mut self) -> &mut BoardContext {
        self.board.as_mut().unwrap()
    }

    pub fn player(&mut self) -> &mut PlayerContext {
        self.player.as_mut().unwrap()
    }

    pub fn staff(&mut self) -> &mut StaffContext {
        self.staff.as_mut().unwrap()
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
