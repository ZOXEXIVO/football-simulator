use crate::club::board::ClubBoard;
use crate::club::tactics::Tactics;
use crate::core::SimulationContext;
use crate::player::contract::PlayerClubContract;
use crate::player::player::Player;
use crate::staff::contract::StaffClubContract;
use crate::utils::IntegerUtils;

#[derive(Clone)]
pub struct Club {
      pub id: u32,
      pub name: String,
      pub board: ClubBoard,
      pub players: Vec<PlayerClubContract>,
      pub staffs: Vec<StaffClubContract>,
      pub tactics: Option<Tactics>,
}

impl Club {
      pub fn new(
            name: String,
            players: Vec<PlayerClubContract>,
            staffs: Vec<StaffClubContract>,
      ) -> Self {
            Club {
                  id: IntegerUtils::random(0, 1000000) as u32,
                  board: ClubBoard::new(),
                  name: name,
                  players: players,
                  staffs: staffs,
                  tactics: None,
            }
      }

      pub fn items_count(&self) -> usize {
            self.players.len()
      }

      pub fn get_players_for_match(&self) -> Vec<&Player> {
            let actual_players = self
                  .players
                  .iter()
                  .filter(|player_contract| !player_contract.is_expired())
                  .map(|p_contract| &p_contract.player)
                  .collect();

            actual_players
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            for player in &mut self.players {
                  player.simulate(context);
            }
            for staff in &mut self.staffs {
                  staff.simulate(context);
            }
      }
}
