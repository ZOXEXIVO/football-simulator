use core::context::{NaiveDateTime, NaiveTime};
use core::{NaiveDate, SimulatorData, Country, PlayerPositionType, Club};
use crate::db::DatabaseEntity;
use core::continent::Continent;
use core::transfers::TransferPool;
use core::utils::IntegerUtils;
use core::league::{League, ScheduleManager, LeagueSettings, DayMonthPeriod, LeagueTable};

const CONTINENTS: [(u32, &'static str); 5] = [
    (0, "Africa"), 
    (1, "Europe"), 
    (2, "North America"), 
    (3, "South America"), 
    (4, "Australia")
];

pub struct Generator;

impl Generator{
    pub fn generate(data: &DatabaseEntity) -> SimulatorData {
        let current_date = NaiveDateTime::new(
            NaiveDate::from_ymd(2020, 11, 15),
            NaiveTime::from_hms(0, 0, 0));
        
        let continents = CONTINENTS.iter().map(|(c_ic, c)| {
            let continent = Continent {
                id: *c_ic,
                name: String::from(c.to_owned()),
                countries: Generator::generate_countries(c, data),
            };
            
            continent
        }).collect();
        
        SimulatorData {
            id: SimulatorData::generate_id(),
            continents,
            date: current_date,
            transfer_pool: TransferPool::new(),
        }
    }
    
    fn generate_countries(continent: &str, data: &DatabaseEntity) -> Vec<Country> {
        return data.countries.iter().filter(|cn| cn.continent == continent).map(|c| {
            let country = Country{
                id: c.id,
                code: c.code.clone(),
                name: c.name.clone(),
                leagues: Generator::generate_leagues(c.id, data),
                reputation: c.reputation
            };
            
            country
        }).collect();
    }

    fn generate_leagues(country_id: u32, data: &DatabaseEntity) -> Vec<League> {
        return data.leagues.iter().filter(|l| l.country_id == country_id).map(|l| {
            let clubs = Generator::generate_clubs(l.id, data);
            
            let club_ids = clubs.iter().map(|c| c.id).collect();
            
            let league = League {
                id: l.id,
                name: l.name.clone(),
                clubs,
                schedule_manager: ScheduleManager::new(),
                settings: LeagueSettings { season_starting_half: DayMonthPeriod {
                        from_day: l.settings.season_starting_half.from_day,
                        from_month: l.settings.season_starting_half.from_month,
                        to_day: l.settings.season_starting_half.to_day,
                        to_month: l.settings.season_starting_half.to_month
                    }, season_ending_half: DayMonthPeriod {
                        from_day: l.settings.season_ending_half.from_day,
                        from_month: l.settings.season_ending_half.from_month,
                        to_day: l.settings.season_ending_half.to_day,
                        to_month: l.settings.season_ending_half.to_month
                    } },
                league_table: LeagueTable::new(club_ids),
                reputation: 0
            };

            league
        }).collect();
    }

    fn generate_clubs(league_id: u32, data: &DatabaseEntity) -> Vec<Club> {
        Vec::new()
        // return data..iter().filter(|l| l.country_id == country_id).map(|l| {
        //     let league = League {
        //         id: l.id,
        //         name: l.name.clone(),
        //         clubs: vec![],
        //         schedule_manager: ScheduleManager::new(),
        //         settings: LeagueSettings { season_starting_half: DayMonthPeriod {
        //             from_day: l.settings.season_starting_half.from_day,
        //             from_month: l.settings.season_starting_half.from_month,
        //             to_day: l.settings.season_starting_half.to_day,
        //             to_month: l.settings.season_starting_half.to_month
        //         }, season_ending_half: DayMonthPeriod {
        //             from_day: l.settings.season_ending_half.from_day,
        //             from_month: l.settings.season_ending_half.from_month,
        //             to_day: l.settings.season_ending_half.to_day,
        //             to_month: l.settings.season_ending_half.to_month
        //         } },
        //         league_table: LeagueTable::new,
        //         reputation: 0
        //     };
        // 
        //     league;
        // }).collect();
    }
}

// impl Club {
//     fn generate() -> Club {
//         let training_schedule = TrainingSchedule::new(
//             NaiveTime::from_hms(10, 0, 0),
//             NaiveTime::from_hms(17, 0, 0),
//         );
// 
//         let sponsorship_contracts = vec![
//             ClubSponsorshipContract::new(String::from("Sponsor 1"),
//                                          IntegerUtils::random(1, 10_000_000),
//                                          NaiveDate::from_ymd(2023, 1, 1)),
//             ClubSponsorshipContract::new(String::from("Sponsor 2"),
//                                          IntegerUtils::random(1, 10_000_000),
//                                          NaiveDate::from_ymd(2025, 1, 1)),
//             ClubSponsorshipContract::new(String::from("Sponsor 3"),
//                                          IntegerUtils::random(1, 10_000_000),
//                                          NaiveDate::from_ymd(2020, 1, 1))
//         ];
// 
//         Club::new(
//             IntegerUtils::random(1, 10_000_000) as u32,
//             StringUtils::random_string(15),
//             Location::new(2),
//             ClubFinances::new(IntegerUtils::random(-10000, 10000000) as i32, sponsorship_contracts),
//             ClubReputation::new(3000, 2000, 1000),
//             training_schedule,
//             PlayerCollection::new((0..30).map(|_| PlayerGenerator::generate()).collect()),
//             StaffCollection::new((0..10).map(|_| Staff::generate()).collect()),
//         )
//     }
// }
// 
// impl Staff {
//     fn generate() -> Staff {
//         let year = IntegerUtils::random(1980, 2010) as u32;
//         let month = IntegerUtils::random(1, 12) as u32;
//         let day = IntegerUtils::random(1, 29) as u32;
// 
//         Staff::new(
//             IntegerUtils::random(1, 10_000_000) as u32,
//             FullName {
//                 first_name: StringUtils::random_string(5),
//                 last_name: StringUtils::random_string(10),
//                 middle_name: StringUtils::random_string(15),
//             },
//             NaiveDate::from_ymd(year as i32, month, day),
//             Some(StaffClubContract::new(
//                 NaiveDate::from_ymd(2020, 3, 14),
//                 StaffPosition::MainCoach,
//                 StaffStatus::Active,
//             )),
//         )
//     }
// }
// 
pub struct PlayerPositionGenerator;

impl PlayerPositionGenerator {
    pub fn generate() -> PlayerPositionType {
        match IntegerUtils::random(0, 3) {
            0 => PlayerPositionType::Goalkeeper,
            1 => PlayerPositionType::Defender,
            2 => PlayerPositionType::Midfielder,
            3 => PlayerPositionType::Forward,
            _ => panic!("Unknown player position type"),
        }
    }
}
