use crate::staff::contract::StaffCollection;
use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::core::SimulationContext;
use crate::player::contract::PlayerCollection;
use crate::player::player::PlayerPosition;
use crate::staff::contract::StaffClubContract;
use crate::utils::IntegerUtils;

#[derive(Debug, Clone)]
pub struct Club {
      pub id: u32,
      pub name: String,
      pub board: ClubBoard,
      pub players: PlayerCollection,
      pub staffs: StaffCollection,
      pub tactics: Option<Tactics>,
}

impl Club {
      pub fn new(
            name: String,
            players: PlayerCollection,
            staffs: StaffCollection,
      ) -> Self {
            Club {
                  id: IntegerUtils::random(0, 1_000_000) as u32,
                  board: ClubBoard::new(),
                  name,
                  players,
                  staffs,
                  tactics: None,
            }
      }

      pub fn items_count(&self) -> usize {
            self.players.len()
      }

      fn select_tactics(&mut self) {
            
      }

      fn get_match_squad(&self) -> Squad {
            self.players.get_match_squad()
      }
      
      pub fn simulate(&mut self, context: &mut SimulationContext) {
            self.players.simulate(context);
            self.staffs.simulate(context);
      }
}
