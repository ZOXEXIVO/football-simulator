use crate::core::SimulationContext;
use crate::models::player::Player;

pub struct Club {
      pub name: String,
      pub players: Vec<Player>,
}

impl Club {
      pub fn new(name: String, players: Vec<Player>) -> Club {
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
