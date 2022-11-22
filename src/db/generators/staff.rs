use core::shared::FullName;
use core::utils::{IntegerUtils, StringUtils};
use core::{
    Datelike, NaiveDate, PeopleNameGeneratorData, PersonAttributes, Staff, StaffAttributes,
    StaffClubContract, StaffCoaching, StaffDataAnalysis, StaffGoalkeeperCoaching, StaffKnowledge,
    StaffLicenseType, StaffMedical, StaffMental, StaffPosition, StaffStatus, Utc,
};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub struct StaffGenerator {
    sequence: Arc<AtomicU32>,
    people_names_data: PeopleNameGeneratorData,
}

impl StaffGenerator {
    pub fn with_people_names(people_names: &PeopleNameGeneratorData) -> Self {
        StaffGenerator {
            sequence: Arc::new(AtomicU32::new(0)),
            people_names_data: PeopleNameGeneratorData {
                first_names: people_names.first_names.clone(),
                last_names: people_names.last_names.clone(),
            },
        }
    }
}

impl StaffGenerator {
    pub fn generate(&mut self, country_id: u32, position: StaffPosition) -> Staff {
        let now = Utc::now();

        let year = IntegerUtils::random(now.year() - 35, now.year() - 15) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        Staff::new(
            self.sequence.fetch_add(1, Ordering::SeqCst),
            FullName {
                first_name: self.generate_first_name(),
                last_name: self.generate_last_name(),
                middle_name: StringUtils::random_string(17),
            },
            country_id,
            NaiveDate::from_ymd_opt(year as i32, month, day).unwrap(),
            Self::generate_staff_attributes(),
            Some(StaffClubContract::new(
                IntegerUtils::random(1000, 200000) as u32,
                NaiveDate::from_ymd_opt(now.year() + IntegerUtils::random(1, 5), 3, 14).unwrap(),
                position,
                StaffStatus::Active,
            )),
            Self::generate_person_attributes(),
            Self::generate_staff_license_type(),
        )
    }

    fn generate_person_attributes() -> PersonAttributes {
        PersonAttributes {
            adaptability: IntegerUtils::random(0, 20) as u8,
            ambition: IntegerUtils::random(0, 20) as u8,
            controversy: IntegerUtils::random(0, 20) as u8,
            loyalty: IntegerUtils::random(0, 20) as u8,
            pressure: IntegerUtils::random(0, 20) as u8,
            professionalism: IntegerUtils::random(0, 20) as u8,
            sportsmanship: IntegerUtils::random(0, 20) as u8,
            temperament: IntegerUtils::random(0, 20) as u8,
        }
    }

    fn generate_staff_license_type() -> StaffLicenseType {
        match IntegerUtils::random(0, 6) {
            0 => StaffLicenseType::ContinentalPro,
            1 => StaffLicenseType::ContinentalA,
            2 => StaffLicenseType::ContinentalB,
            3 => StaffLicenseType::ContinentalC,
            4 => StaffLicenseType::NationalA,
            5 => StaffLicenseType::NationalB,
            6 => StaffLicenseType::NationalC,
            _ => StaffLicenseType::NationalC,
        }
    }

    fn generate_staff_attributes() -> StaffAttributes {
        StaffAttributes {
            coaching: StaffCoaching {
                attacking: IntegerUtils::random(0, 20) as u8,
                defending: IntegerUtils::random(0, 20) as u8,
                fitness: IntegerUtils::random(0, 20) as u8,
                mental: IntegerUtils::random(0, 20) as u8,
                tactical: IntegerUtils::random(0, 20) as u8,
                technical: IntegerUtils::random(0, 20) as u8,
                working_with_youngsters: IntegerUtils::random(0, 20) as u8,
            },
            goalkeeping: StaffGoalkeeperCoaching {
                distribution: IntegerUtils::random(0, 20) as u8,
                handling: IntegerUtils::random(0, 20) as u8,
                shot_stopping: IntegerUtils::random(0, 20) as u8,
            },
            mental: StaffMental {
                adaptability: IntegerUtils::random(0, 20) as u8,
                determination: IntegerUtils::random(0, 20) as u8,
                discipline: IntegerUtils::random(0, 20) as u8,
                man_management: IntegerUtils::random(0, 20) as u8,
                motivating: IntegerUtils::random(0, 20) as u8,
            },
            knowledge: StaffKnowledge {
                judging_player_ability: IntegerUtils::random(0, 20) as u8,
                judging_player_potential: IntegerUtils::random(0, 20) as u8,
                tactical_knowledge: IntegerUtils::random(0, 20) as u8,
            },
            data_analysis: StaffDataAnalysis {
                judging_player_data: IntegerUtils::random(0, 20) as u8,
                judging_team_data: IntegerUtils::random(0, 20) as u8,
                presenting_data: IntegerUtils::random(0, 20) as u8,
            },
            medical: StaffMedical {
                physiotherapy: IntegerUtils::random(0, 20) as u8,
                sports_science: IntegerUtils::random(0, 20) as u8,
                non_player_tendencies: IntegerUtils::random(0, 20) as u8,
            },
        }
    }

    fn generate_first_name(&self) -> String {
        if self.people_names_data.first_names.len() > 0 {
            let idx =
                IntegerUtils::random(0, self.people_names_data.first_names.len() as i32) as usize;

            self.people_names_data.first_names[idx].to_owned()
        } else {
            StringUtils::random_string(5)
        }
    }

    fn generate_last_name(&self) -> String {
        if self.people_names_data.first_names.len() > 0 {
            let idx =
                IntegerUtils::random(0, self.people_names_data.last_names.len() as i32) as usize;
            self.people_names_data.last_names[idx].to_owned()
        } else {
            StringUtils::random_string(12)
        }
    }
}
