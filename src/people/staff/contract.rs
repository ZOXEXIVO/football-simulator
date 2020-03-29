use crate::core::context::Context;
use crate::people::{Staff, StaffContext};
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
    staff: Staff,
    expired: NaiveDate,
    position: StaffPosition,
    pub status: StaffStatus,
}

impl StaffClubContract {
    pub fn new(
        staff: Staff,
        expired: NaiveDate,
        position: StaffPosition,
        status: StaffStatus,
    ) -> Self {
        StaffClubContract {
            staff,
            expired,
            position,
            status,
        }
    }

    pub fn is_expired(&self, context: &mut dyn Context) -> bool {
        self.expired >= context.date().date()
    }

    pub fn simulate(&mut self, context: &mut StaffContext) {
        if context.check_contract_expiration() && self.is_expired(context) {
            self.status = StaffStatus::ExpiredContract;
        }

        self.staff.simulate(context);
    }
}

#[derive(Debug)]
pub struct StaffCollection {
    pub staffs: Vec<StaffClubContract>,
    pub roles: StaffRoles,

    stub: Staff,
}

#[derive(Debug)]
pub struct StaffRoles {
    main_coach: Option<StaffClubContract>,
    contract_resolver: Option<StaffClubContract>,
}

impl StaffCollection {
    pub fn new(staffs: Vec<StaffClubContract>) -> Self {
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

    pub fn simulate(&mut self, context: &mut StaffContext) {
        for staff_contract in &mut self.staffs {
            staff_contract.simulate(context)
        }
    }

    pub fn get_main_coach(&self) -> &Staff {
        self.get_by_position(&StaffPosition::MainCoach)
    }

    pub fn get_contract_resolver(&self) -> &Staff {
        self.get_by_position(&StaffPosition::MainCoach)
    }

    fn get_by_position(&self, position: &StaffPosition) -> &Staff {
        let main_coach_contract = self.staffs.iter().find(|c| c.position == *position);

        if main_coach_contract.is_none() {
            return &self.stub;
        }

        &main_coach_contract.unwrap().staff
    }
}
