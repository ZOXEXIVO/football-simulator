use crate::{Person, Player, PlayerPositions, PlayerStatus, PlayerStatusType};
use chrono::{Datelike, Local, NaiveDate};

pub struct PlayerValueCalculator;

// write calculate function

impl PlayerValueCalculator {
    pub fn calculate(player: &Player, now: NaiveDate) -> f64 {
        let base_value = Self::determine_base_value(player);
        let age_factor = Self::determine_age_factor(player, now);

        let status_factor = Self::determine_status_factor(player);

        let contract_factor = Self::determine_contract_factor(player, now);

        let country_factor = Self::determine_country_factor(player);

        let form_factor = Self::determine_form_factor(player);

        let match_appearance_factor = Self::determine_match_appearance_factor(player);

        let statistics_factor = Self::determine_statistics_factor(player);

        let other_factors = Self::determine_other_factors(player);

        let value = base_value
            * age_factor
            * status_factor
            * contract_factor
            * country_factor
            * form_factor
            * match_appearance_factor
            * statistics_factor
            * other_factors;

        value
    }

    fn determine_base_value(player: &Player) -> f64 {
        const BASE_PRICE: f64 = 1_000_000.0;

        let technical_skills = &player.skills.technical;
        let mental_skills = &player.skills.mental;
        let physical_skills = &player.skills.physical;

        let technical_mean = (technical_skills.corners
            + technical_skills.crossing
            + technical_skills.dribbling
            + technical_skills.finishing
            + technical_skills.first_touch
            + technical_skills.free_kick_taking
            + technical_skills.heading
            + technical_skills.long_shots
            + technical_skills.long_throws
            + technical_skills.marking
            + technical_skills.passing
            + technical_skills.penalty_taking
            + technical_skills.tackling
            + technical_skills.technique) as f64
            / 14.0;
        let mental_mean = (mental_skills.aggression
            + mental_skills.anticipation
            + mental_skills.bravery
            + mental_skills.composure
            + mental_skills.concentration
            + mental_skills.decisions
            + mental_skills.determination
            + mental_skills.flair
            + mental_skills.leadership
            + mental_skills.off_the_ball
            + mental_skills.positioning
            + mental_skills.teamwork
            + mental_skills.vision
            + mental_skills.work_rate) as f64
            / 14.0;
        let physical_mean = (physical_skills.acceleration
            + physical_skills.agility
            + physical_skills.balance
            + physical_skills.jumping_reach
            + physical_skills.natural_fitness
            + physical_skills.pace
            + physical_skills.stamina
            + physical_skills.strength) as f64
            / 8.0;

        let base_value = (technical_mean + mental_mean + physical_mean) / 3.0;

        BASE_PRICE * base_value
    }

    fn determine_age_factor(player: &Player, date: NaiveDate) -> f64 {
        match player.age(date) {
            age if age < 21 => 0.7,
            age if age < 25 => 0.8,
            age if age < 30 => 0.9,
            age if age < 35 => 0.7,
            age if age < 40 => 0.5,
            _ => 0.3,
        }
    }

    fn determine_status_factor(player: &Player) -> f64 {
        let statuses = player.statuses.get();

        let mut status_factor = 1.0f64;

        if statuses.contains(&PlayerStatusType::Inj) {
            status_factor *= 0.7;
        }

        if statuses.contains(&PlayerStatusType::Unh) {
            status_factor *= 0.8;
        }

        if statuses.contains(&PlayerStatusType::Loa) {
            status_factor *= 0.9;
        }

        status_factor
    }

    fn determine_contract_factor(player: &Player, date: NaiveDate) -> f64 {
        let contract = match &player.contract {
            Some(contract) => contract,
            None => return 0.0,
        };

        let remaining_years = (contract.expiration.year() as i32 - date.year() as i32) as f64;
        let contract_factor = match remaining_years {
            remaining_years if remaining_years > 2.0 => 1.0,
            remaining_years if remaining_years > 1.0 => 0.9,
            remaining_years if remaining_years > 0.5 => 0.8,
            _ => 0.7,
        };

        contract_factor
    }

    fn determine_country_factor(player: &Player) -> f64 {
        let country_factor = 1.0;
        country_factor
    }

    fn determine_form_factor(player: &Player) -> f64 {
        let form_factor = 1.0;

        // let form = match player.statistics.get_form(date) {
        //     Some(form) => form,
        //     None => return 1.0,
        // };
        //
        // let form_factor = match form {
        //     form if form > 8.0 => 1.1,
        //     form if form > 6.0 => 1.0,
        //     form if form > 4.0 => 0.9,
        //     form if form > 2.0 => 0.8,
        //     _ => 0.7,
        // };

        form_factor
    }

    fn determine_match_appearance_factor(player: &Player) -> f64 {
        let match_appearance_factor = 1.0;
        match_appearance_factor
    }

    fn determine_statistics_factor(player: &Player) -> f64 {
        let match_appearance_factor = match player.statistics.played {
            match_appearances if match_appearances > 20 => 1.1,
            match_appearances if match_appearances > 10 => 1.0,
            match_appearances if match_appearances > 5 => 0.9,
            match_appearances if match_appearances > 2 => 0.8,
            _ => 0.7,
        };

        match_appearance_factor
    }

    fn determine_other_factors(player: &Player) -> f64 {
        let mut other_factors = 1.0;

        if player.positions.is_goalkeeper() {
            other_factors *= 1.2;
        }

        if player.happiness.is_happy() {
            other_factors *= 1.1;
        }

        if player.attributes.loyalty > 8 {
            other_factors *= 1.1;
        }

        other_factors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_is_correct() {}
}
