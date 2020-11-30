pub use chrono::prelude::{DateTime, Datelike, NaiveDate, Utc};
use crate::context::SimulationContext;

#[derive(Debug)]
pub enum ContractType{
    PartTime,
    FullTime,
    Amateur,
    Youth,
    NonContract
}

#[derive(Debug)]
pub enum PlayerSquadStatus {
    Invalid,
    NotYetSet,
    KeyPlayer,
    FirstTeamRegular,
    FirstTeamSquadRotation,
    MainBackupPlayer,
    HotProspectForTheFuture,
    DecentYoungster,
    NotNeeded,
    SquadStatusCount
}

#[derive(Debug)]
pub enum PlayerTransferStatus {
    TransferListed,
    LoadListed,
    TransferAndLoadListed
}

#[derive(Debug)]
pub struct PlayerClubContract {
    pub salary: f64,
    pub contract_type: ContractType,
    pub squad_status: PlayerSquadStatus,
    
    pub is_transfer_listed: bool,
    pub transfer_status: PlayerTransferStatus,
    
    pub started: Option<NaiveDate>,
    pub expired: NaiveDate,
    
    pub bonuses: Vec<ContractBonus>,
    pub clauses: Vec<ContractClause>,
}

impl PlayerClubContract {
    pub fn new(salary: f64, expired: NaiveDate) -> Self {
        PlayerClubContract {
            salary,
            contract_type: ContractType::FullTime,
            squad_status: PlayerSquadStatus::NotYetSet,            
            transfer_status: PlayerTransferStatus::TransferListed,
            is_transfer_listed: false,
            started: Option::None,
            expired,
            bonuses: vec![],
            clauses: vec![]
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        
        let naive_now = NaiveDate::from_ymd(now.year(), now.month(), now.day());

        self.expired >= naive_now
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        if context.check_contract_expiration() && self.is_expired() {}
    }
}

// Bonuses
#[derive(Debug)]
pub enum ContractBonusType {
    AppearanceFee,
    GoalFee,
    CleanSheetFee,
    TeamOfTheYear,
    TopGoalscorer,
    PromotionFee,
    AvoidRelegationFee,
    InternationalCapFee,
    UnusedSubstitutionFee
}

#[derive(Debug)]
pub struct ContractBonus {
    pub value: i32,
    pub bonus_type: ContractBonusType
}

impl ContractBonus {
    pub fn new(value: i32, bonus_type: ContractBonusType) -> Self {
        ContractBonus {
            value,
            bonus_type
        }
    }
}

// Clauses
#[derive(Debug)]
pub enum ContractClauseType {
    MinimumFeeRelease,
    RelegationFeeRelease,
    NonPromotionRelease,
    YearlyWageRise,
    PromotionWageIncrease,
    RelegationWageDecrease,
    StaffJobRelease,
    UnknownType7,
    SellOnFee,
    SellOnFeeProfit,
    SeasonalLandmarkGoalBonus,
    OneYearExtensionAfterLeagueGamesFinalSeason,
    MatchHighestEarner,
    WageAfterReachingClubCareerLeagueGames,
    TopDivisionPromotionWageRise,
    TopDivisionRelegationWageDrop,
    MinimumFeeReleaseToForeignClubs,
    MinimumFeeReleaseToHigherDivisionClubs,
    MinimumFeeReleaseToDomesticClubs,
    WageAfterReachingInternationalCaps,
    UnknownType20,
    UnknownType21,
    OptionalContractExtensionByClub
}

#[derive(Debug)]
pub struct ContractClause {
    pub value: i32,
    pub bonus_type: ContractClauseType
}

impl ContractClause {
    pub fn new(value: i32, bonus_type: ContractClauseType) -> Self {
        ContractClause {
            value,
            bonus_type
        }
    }
}