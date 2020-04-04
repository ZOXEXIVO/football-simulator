use crate::club::board::ClubBoard;
use crate::club::squad::Squad;
use crate::club::tactics::Tactics;
use crate::club::{BoardContext, ClubContext, ClubMood, TacticsSelector, TransferItem};
use crate::continent::ContinentContext;
use crate::core::context::GlobalContext;
use crate::core::SimulationContext;
use crate::country::CountryContext;
use crate::league::LeagueContext;
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

    pub fn simulate(&mut self, ctx: &mut GlobalContext) {
        let mut player_ctx = PlayerContext::new();
        
        self.players.simulate(&mut ctx.with_player(&mut player_ctx));

        //for player_id in player_ctx.player.unwrap().transfer_requests {}

        let mut staff_ctx = StaffContext::new();
        self.staffs.simulate(&mut ctx.with_staff(&mut staff_ctx));

        let mut board_ctx = BoardContext::new();

        self.board.simulate(&mut ctx.with_board(&mut board_ctx));
        //self.process_ctx(context);
    }

    fn process_ctx(&mut self, context: PlayerContext) {
        //or transfer_request in context.transfer_requests {
        //let player = self.players.get(transfer_request);

        // match TransferRequestNegotiation::negotiate(self, player) {
        //     TransferRequestNegotiationResult::Complete => {}
        // }
        //}

        //for improve_contract_request in context.contract_improve_requests {
        // match ContractImproveRequestNegotiation::negotiate(
        //     self,
        //     self.players.get(improve_contract_request),
        // ) {
        //     ContractImproveRequestNegotiationResult::Complete => {}
        // }
        //}
    }
}
