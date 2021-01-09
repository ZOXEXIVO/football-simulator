use core::shared::FullName;
use core::utils::{IntegerUtils, StringUtils};
use core::{
    Datelike, Mental, NaiveDate, PeopleNameGeneratorData, PersonAttributes, Physical, Player,
    PlayerAttributes, PlayerClubContract, PlayerPosition, PlayerPositionType, PlayerPositions,
    PlayerSkills, Technical, Utc,
};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub struct PlayerGenerator {
    sequence: Arc<AtomicU32>,
    people_names_data: PeopleNameGeneratorData,
}

impl PlayerGenerator {
    pub fn with_people_names(people_names: &PeopleNameGeneratorData) -> Self {
        PlayerGenerator {
            sequence: Arc::new(AtomicU32::new(0)),
            people_names_data: PeopleNameGeneratorData {
                first_names: people_names.first_names.clone(),
                last_names: people_names.last_names.clone(),
            },
        }
    }
}

pub enum PositionType {
    Goalkeeper,
    Defender,
    Midfielder,
    Striker,
}

impl PlayerGenerator {
    pub fn generate(&mut self, country_id: u32, position: PositionType) -> Player {
        let now = Utc::now();

        let year = IntegerUtils::random(now.year() - 35, now.year() - 15) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        Player::new(
            self.sequence.fetch_add(1, Ordering::SeqCst),
            FullName {
                first_name: self.generate_first_name(),
                last_name: self.generate_last_name(),
                middle_name: StringUtils::random_string(17),
            },
            NaiveDate::from_ymd(year as i32, month, day),
            country_id,
            Self::generate_skills(),
            Self::generate_person_attributes(),
            Self::generate_player_attributes(),
            Some(PlayerClubContract::new(
                IntegerUtils::random(1000, 200000) as u32,
                NaiveDate::from_ymd(now.year() + IntegerUtils::random(1, 5), 3, 14),
            )),
            Self::generate_positions(position),
        )
    }

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

    fn generate_positions(position: PositionType) -> PlayerPositions {
        let mut positions = Vec::with_capacity(5);

        match position {
            PositionType::Goalkeeper => positions.push(PlayerPosition {
                position: PlayerPositionType::Goalkeeper,
                level: 20 as u8,
            }),
            PositionType::Defender => match IntegerUtils::random(0, 3) % 3 {
                0 => {
                    positions.push(PlayerPosition {
                        position: PlayerPositionType::DefenderLeft,
                        level: 20 as u8,
                    });
                }
                1 => {
                    positions.push(PlayerPosition {
                        position: PlayerPositionType::DefenderCenter,
                        level: 20 as u8,
                    });
                }
                2 => {
                    positions.push(PlayerPosition {
                        position: PlayerPositionType::DefenderRight,
                        level: 20 as u8,
                    });
                }
                _ => {}
            },
            PositionType::Midfielder => match IntegerUtils::random(0, 3) % 3 {
                0 => {
                    positions.push(PlayerPosition {
                        position: PlayerPositionType::MidfielderLeft,
                        level: 20,
                    });
                }
                1 => {
                    positions.push(PlayerPosition {
                        position: PlayerPositionType::MidfielderCenter,
                        level: 20,
                    });
                }
                2 => {
                    positions.push(PlayerPosition {
                        position: PlayerPositionType::MidfielderRight,
                        level: 20 as u8,
                    });
                }
                _ => {}
            },
            PositionType::Striker => match IntegerUtils::random(0, 3) % 3 {
                0 => {
                    positions.push(PlayerPosition {
                        position: PlayerPositionType::WingbackLeft,
                        level: 20 as u8,
                    });
                }
                1 => {
                    positions.push(PlayerPosition {
                        position: PlayerPositionType::Striker,
                        level: 20 as u8,
                    });
                }
                2 => {
                    positions.push(PlayerPosition {
                        position: PlayerPositionType::WingbackRight,
                        level: 20 as u8,
                    });
                }
                _ => {}
            },
        }

        PlayerPositions { positions }
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

    fn generate_player_attributes() -> PlayerAttributes {
        PlayerAttributes {
            is_banned: false,
            is_injured: false,
            condition: IntegerUtils::random(0, 10000) as i16,
            fitness: IntegerUtils::random(0, 10000) as i16,
            jadedness: IntegerUtils::random(0, 10000) as i16,
            weight: IntegerUtils::random(60, 100) as u8,
            height: IntegerUtils::random(150, 220) as u8,
            value: 0,
            current_reputation: IntegerUtils::random(0, 3000) as i16,
            home_reputation: IntegerUtils::random(0, 3000) as i16,
            world_reputation: IntegerUtils::random(0, 1000) as i16,
            current_ability: IntegerUtils::random(0, 100) as u8,
            potential_ability: IntegerUtils::random(80, 200) as u8,
            international_apps: IntegerUtils::random(0, 100) as u16,
            international_goals: IntegerUtils::random(0, 40) as u16,
            under_21_international_apps: IntegerUtils::random(0, 30) as u16,
            under_21_international_goals: IntegerUtils::random(0, 10) as u16,
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
