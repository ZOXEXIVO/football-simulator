use crate::club::ClubContext;
use crate::continent::ContinentContext;
use crate::core::context::GlobalContext;
use crate::core::SimulationContext;
use crate::country::CountryContext;
use crate::league::LeagueContext;
use crate::people::{Player, PlayerContext};
pub use chrono::prelude::{DateTime, Datelike, NaiveDate, Utc};

#[derive(Debug)]
pub struct PlayerClubContract {
    pub salary: f64,
    pub expired: NaiveDate,
    pub additional_options: AdditionalOptions,
}

#[derive(Debug)]
pub struct AdditionalOptions {
    pub yearly_increase_wage: u16,
}

impl PlayerClubContract {
    pub fn new(expired: NaiveDate) -> Self {
        PlayerClubContract {
            salary: 100_000.0,
            expired,
            additional_options: AdditionalOptions {
                yearly_increase_wage: 15,
            },
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now();

        let naive_now = NaiveDate::from_ymd(now.year(), now.month(), now.day());

        self.expired >= naive_now
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        if context.check_contract_expiration() && self.is_expired() {}
    }
}

#[derive(Debug)]
pub struct PlayerCollection {
    pub players: Vec<Player>,
}

impl PlayerCollection {
    pub fn new(players: Vec<Player>) -> Self {
        PlayerCollection { players }
    }

    pub fn len(&self) -> usize {
        self.players.len()
    }

    pub fn simulate(&mut self, ctx: &mut GlobalContext) {
        for player in &mut self.players {
            player.simulate(ctx);
        }
    }

    pub fn players(&self) -> Vec<&Player> {
        self.players.iter().map(|player| player).collect()
    }
}
