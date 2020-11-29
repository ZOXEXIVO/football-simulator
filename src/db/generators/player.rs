use core::{Player, NaiveDate, PlayerClubContract, PlayerSkills, Technical, Mental, Physical, PlayerPosition, PlayerAttributes, PersonAttributes, PlayerPositions};
use core::utils::{IntegerUtils, StringUtils};
use core::shared::FullName;
use crate::db::PlayerPositionGenerator;

pub struct PlayerGenerator{
}

impl PlayerGenerator{
    pub fn generate() -> Player {
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
            Self::generate_skills(),
            Self::generate_person_attributes(),
            Self::generate_player_attributes(),
            Some(PlayerClubContract::new(
                IntegerUtils::random(1980, 2010) as f64, NaiveDate::from_ymd(2020, 3, 14))),
            Self::generate_positions(),
        );
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
    
    fn generate_positions() -> PlayerPositions {
        let positions_to_generate = IntegerUtils::random(1, 4) as u32;

        let mut positions = Vec::with_capacity(positions_to_generate as usize);

        for _ in 0..positions_to_generate {
            positions.push(PlayerPosition {
                position: PlayerPositionGenerator::generate(),
                level: IntegerUtils::random(0, 20) as u8,
            })
        }

        PlayerPositions {
            positions
        }
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
            temperament: IntegerUtils::random(0, 20) as u8
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
            current_ability: IntegerUtils::random(0, 20) as u8,
            potential_ability: IntegerUtils::random(0, 20) as i8,
            international_apps: IntegerUtils::random(0, 100) as u16,
            international_goals: IntegerUtils::random(0, 40) as u16,
            under_21_international_apps: IntegerUtils::random(0, 30) as u16,
            under_21_international_goals: IntegerUtils::random(0, 10) as u16
        }
    }
}