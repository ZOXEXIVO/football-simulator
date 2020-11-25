use core::{Player, NaiveDate, PlayerClubContract, PlayerSkills, Technical, Mental, Physical, PlayerPosition, PlayerAttributes};
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
            Self::generate_attributes(),
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
    
    fn generate_positions() -> Vec<PlayerPosition> {
        let positions_to_generate = IntegerUtils::random(1, 4) as u32;

        let mut positions = Vec::with_capacity(positions_to_generate as usize);

        for _ in 0..positions_to_generate {
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