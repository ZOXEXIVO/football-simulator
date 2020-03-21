use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::club::{ClubMood, ClubSimulationContext, TacticsSelector, TransferItem};
use crate::core::SimulationContext;
use crate::people::{
    ContractImproveRequestNegotiation, ContractImproveRequestNegotiationResult, Player,
    PlayerCollection, PlayerSelector, StaffCollection, TransferRequestNegotiation,
    TransferRequestNegotiationResult,
};

#[derive(Debug)]
pub struct Club {
    pub id: u32,
    pub name: String,
    pub mood: ClubMood,
    pub board: ClubBoard,
    pub players: PlayerCollection,
    pub staffs: StaffCollection,
    pub tactics: Option<Tactics>,

    pub transfer_list: Vec<TransferItem>,
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
        // for transfer_request in context.transfer_requests {
        //     let player = self.players.get(transfer_request);
        //
        //     match TransferRequestNegotiation::negotiate(self, player) {
        //         TransferRequestNegotiationResult::Complete => {}
        //     }
        // }
        //
        // for improve_contract_request in context.contract_improve_requests {
        //     match ContractImproveRequestNegotiation::negotiate(
        //         self,
        //         self.players.get(improve_contract_request),
        //     ) {
        //         ContractImproveRequestNegotiationResult::Complete => {}
        //     }
        // }
    }
}
