use crate::r#match::{MatchPlayer, PlayerState};

pub struct PlayerConditions;

impl PlayerConditions {
    pub fn process(player: &mut MatchPlayer){
        Self::update_conditions(player);
    }

    fn update_conditions(player: &mut MatchPlayer) {
        match player.state {
            PlayerState::Running => player.skills.physical.stamina -= 0.1,
            PlayerState::Walking => player.skills.physical.stamina -= 0.05,
            _ => player.skills.physical.stamina -= 0.01,
        }

        if player.skills.physical.stamina < 0.0 {
            player.skills.physical.stamina = 0.0;
        }
    }
}
