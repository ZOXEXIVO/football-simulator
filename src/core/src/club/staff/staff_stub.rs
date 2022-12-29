use crate::club::PersonBehaviour;
use crate::context::GlobalContext;
use crate::shared::fullname::FullName;
use crate::utils::{DateUtils, Logging};
use crate::{
    Person, PersonAttributes, Relations, Staff, StaffAttributes, StaffCoaching,
    StaffCollectionResult, StaffDataAnalysis, StaffFocus, StaffGoalkeeperCoaching, StaffKnowledge,
    StaffLicenseType, StaffMedical, StaffMental, StaffSkillFocusType, TeamType,
};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
pub struct StaffStub;

impl StaffStub {
    pub fn default() -> Staff {
        Staff {
            id: 0,
            full_name: FullName::with_full(
                "stub".to_string(),
                "stub".to_string(),
                "stub".to_string(),
            ),
            contract: None,
            country_id: 0,
            behaviour: PersonBehaviour::default(),
            birth_date: NaiveDate::from_ymd_opt(2019, 1, 1).unwrap(),
            relations: Relations::new(),
            license: StaffLicenseType::NationalC,
            attributes: PersonAttributes {
                adaptability: 1,
                ambition: 1,
                controversy: 1,
                loyalty: 1,
                pressure: 1,
                professionalism: 1,
                sportsmanship: 1,
                temperament: 1,
            },
            staff_attributes: StaffAttributes {
                coaching: StaffCoaching {
                    attacking: 1,
                    defending: 1,
                    fitness: 1,
                    mental: 1,
                    tactical: 1,
                    technical: 1,
                    working_with_youngsters: 1,
                },
                goalkeeping: StaffGoalkeeperCoaching {
                    distribution: 1,
                    handling: 1,
                    shot_stopping: 1,
                },
                mental: StaffMental {
                    adaptability: 1,
                    determination: 1,
                    discipline: 1,
                    man_management: 1,
                    motivating: 1,
                },
                knowledge: StaffKnowledge {
                    judging_player_ability: 1,
                    judging_player_potential: 1,
                    tactical_knowledge: 1,
                },
                data_analysis: StaffDataAnalysis {
                    judging_player_data: 1,
                    judging_team_data: 1,
                    presenting_data: 1,
                },
                medical: StaffMedical {
                    physiotherapy: 1,
                    sports_science: 1,
                    non_player_tendencies: 1,
                },
            },
            focus: Some(StaffFocus {
                technical_focus: vec![StaffSkillFocusType::Dribbling],
                mental_focus: vec![],
                physical_focus: vec![StaffSkillFocusType::Crossing, StaffSkillFocusType::WorkRate],
            }),
        }
    }
}
