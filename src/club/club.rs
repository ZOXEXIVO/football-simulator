use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::club::{BoardContext, ClubContext, ClubMood, TacticsSelector, TransferItem};
use crate::people::{
    Player, PlayerCollection, PlayerContext, PlayerSelector, Staff, StaffCollection, StaffContext,
    TransferRequestNegotiation, TransferRequestNegotiationResult,
};

#[derive(Debug)]
pub struct Club {
    pub id: u32,
    pub name: String,
    pub mood: ClubMood,
    pub board: ClubBoard,
    pub tactics: Option<Tactics>,

    pub players: PlayerCollection,
    pub staffs: StaffCollection,

    pub transfer_list: Vec<TransferItem>,
}

impl Club {
    pub fn items_count(&self) -> usize {
        self.players.len() + self.staffs.len()
    }

    pub fn players(&self) -> Vec<&Player> {
        self.players.players()
    }

    pub fn get_match_squad(&self) -> Squad {
        let main_coach = self.staffs.get_main_coach();

        Squad {
            club_id: self.id,
            tactics: TacticsSelector::select(self, main_coach.unwrap()),
            players: PlayerSelector::select(self, main_coach.unwrap()),
        }
    }

    pub fn simulate(&mut self, context: &mut ClubContext) {
        let mut player_context = PlayerContext::new(context);

        self.players.simulate(&mut player_context);

        for request in player_context.transfer_requests {}

        let mut staff_context = StaffContext::new(context);

        self.staffs.simulate(&mut staff_context);

        let mut board_context = BoardContext::new(context);
        self.board.simulate(&mut board_context);

        //self.process_context(context);
    }

    fn process_context(&mut self, context: PlayerContext) {
        for transfer_request in context.transfer_requests {
            //let player = self.players.get(transfer_request);

            // match TransferRequestNegotiation::negotiate(self, player) {
            //     TransferRequestNegotiationResult::Complete => {}
            // }
        }

        for improve_contract_request in context.contract_improve_requests {
            // match ContractImproveRequestNegotiation::negotiate(
            //     self,
            //     self.players.get(improve_contract_request),
            // ) {
            //     ContractImproveRequestNegotiationResult::Complete => {}
            // }
        }
    }
}
