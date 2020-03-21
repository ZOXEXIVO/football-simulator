#[derive(Debug)]
pub struct PlayerAttributes {
    pub current_ability: u8,
    pub potential_ability: i8,
}

impl PlayerAttributes {
    pub fn new(current_ability: u8, potential_ability: i8) -> Self {
        PlayerAttributes {
            current_ability,
            potential_ability,
        }
    }
}
