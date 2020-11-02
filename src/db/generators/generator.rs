use core::context::{NaiveDateTime, NaiveTime};
use core::{NaiveDate, SimulatorData, Country, PlayerPositionType, Club, ClubMood, ClubBoard, ClubFinances, ClubSponsorship, ClubFinancialBalance, ClubReputation, PlayerCollection, TrainingSchedule, StaffCollection};
use crate::db::{DatabaseEntity, PlayerGenerator};
use core::continent::Continent;
use core::transfers::TransferPool;
use core::utils::IntegerUtils;
use core::league::{League, ScheduleManager, LeagueSettings, DayMonthPeriod, LeagueTable};
use core::shared::Location;
use core::club::academy::ClubAcademy;

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
        return data.clubs.iter().filter(|c| c.league_id == league_id).map(|club| {
            let club = Club {
                id: club.id,
                name: club.name.clone(),
                location: Location {
                    city_id: club.location.city_id
                },
                mood: ClubMood::default(),
                board: ClubBoard::new(),
                finance: ClubFinances::new(club.finance.balance, Vec::new()),
                reputation: ClubReputation {
                    home: club.reputation.home,
                    national: club.reputation.national,
                    world: club.reputation.world,
                },
                academy: ClubAcademy::new(100),
                tactics: Option::None,
                players: PlayerCollection::new((0..50).map(|i|PlayerGenerator::generate()).collect()),
                staffs: StaffCollection::new(Vec::new()),
                training_schedule: TrainingSchedule::new(
                NaiveTime::from_hms(10, 0, 0),
                NaiveTime::from_hms(17, 0, 0),
                ),
                transfer_list: vec![],
                match_history: vec![]
            };

            club
        }).collect();
    }
}

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
