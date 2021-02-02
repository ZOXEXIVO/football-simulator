use crate::shared::FullName;
use crate::utils::IntegerUtils;
use crate::{Mental, PersonAttributes, PersonBehaviour, PersonBehaviourState, Physical, Player, PlayerAttributes, PlayerMailbox, PlayerPositions, PlayerPreferredFoot, PlayerSkills, PlayerStatusData, PlayerTraining, Relations, Technical, PlayerHappiness};
use chrono::{Datelike, NaiveDate};

pub struct PlayerGenerator;

impl PlayerGenerator {
    pub fn generate_young_player(country_id: u32, now: NaiveDate) -> Player {
        let year = IntegerUtils::random(now.year() - 14, now.year() - 16) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        Player {
            id: 0,
            full_name: FullName {
                first_name: "".to_string(),
                last_name: "".to_string(),
                middle_name: "".to_string(),
            },
            birth_date: NaiveDate::from_ymd(year as i32, month, day),
            country_id,
            behaviour: PersonBehaviour {
                state: PersonBehaviourState::Poor,
            },
            attributes: PersonAttributes {
                adaptability: 0,
                ambition: 0,
                controversy: 0,
                loyalty: 0,
                pressure: 0,
                professionalism: 0,
                sportsmanship: 0,
                temperament: 0,
            },
            happiness: PlayerHappiness::new(),
            statuses: PlayerStatusData { statuses: vec![] },
            skills: PlayerSkills {
                technical: Technical {
                    corners: 0,
                    crossing: 0,
                    dribbling: 0,
                    finishing: 0,
                    first_touch: 0,
                    free_kick_taking: 0,
                    heading: 0,
                    long_shots: 0,
                    long_throws: 0,
                    marking: 0,
                    passing: 0,
                    penalty_taking: 0,
                    tackling: 0,
                    technique: 0,
                },
                mental: Mental {
                    aggression: 0,
                    anticipation: 0,
                    bravery: 0,
                    composure: 0,
                    concentration: 0,
                    decisions: 0,
                    determination: 0,
                    flair: 0,
                    leadership: 0,
                    off_the_ball: 0,
                    positioning: 0,
                    teamwork: 0,
                    vision: 0,
                    work_rate: 0,
                },
                physical: Physical {
                    acceleration: 0,
                    agility: 0,
                    balance: 0,
                    jumping_reach: 0,
                    natural_fitness: 0,
                    pace: 0,
                    stamina: 0,
                    strength: 0,
                    match_readiness: 0,
                },
            },
            contract: Option::None,
            positions: PlayerPositions { positions: vec![] },
            preferred_foot: PlayerPreferredFoot::Left,
            player_attributes: PlayerAttributes {
                is_banned: false,
                is_injured: false,
                condition: 0,
                fitness: 0,
                jadedness: 0,
                weight: 0,
                height: 0,
                value: 0,
                current_reputation: 0,
                home_reputation: 0,
                world_reputation: 0,
                current_ability: 0,
                potential_ability: 0,
                international_apps: 0,
                international_goals: 0,
                under_21_international_apps: 0,
                under_21_international_goals: 0,
            },
            mailbox: PlayerMailbox::new(),
            training: PlayerTraining::new(),
            relations: Relations::new(),
        }
    }
}
