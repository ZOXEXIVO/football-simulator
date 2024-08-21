pub const CONDITION_MAX_VALUE: i16 = 10000;

#[derive(Debug, Clone, Copy)]
pub struct PlayerAttributes {
    pub is_banned: bool,
    pub is_injured: bool,

    pub condition: i16,
    pub fitness: i16,
    pub jadedness: i16,

    pub weight: u8,
    pub height: u8,

    pub value: u32,

    //reputation
    pub current_reputation: i16,
    pub home_reputation: i16,
    pub world_reputation: i16,

    //ability
    pub current_ability: u8,
    pub potential_ability: u8,

    //international expirience
    pub international_apps: u16,
    pub international_goals: u16,

    pub under_21_international_apps: u16,
    pub under_21_international_goals: u16,
}

impl PlayerAttributes {
    pub fn rest(&mut self, val: u16) {
        self.condition += val as i16;
        if self.condition > CONDITION_MAX_VALUE {
            self.condition = CONDITION_MAX_VALUE;
        }
    }

    pub fn condition_percentage(&self) -> u32 {
        (self.condition as f32 * 100.0 / CONDITION_MAX_VALUE as f32 ).floor() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rest_increases_condition() {
        let mut player_attributes = PlayerAttributes {
            is_banned: false,
            is_injured: false,
            condition: 5000,
            fitness: 8000,
            jadedness: 2000,
            weight: 75,
            height: 180,
            value: 1000000,
            current_reputation: 50,
            home_reputation: 60,
            world_reputation: 70,
            current_ability: 80,
            potential_ability: 90,
            international_apps: 10,
            international_goals: 5,
            under_21_international_apps: 15,
            under_21_international_goals: 7,
        };

        player_attributes.rest(1000);
        assert_eq!(player_attributes.condition, 6000);
    }

    #[test]
    fn test_rest_does_not_exceed_max_condition() {
        let mut player_attributes = PlayerAttributes {
            is_banned: false,
            is_injured: false,
            condition: 9500,
            fitness: 8000,
            jadedness: 2000,
            weight: 75,
            height: 180,
            value: 1000000,
            current_reputation: 50,
            home_reputation: 60,
            world_reputation: 70,
            current_ability: 80,
            potential_ability: 90,
            international_apps: 10,
            international_goals: 5,
            under_21_international_apps: 15,
            under_21_international_goals: 7,
        };

        player_attributes.rest(1000);
        assert_eq!(player_attributes.condition, CONDITION_MAX_VALUE);
    }

    #[test]
    fn test_condition_percentage() {
        let player_attributes = PlayerAttributes {
            is_banned: false,
            is_injured: false,
            condition: 7500,
            fitness: 8000,
            jadedness: 2000,
            weight: 75,
            height: 180,
            value: 1000000,
            current_reputation: 50,
            home_reputation: 60,
            world_reputation: 70,
            current_ability: 80,
            potential_ability: 90,
            international_apps: 10,
            international_goals: 5,
            under_21_international_apps: 15,
            under_21_international_goals: 7,
        };

        let condition_percentage = player_attributes.condition_percentage();
        assert_eq!(condition_percentage, 75);
    }

    #[test]
    fn test_condition_percentage_rounding() {
        let player_attributes = PlayerAttributes {
            is_banned: false,
            is_injured: false,
            condition: 7499,
            fitness: 8000,
            jadedness: 2000,
            weight: 75,
            height: 180,
            value: 1000000,
            current_reputation: 50,
            home_reputation: 60,
            world_reputation: 70,
            current_ability: 80,
            potential_ability: 90,
            international_apps: 10,
            international_goals: 5,
            under_21_international_apps: 15,
            under_21_international_goals: 7,
        };

        let condition_percentage = player_attributes.condition_percentage();
        assert_eq!(condition_percentage, 74);
    }
}