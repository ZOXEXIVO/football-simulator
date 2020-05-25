use crate::club::{Club, ClubBoard, ClubMood, TrainingSchedule, ClubFinances, ClubSponsorshipContract};
use crate::country::Country;
use crate::league::{League, LeagueSettings, LeagueTable};
use crate::shared::fullname::FullName;
use crate::simulator::SimulatorData;
use crate::utils::{IntegerUtils, StringUtils};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use crate::continent::Continent;
use crate::people::{
    Mental, Physical, Player, PlayerAttributes, PlayerClubContract, PlayerCollection,
    PlayerPosition, PlayerPositionType, PlayerSkills, Staff, StaffClubContract, StaffCollection,
    StaffPosition, StaffStatus, Technical,
};
use crate::transfers::TransferPool;

impl SimulatorData {
    pub fn generate() -> SimulatorData {
        let date = NaiveDate::from_ymd(2020, 11, 15);
        let time = NaiveTime::from_hms(0, 0, 0);

        SimulatorData {
            id: SimulatorData::generate_id(),
            continents: vec![
                Continent {
                    id: 0,
                    name: "Africa".to_string(),
                    countries: vec![
                        Country {
                            id: 1,
                            name: String::from("Algeria"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 2,
                            name: String::from("Angola"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 3,
                            name: String::from("Benin"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 4,
                            name: String::from("Botswana"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 5,
                            name: String::from("Burkina Faso"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 6,
                            name: String::from("Burundi"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 7,
                            name: String::from("Cameroon"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 8,
                            name: String::from("Cape Verde"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 9,
                            name: String::from("Central African Republic"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 10,
                            name: String::from("Chad"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 11,
                            name: String::from("Camoros"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 12,
                            name: String::from("Democratic Republic of the Congo"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 13,
                            name: String::from("Djibouti"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 14,
                            name: String::from("Egypt"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 15,
                            name: String::from("Equatorial Guinea"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 16,
                            name: String::from("Eritrea"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 17,
                            name: String::from("Ethiopia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 18,
                            name: String::from("Gabon"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 19,
                            name: String::from("Gambia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 20,
                            name: String::from("Ghana"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 21,
                            name: String::from("Guinea"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 22,
                            name: String::from("Guinea-Bissau"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 23,
                            name: String::from("Ivory Coast"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 24,
                            name: String::from("Kenya"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 25,
                            name: String::from("Lesotho"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 26,
                            name: String::from("Liberia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 27,
                            name: String::from("Libya"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 28,
                            name: String::from("Madagascar"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 29,
                            name: String::from("Malawi"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 30,
                            name: String::from("Mali"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 31,
                            name: String::from("Mauritania"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 32,
                            name: String::from("Mauritius"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 33,
                            name: String::from("Morocco"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 34,
                            name: String::from("Mozambique"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 35,
                            name: String::from("Namibia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 36,
                            name: String::from("Niger"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 37,
                            name: String::from("Nigeria"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 38,
                            name: String::from("Rwanda"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 39,
                            name: String::from("Sao Tome and Principe"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 40,
                            name: String::from("Senegal"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 41,
                            name: String::from("Seychelles"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 42,
                            name: String::from("Sierra Leone"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 43,
                            name: String::from("Somalia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 44,
                            name: String::from("South Africa"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 45,
                            name: String::from("South Sudan"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 46,
                            name: String::from("Sudan"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 47,
                            name: String::from("Swaziland"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 48,
                            name: String::from("Tanzania"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 49,
                            name: String::from("Togo"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 50,
                            name: String::from("Tunisia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 51,
                            name: String::from("Uganda"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 52,
                            name: String::from("Zambia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 53,
                            name: String::from("Zimbabwe"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        }
                    ],
                },
                Continent {
                    id: 1,
                    name: "Europe".to_string(),
                    countries: vec![
                        Country {
                            id: 54,
                            name: String::from("Albania"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 55,
                            name: String::from("Andorra"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 56,
                            name: String::from("Austria"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 57,
                            name: String::from("Belarus"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 58,
                            name: String::from("Belgium"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 59,
                            name: String::from("Bosnia and Herzegovina"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 60,
                            name: String::from("Bulgaria"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 61,
                            name: String::from("Croatia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 62,
                            name: String::from("Czech Republic"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 63,
                            name: String::from("Denmark"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 64,
                            name: String::from("Estonia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 65,
                            name: String::from("Finland"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 66,
                            name: String::from("France"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 67,
                            name: String::from("Germany"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 68,
                            name: String::from("Greece"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 69,
                            name: String::from("Hungary"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 70,
                            name: String::from("Iceland"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 71,
                            name: String::from("Ireland"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 72,
                            name: String::from("Italy"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 73,
                            name: String::from("Latvia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 74,
                            name: String::from("Liechtenstein"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 75,
                            name: String::from("Lithuania"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 76,
                            name: String::from("Luxembourg"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 77,
                            name: String::from("Malta"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 78,
                            name: String::from("Moldova"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 79,
                            name: String::from("Monaco"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 80,
                            name: String::from("Montenegro"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 81,
                            name: String::from("Netherlands"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 82,
                            name: String::from("North Macedonia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 83,
                            name: String::from("Norway"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 84,
                            name: String::from("Poland"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 85,
                            name: String::from("Portugal"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 86,
                            name: String::from("Romania"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 87,
                            name: String::from("Russia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 88,
                            name: String::from("San Marino"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 89,
                            name: String::from("Serbia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 90,
                            name: String::from("Slovakia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 91,
                            name: String::from("Slovenia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 92,
                            name: String::from("Spain"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 93,
                            name: String::from("Sweden"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 94,
                            name: String::from("Switzerland"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 95,
                            name: String::from("Ukraine"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 96,
                            name: String::from("United Kingdom"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                       
                    ],
                },
                Continent {
                    id: 2,
                    name: "North America".to_string(),
                    countries: vec![
                        Country {
                            id: 97,
                            name: String::from("Antigua and Barbuda"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 98,
                            name: String::from("Bahamas"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 99,
                            name: String::from("Barbados"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 100,
                            name: String::from("Belize"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 101,
                            name: String::from("Canada"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 102,
                            name: String::from("Costa Rica"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 103,
                            name: String::from("Cuba"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 104,
                            name: String::from("Dominica"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 105,
                            name: String::from("Dominican Republic"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 106,
                            name: String::from("El Salvador"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 107,
                            name: String::from("Grenada"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 108,
                            name: String::from("Guatemala"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 109,
                            name: String::from("Haiti"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 110,
                            name: String::from("Honduras"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 111,
                            name: String::from("Jamaica"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 112,
                            name: String::from("Mexico"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 113,
                            name: String::from("Nicaragua"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 114,
                            name: String::from("Panama"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 115,
                            name: String::from("Saint Kitts and Nevis"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 116,
                            name: String::from("Saint Lucia"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 117,
                            name: String::from("Saint Vincent and the Grenadines"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 118,
                            name: String::from("Trinidad and Tobago"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        },
                        Country {
                            id: 119,
                            name: String::from("United States"),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 5000,
                        }
                    ]
                },
                Continent {
                    id: 3,
                    name: "Sourth America".to_string(),
                    countries: vec![
                        Country {
                            id: 120,
                            name: "Argentina".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 121,
                            name: "Bolivia".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 122,
                            name: "Brazil".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 123,
                            name: "Chile".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 124,
                            name: "Colombia".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 125,
                            name: "Ecuador".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 126,
                            name: "Guyana".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 127,
                            name: "Paraguay".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 128,
                            name: "Peru".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 129,
                            name: "Suriname".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 130,
                            name: "Uruguay".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        },
                        Country {
                            id: 131,
                            name: "Venezuela".to_string(),
                            leagues: (0..2).map(|_| League::generate()).collect(),
                            reputation: 4000,
                        }                       
                    ]
                },
                Continent {
                    id: 4,
                    name: "Australia".to_string(),
                    countries: vec![
                        Country {
                            id: 132,
                            name: "Australia".to_string(),
                            leagues: vec![],
                            reputation: 4000,
                        },
                        Country {
                            id: 133,
                            name: "Fiji".to_string(),
                            leagues: vec![],
                            reputation: 4000,
                        },
                        Country {
                            id: 134,
                            name: "Kiribati".to_string(),
                            leagues: vec![],
                            reputation: 4000,
                        },
                        Country {
                            id: 135,
                            name: "New Zealand".to_string(),
                            leagues: vec![],
                            reputation: 4000,
                        },
                        Country {
                            id: 136,
                            name: "Palau".to_string(),
                            leagues: vec![],
                            reputation: 4000,
                        },
                        Country {
                            id: 137,
                            name: "Samoa".to_string(),
                            leagues: vec![],
                            reputation: 4000,
                        },
                        Country {
                            id: 138,
                            name: "Tonga".to_string(),
                            leagues: vec![],
                            reputation: 4000,
                        },
                        Country {
                            id: 139,
                            name: "Tuvalu".to_string(),
                            leagues: vec![],
                            reputation: 4000,
                        },
                        Country {
                            id: 140,
                            name: "Vanuatu".to_string(),
                            leagues: vec![],
                            reputation: 4000,
                        }                     
                    ]
                },
            ],
            date: NaiveDateTime::new(date, time),
            transfer_pool: TransferPool::new(),
        }
    }
}

impl League {
    fn generate() -> League {
        let clubs_count = 10;
        
        let clubs: Vec<Club> = (0..clubs_count).map(|_| Club::generate()).collect();
        let club_headers: Vec<(u32, String)> = clubs.iter().map(|c| (c.id, c.name.clone())).collect();
        
        League {
            id: IntegerUtils::random(1, 10_000_000) as u32,
            name: StringUtils::random_string(30),
            clubs,
            schedule: None,
            settings: LeagueSettings {
                season_starting: (1, 1),
                season_ending: (1, 12),
            },
            table: LeagueTable::new(club_headers),
            reputation: 5000,
        }
    }
}

impl Club {
    fn generate() -> Club {
        let training_schedule = TrainingSchedule::new(
            NaiveTime::from_hms(10, 0, 0),
            NaiveTime::from_hms(17, 0, 0),
        );

        let sponsortship_contracts = vec![
            ClubSponsorshipContract::new(String::from("Sponsor 1"),
                                         IntegerUtils::random(1, 10_000_000) as u32,
                                         NaiveDate::from_ymd(2023, 1, 1)),
            ClubSponsorshipContract::new(String::from("Sponsor 2"),
                                         IntegerUtils::random(1, 10_000_000) as u32,
                                         NaiveDate::from_ymd(2025, 1, 1)),
            ClubSponsorshipContract::new(String::from("Sponsor 3"),
                                         IntegerUtils::random(1, 10_000_000) as u32,
                                         NaiveDate::from_ymd(2020, 1, 1))
        ];

        Club {
            id: IntegerUtils::random(1, 10_000_000) as u32,
            name: StringUtils::random_string(15),
            finance: ClubFinances::new(IntegerUtils::random(-10000, 10000000) as i32, sponsortship_contracts),
            mood: ClubMood::default(),
            board: ClubBoard::new(),
            players: PlayerCollection::new((0..30).map(|_| Player::generate()).collect()),
            staffs: StaffCollection::new((0..10).map(|_| Staff::generate()).collect()),
            tactics: None,
            training_schedule,
            transfer_list: Vec::new(),
            match_history: Vec::new(),
        }
    }
}

impl Player {
    fn generate() -> Player {
        let year = IntegerUtils::random(1980, 2010) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        return Player::new(
            IntegerUtils::random(1, 1_000_000) as u32,
            FullName {
                first_name: StringUtils::random_string(5),
                last_name: StringUtils::random_string(10),
                middle_name: StringUtils::random_string(15),
            },
            NaiveDate::from_ymd(year as i32, month, day),
            generate_skills(),
            generate_attributes(),
            Some(PlayerClubContract::new(
                IntegerUtils::random(1980, 2010) as f64, NaiveDate::from_ymd(2020, 3, 14))),
            generate_positions(),
        );

        fn generate_skills() -> PlayerSkills {
            PlayerSkills {
                technical: Technical {
                    corners: IntegerUtils::random(1, 20) as u8,
                    crossing: IntegerUtils::random(1, 20) as u8,
                    dribbling: IntegerUtils::random(1, 20) as u8,
                    finishing: IntegerUtils::random(1, 20) as u8,
                    first_touch: IntegerUtils::random(1, 20) as u8,
                    free_kick_taking: IntegerUtils::random(1, 20) as u8,
                    heading: IntegerUtils::random(1, 20) as u8,
                    long_shots: IntegerUtils::random(1, 20) as u8,
                    long_throws: IntegerUtils::random(1, 20) as u8,
                    marking: IntegerUtils::random(1, 20) as u8,
                    passing: IntegerUtils::random(1, 20) as u8,
                    penalty_taking: IntegerUtils::random(1, 20) as u8,
                    tackling: IntegerUtils::random(1, 20) as u8,
                    technique: IntegerUtils::random(1, 20) as u8,
                },
                mental: Mental {
                    aggression: IntegerUtils::random(1, 20) as u8,
                    anticipation: IntegerUtils::random(1, 20) as u8,
                    bravery: IntegerUtils::random(1, 20) as u8,
                    composure: IntegerUtils::random(1, 20) as u8,
                    concentration: IntegerUtils::random(1, 20) as u8,
                    decisions: IntegerUtils::random(1, 20) as u8,
                    determination: IntegerUtils::random(1, 20) as u8,
                    flair: IntegerUtils::random(1, 20) as u8,
                    leadership: IntegerUtils::random(1, 20) as u8,
                    off_the_ball: IntegerUtils::random(1, 20) as u8,
                    positioning: IntegerUtils::random(1, 20) as u8,
                    teamwork: IntegerUtils::random(1, 20) as u8,
                    vision: IntegerUtils::random(1, 20) as u8,
                    work_rate: IntegerUtils::random(1, 20) as u8,
                },
                physical: Physical {
                    acceleration: IntegerUtils::random(1, 20) as u8,
                    agility: IntegerUtils::random(1, 20) as u8,
                    balance: IntegerUtils::random(1, 20) as u8,
                    jumping_reach: IntegerUtils::random(1, 20) as u8,
                    natural_fitness: IntegerUtils::random(1, 20) as u8,
                    pace: IntegerUtils::random(1, 20) as u8,
                    stamina: IntegerUtils::random(1, 20) as u8,
                    strength: IntegerUtils::random(1, 20) as u8,
                    match_readiness: IntegerUtils::random(1, 20) as u8,
                },
            }
        }

        fn generate_positions() -> Vec<PlayerPosition> {
            let positions_to_generate = IntegerUtils::random(1, 4) as u32;

            let mut positions = Vec::with_capacity(positions_to_generate as usize);

            for pos in 0..positions_to_generate {
                positions.push(PlayerPosition {
                    position: PlayerPositionGenerator::generate(),
                    level: IntegerUtils::random(0, 20) as u8,
                })
            }

            positions
        }

        fn generate_attributes() -> PlayerAttributes {
            PlayerAttributes::new(
                IntegerUtils::random(0, 20) as u8,
                IntegerUtils::random(-20, 20) as i8,
            )
        }
    }
}

impl Staff {
    fn generate() -> Staff {
        let year = IntegerUtils::random(1980, 2010) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        Staff::new(
            IntegerUtils::random(1, 10_000_000) as u32,
            FullName {
                first_name: StringUtils::random_string(5),
                last_name: StringUtils::random_string(10),
                middle_name: StringUtils::random_string(15),
            },
            NaiveDate::from_ymd(year as i32, month, day),
            Some(StaffClubContract::new(
                NaiveDate::from_ymd(2020, 3, 14),
                StaffPosition::MainCoach,
                StaffStatus::Active,
            )),
        )
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
            _ => PlayerPositionType::Goalkeeper,
        }
    }
}
