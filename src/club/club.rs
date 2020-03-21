use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::club::{ClubMood, ClubSimulationContext, TacticsSelector};
use crate::core::SimulationContext;
use crate::people::{
    Player, PlayerCollection, PlayerSelector, StaffCollection, TransferRequestNegotiation,
    TransferRequestNegotiationResult,
};

#[derive(Debug, Clone)]
pub struct Club {
    pub id: u32,
    pub name: String,
    pub mood: ClubMood,
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
        let mut club_context = ClubSimulationContext::new(context);

        self.players.simulate(&mut club_context);
        self.staffs.simulate(&mut club_context);

        self.board.simulate(&mut club_context);

        self.process_context(club_context);
    }

    fn process_context(&mut self, context: ClubSimulationContext) {
        for transfer_request in context.transfer_requests {
            match self.players.get_player(transfer_request) {
                Some(player) => match TransferRequestNegotiation::negotiate(self, player) {
                    TransferRequestNegotiationResult::Complete => {}
                },
                None => {}
            }
        }
    }
}
