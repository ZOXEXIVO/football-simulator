use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::club::TacticsSelector;
use crate::core::SimulationContext;
use crate::people::{Player, PlayerCollection, PlayerSelector, StaffCollection};

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
    pub fn items_count(&self) -> usize {
        self.players.len() + self.staffs.len()
    }

    pub fn players(&self) -> Vec<&Player> {
        self.players.contracts.iter().map(|p| &p.player).collect()
    }

    pub fn get_match_squad(&self) -> Squad {
        let main_coach = self.staffs.get_main_coach();

        Squad {
            club_id: self.id,
            tactics: TacticsSelector::select(self, main_coach),
            players: PlayerSelector::select(self, main_coach),
        }
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        self.players.simulate(context);
        self.staffs.simulate(context);
    }
}
