use crate::people::Staff;
use crate::simulator::context::GlobalContext;
use crate::simulator::SimulationContext;
pub use chrono::prelude::{DateTime, Datelike, NaiveDate, Utc};

#[derive(Debug, PartialEq)]
pub enum StaffPosition {
    SportDirector,
    MainCoach,
    Coach,
    Physio,
}

#[derive(Debug, PartialEq)]
pub enum StaffStatus {
    Active,
    ExpiredContract,
}

#[derive(Debug)]
pub struct StaffClubContract {
    expired: NaiveDate,
    position: StaffPosition,
    pub status: StaffStatus,
}

impl StaffClubContract {
    pub fn new(expired: NaiveDate, position: StaffPosition, status: StaffStatus) -> Self {
        StaffClubContract {
            expired,
            position,
            status,
        }
    }

    pub fn is_expired(&self, context: &mut SimulationContext) -> bool {
        self.expired >= context.date.date()
    }

    pub fn simulate(&mut self, context: &mut SimulationContext) {
        if context.check_contract_expiration() && self.is_expired(context) {
            self.status = StaffStatus::ExpiredContract;
        }
    }
}

#[derive(Debug)]
pub struct StaffCollection {
    pub staffs: Vec<Staff>,
    pub roles: StaffRoles,

    stub: Staff,
}

#[derive(Debug)]
pub struct StaffRoles {
    main_coach: Option<StaffClubContract>,
    contract_resolver: Option<StaffClubContract>,
}

impl StaffCollection {
    pub fn new(staffs: Vec<Staff>) -> Self {
        StaffCollection {
            staffs,
            roles: StaffRoles {
                main_coach: None,
                contract_resolver: None,
            },
            stub: Staff::stub(),
        }
    }

    pub fn len(&self) -> usize {
        self.staffs.len()
    }

    pub fn simulate(&mut self, ctx: &mut GlobalContext) {
        for staff_contract in &mut self.staffs {
            staff_contract.simulate(ctx);
        }
    }

    pub fn get_main_coach(&self) -> Option<&Staff> {
        self.get_by_position(&StaffPosition::MainCoach)
    }

    pub fn get_contract_resolver(&self) -> Option<&Staff> {
        self.get_by_position(&StaffPosition::MainCoach)
    }

    fn get_by_position(&self, position: &StaffPosition) -> Option<&Staff> {
        let main_coach_contract = self.staffs.iter().find(|staff| {
            staff.contract.is_some() && staff.contract.as_ref().unwrap().position == *position
        });

        if main_coach_contract.is_none() {
            return Some(&self.stub);
        }

        Some(&main_coach_contract.unwrap())
    }
}
