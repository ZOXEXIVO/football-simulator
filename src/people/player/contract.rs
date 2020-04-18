use crate::people::Player;
use crate::simulator::context::GlobalContext;
use crate::simulator::SimulationContext;
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

    pub fn simulate(&mut self, ctx: GlobalContext) {
        for player in &mut self.players {
            player.simulate(ctx.with_player());
        }
    }

    pub fn add(&mut self, players: Vec<Player>){
        for player in players {
            self.players.push(player);
        }
    }
    
    pub fn players(&self) -> Vec<&Player> {
        self.players.iter().map(|player| player).collect()
    }
    
    pub fn take(&mut self, player_id: u32) -> Player{
        let player_idx = self.players.iter().position(|p| p.id == player_id).unwrap();
        self.players.remove(player_idx)
    }
}
