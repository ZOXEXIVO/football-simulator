use crate::club::academy::result::ClubAcademyResult;
use crate::club::{BoardResult, ClubFinanceResult};
use crate::simulator::SimulatorData;
use crate::{
    Player, PlayerContractProposal, PlayerMessage, PlayerMessageType, PlayerResult, TeamResult,
};

pub struct ClubResult {
    pub finance: ClubFinanceResult,
    pub teams: Vec<TeamResult>,
    pub board: BoardResult,
    pub academy: ClubAcademyResult,
}

impl ClubResult {
    pub fn new(
        finance: ClubFinanceResult,
        teams: Vec<TeamResult>,
        board: BoardResult,
        academy: ClubAcademyResult,
    ) -> Self {
        ClubResult {
            finance,
            teams,
            board,
            academy,
        }
    }

    pub fn process(&self, data: &mut SimulatorData) {
        self.finance.process(data);

        for team_result in &self.teams {
            for player_result in &team_result.players.players {
                Self::process_player_interaction(player_result, data);
            }

            team_result.process(data);
        }

        self.board.process(data);
        self.academy.process(data);
    }

    fn process_player_interaction(result: &PlayerResult, data: &mut SimulatorData) {
        if result.contract.no_contract || result.contract.want_improve_contract {
            let player = data.player(result.player_id).unwrap();

            let player_growth_potential = player.growth_potential(data.date.date());

            player.mailbox.push(PlayerMessage {
                message_type: PlayerMessageType::ContractProposal(PlayerContractProposal {
                    salary: get_contract_wage(player_growth_potential),
                    years: 3,
                }),
            })
        }

        fn get_contract_wage(player_growth_potential: f32) -> u32 {
            match player_growth_potential {
                0.0..=3.0 => 1000u32,
                3.0..=4.0 => 2000u32,
                4.0..=5.0 => 3000u32,
                _ => 1000u32,
            }
        }
    }
}
