use crate::{ContractType, Player, PlayerClubContract, PlayerContractProposal, PlayerSquadStatus};
use chrono::NaiveDate;

pub struct AcceptContractHandler;

impl AcceptContractHandler {
    pub fn process(player: &mut Player, proposal: PlayerContractProposal, now: NaiveDate) {
        player.contract = Some(PlayerClubContract {
            salary: proposal.salary,
            contract_type: ContractType::FullTime,
            squad_status: PlayerSquadStatus::FirstTeamRegular,
            is_transfer_listed: false,
            transfer_status: Option::None,
            started: Option::None,
            expiration: now, //TODO ADD YEARS
            bonuses: vec![],
            clauses: vec![],
        });
    }
}
