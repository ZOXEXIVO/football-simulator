use crate::r#match::{MatchPlayer};
use crate::r#match::goalkeepers::states::state::GoalkeeperState;
use crate::r#match::player::events::PlayerUpdateEvent;
use crate::r#match::player::state::PlayerState;

pub struct PlayerConditions;

impl PlayerConditions {
    pub fn process(player: &mut MatchPlayer){
        Self::update_conditions(player);
    }

    fn update_conditions(player: &mut MatchPlayer) {
        match player.state {
            PlayerState::Running => player.skills.physical.stamina -= 0.1,
            PlayerState::Goalkeeper(GoalkeeperState::Resting) |
            PlayerState::Goalkeeper(GoalkeeperState::Walking) => {
                player.skills.physical.stamina -= 0.05;
                player.player_attributes.condition += 10;
            },
            _ => player.skills.physical.stamina -= 0.01,
        }

        if player.skills.physical.stamina < 0.0 {
            player.skills.physical.stamina = 0.0;
        }
    }
}
