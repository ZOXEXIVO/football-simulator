pub use chrono::prelude::*;

use crate::club::{BoardContext, ClubContext};
use crate::continent::ContinentContext;
use crate::country::CountryContext;
use crate::league::LeagueContext;
use crate::people::{PlayerContext, StaffContext};
use chrono::Duration;
use std::rc::Rc;

pub struct GlobalContext<'c> {
    pub simulation: Rc<&'c mut SimulationContext>,
    pub continent: Option<Rc<&'c mut ContinentContext>>,
    pub country: Option<Rc<&'c mut CountryContext>>,
    pub league: Option<Rc<&'c mut LeagueContext>>,
    pub club: Option<Rc<&'c mut ClubContext>>,
    pub board: Option<Rc<&'c mut BoardContext>>,
    pub player: Option<Rc<&'c mut PlayerContext>>,
    pub staff: Option<Rc<&'c mut StaffContext>>,
}

impl<'c> GlobalContext<'c> {
    pub fn new(simulation_ctx: &'c mut SimulationContext) -> Self {
        GlobalContext {
            simulation: Rc::new(simulation_ctx),
            continent: None,
            country: None,
            league: None,
            club: None,
            board: None,
            player: None,
            staff: None,
        }
    }

    pub fn with_continent(&mut self, continent_ctx: &'c mut ContinentContext) -> Self {
        GlobalContext {
            simulation: Rc::clone(&mut self.simulation),
            continent: Some(Rc::new(continent_ctx)),
            country: None,
            league: None,
            club: None,
            board: None,
            player: None,
            staff: None,
        }
    }

    pub fn with_country(&mut self, country_ctx: &'c mut CountryContext) -> Self {
        GlobalContext {
            simulation: Rc::clone(&mut self.simulation),
            continent: Some(Rc::clone(&mut self.continent.as_mut().unwrap())),
            country: Some(Rc::new(country_ctx)),
            league: None,
            club: None,
            board: None,
            player: None,
            staff: None,
        }
    }

    pub fn with_league(&mut self, league_ctx: &'c mut LeagueContext) -> Self {
        GlobalContext {
            simulation: Rc::clone(&mut self.simulation),
            continent: Some(Rc::clone(&mut self.continent.as_mut().unwrap())),
            country: Some(Rc::clone(&mut self.country.as_mut().unwrap())),
            league: Some(Rc::new(league_ctx)),
            club: None,
            board: None,
            player: None,
            staff: None,
        }
    }

    pub fn with_club(&mut self, club_ctx: &'c mut ClubContext) -> Self {
        GlobalContext {
            simulation: Rc::clone(&mut self.simulation),
            continent: Some(Rc::clone(&mut self.continent.as_mut().unwrap())),
            country: Some(Rc::clone(&mut self.country.as_mut().unwrap())),
            league: Some(Rc::clone(&mut self.league.as_mut().unwrap())),
            club: Some(Rc::new(club_ctx)),
            board: None,
            player: None,
            staff: None,
        }
    }

    pub fn with_player(&mut self, player_ctx: &'c mut PlayerContext) -> Self {
        GlobalContext {
            simulation: Rc::clone(&mut self.simulation),
            continent: Some(Rc::clone(&mut self.continent.as_mut().unwrap())),
            country: Some(Rc::clone(&mut self.country.as_mut().unwrap())),
            league: Some(Rc::clone(&mut self.league.as_mut().unwrap())),
            club: Some(Rc::clone(&mut self.club.as_mut().unwrap())),
            board: None,
            player: Some(Rc::new(player_ctx)),
            staff: None,
        }
    }

    pub fn with_staff(&mut self, staff_ctx: &'c mut StaffContext) -> Self {
        GlobalContext {
            simulation: Rc::clone(&mut self.simulation),
            continent: Some(Rc::clone(&mut self.continent.as_mut().unwrap())),
            country: Some(Rc::clone(&mut self.country.as_mut().unwrap())),
            league: Some(Rc::clone(&mut self.league.as_mut().unwrap())),
            club: Some(Rc::clone(&mut self.club.as_mut().unwrap())),
            board: None,
            player: Some(Rc::clone(&mut self.player.as_mut().unwrap())),
            staff: Some(Rc::new(staff_ctx)),
        }
    }

    pub fn with_board(&mut self, board_ctx: &'c mut BoardContext) -> Self {
        GlobalContext {
            simulation: Rc::clone(&mut self.simulation),
            continent: Some(Rc::clone(&mut self.continent.as_mut().unwrap())),
            country: Some(Rc::clone(&mut self.country.as_mut().unwrap())),
            league: Some(Rc::clone(&mut self.league.as_mut().unwrap())),
            club: Some(Rc::clone(&mut self.club.as_mut().unwrap())),
            board: Some(Rc::new(board_ctx)),
            player: Some(Rc::clone(&mut self.player.as_mut().unwrap())),
            staff: Some(Rc::clone(&mut self.staff.as_mut().unwrap())),
        }
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

    pub fn next_date(&mut self) {
        self.date += Duration::hours(1);

        self.day = self.date.day() as u8;
        self.hour = self.date.time().hour() as u8;
    }

    pub fn check_contract_expiration(&self) -> bool {
        self.hour == 0
    }
}
