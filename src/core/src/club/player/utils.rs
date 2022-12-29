use crate::{Person, Player, PlayerPositionType};
use chrono::NaiveDate;

pub struct PlayerUtils;

impl PlayerUtils {
    #[inline]
    pub fn growth_potential(player: &Player, now: NaiveDate) -> u8 {
        let age = player.age(now);
        let age_factor = Self::age_factor(age);

        let determination = player.skills.mental.determination as f32 / 20.0;
        let ambition = player.attributes.ambition as f32 / 20.0;
        let professionalism = player.attributes.professionalism as f32 / 20.0;
        let base_factor = determination + ambition + professionalism;

        let current_ability = player.player_attributes.current_ability as f32;
        let potential_ability = player.player_attributes.potential_ability as f32;
        let ability_factor = (potential_ability - current_ability) / 20.0;

        let condition = player.player_attributes.condition as f32 / 100.0;
        let fitness = player.player_attributes.fitness as f32 / 100.0;
        let jadedness = player.player_attributes.jadedness as f32 / 100.0;
        let physical_factor = (condition + fitness - jadedness) / 3.0;

        let reputation = player.player_attributes.current_reputation as f32 / 100.0;
        let international_factor = player.player_attributes.international_apps as f32 / 100.0;

        let total_factor = age_factor
            * base_factor
            * ability_factor
            * physical_factor
            * reputation
            * international_factor;

        let growth_potential = (total_factor * 5.0).round() as u8;
        growth_potential
    }

    fn age_factor(age: u8) -> f32 {
        let a = age as f32;
        let factor = 1.0 / (1.0 + (-0.1 * a).exp());
        factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        PersonAttributes, PersonBehaviour, PlayerAttributes, PlayerPositions, PlayerPreferredFoot,
        PlayerSkills, PlayerStatus,
    };
    use chrono::Utc;

    #[test]
    fn test_age_factor_under_24() {
        let age = 23;
        let factor = PlayerUtils::age_factor(age);
        assert!(factor > 0.9);
    }

    #[test]
    fn test_age_factor_24_29() {
        let age = 24;
        let factor = PlayerUtils::age_factor(age);
        assert!(factor > 0.8);
        assert!(factor < 0.9);
    }
}
