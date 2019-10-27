use crate::models::player::contract::PlayerClubContract;
use crate::core::SimulationContext;

pub struct Club {
      pub name: String,
      pub players: Vec<PlayerClubContract>,
}

impl Club {
      pub fn new(name: String, players: Vec<PlayerClubContract>) -> Club {
            Club {
                  name: name,
                  players: players,
            }
      }

      pub fn items_count(&self) -> usize {
            self.players.len()
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            for player in &mut self.players {
                  player.simulate(context);
            }
      }
}
