use crate::people::player::player::Player;
use crate::people::{Behaviour, StaffClubContract, StaffResult, StaffPosition};
use crate::shared::fullname::FullName;
use crate::simulator::context::GlobalContext;
use crate::utils::DateUtils;
use chrono::NaiveDate;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Staff {
    pub id: u32,
    pub full_name: FullName,
    pub birth_date: NaiveDate,
    pub behaviour: Behaviour,

    pub contract: Option<StaffClubContract>,

    favorite_players: HashSet<u32>,
}

impl Staff {
    pub fn new(
        id: u32,
        full_name: FullName,
        birth_date: NaiveDate,
        contract: Option<StaffClubContract>,
    ) -> Self {
        Staff {
            id,
            full_name,
            birth_date,
            contract,
            behaviour: Behaviour::default(),
            favorite_players: HashSet::new(),
        }
    }

    pub fn stub() -> Self {
        Staff {
            id: 0,
            full_name: FullName {
                first_name: "stub".to_string(),
                last_name: "stub".to_string(),
                middle_name: "stub".to_string(),
            },
            contract: None,
            behaviour: Behaviour::default(),
            birth_date: NaiveDate::from_ymd(2019, 1, 1),
            favorite_players: HashSet::new(),
        }
    }

    pub fn add_to_favorites(&mut self, player: &Player) {
        self.favorite_players.insert(player.id);
    }

    pub fn is_favorite(&self, player: &Player) -> bool {
        self.favorite_players.contains(&player.id)
    }

    pub fn simulate(&mut self, ctx: GlobalContext) -> StaffResult {
        let result = StaffResult::new();
        
        if DateUtils::is_birthday(self.birth_date, ctx.simulation.date.date()) {}

        result  
    }
}

//DISPLAY
impl Display for Staff {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.full_name, self.birth_date)
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

    pub fn simulate(&mut self, ctx: GlobalContext) -> StaffResult {
        let result = StaffResult::new();

        for staff_contract in &mut self.staffs {
            staff_contract.simulate(ctx.with_staff());
        }

        result
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
