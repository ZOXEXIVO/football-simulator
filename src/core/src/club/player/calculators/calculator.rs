use crate::{Person, Player};
use chrono::NaiveDate;

pub struct PlayerValueCalculator;

impl PlayerValueCalculator {
    pub fn calculate(player: &Player, date: NaiveDate) -> f64 {
        if player.contract.is_none() {
            return 0.0;
        }

        // TODO improve value calculation
        // Use player reputation + club/league reputation

        let player_age = player.age(date);

        let base_price = match player_age {
            1..=17 => 100_000,
            18..=25 => 10_000_000,
            26..=30 => 8_000_000,
            31..=35 => 2_000_000,
            36..=50 => 100_000,
            _ => 10000,
        } as f64;

        let age_rate = match player.age(date) {
            1..=17 => 0.95,
            18..=25 => 1.0,
            26..=30 => 0.9,
            31..=35 => 0.6,
            36..=50 => 0.3,
            _ => 0.1,
        };

        base_price * age_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_is_correct() {}
}
