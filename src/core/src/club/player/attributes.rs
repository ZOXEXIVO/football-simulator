const CONDITION_MAX_VALUE: i16 = 10000;

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
        self.condition as f32 / CONDITION_MAX_VALUE as u32
    }
}
