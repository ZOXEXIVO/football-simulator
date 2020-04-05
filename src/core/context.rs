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

    pub fn with_continent(&mut self, continent_ctx: ContinentContext) -> &mut Self {
        self.continent = Some(continent_ctx);
        self
    }

    pub fn with_country(&mut self, country_ctx: CountryContext) -> &mut Self {
        self.country = Some(country_ctx);
        self
    }

    pub fn with_league(&mut self, league_ctx: LeagueContext) -> &mut Self {
        self.league = Some(league_ctx);
        self
    }

    pub fn with_club(&mut self, club_ctx: ClubContext) -> &mut Self {
        self.club = Some(club_ctx);
        self
    }

    pub fn with_board(&mut self, board_ctx: BoardContext) -> &mut Self {
        self.board = Some(board_ctx);
        self
    }

    pub fn with_player(&mut self, player_ctx: PlayerContext) -> &mut Self {
        self.player = Some(player_ctx);
        self
    }

    pub fn with_staff(&mut self, staff_ctx: StaffContext) -> &mut Self {
        self.staff = Some(staff_ctx);
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
