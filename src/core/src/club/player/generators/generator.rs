use crate::shared::FullName;
use crate::utils::IntegerUtils;
use crate::{
    Mental, PersonAttributes, PersonBehaviour, PersonBehaviourState, Physical, Player,
    PlayerAttributes, PlayerHappiness, PlayerMailbox, PlayerPosition, PlayerPositionType,
    PlayerPositions, PlayerPreferredFoot, PlayerSkills, PlayerStatistics, PlayerStatisticsHistory,
    PlayerStatus, PlayerTraining, PlayerTrainingHistory, Relations, Technical,
};
use chrono::{Datelike, NaiveDate};

pub struct PlayerGenerator;

impl PlayerGenerator {
    pub fn generate(
        country_id: u32,
        now: NaiveDate,
        position: PlayerPositionType,
        level: u8,
    ) -> Player {
        let year = IntegerUtils::random(now.year() - 14, now.year() - 16) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        let positions = PlayerPositions {
            positions: vec![PlayerPosition { position, level }],
        };

        Player {
            id: 0,
            full_name: FullName::with_full("".to_string(), "".to_string(), "".to_string()),
            birth_date: NaiveDate::from_ymd_opt(year as i32, month, day).unwrap(),
            country_id,
            behaviour: PersonBehaviour {
                state: PersonBehaviourState::Poor,
            },
            attributes: PersonAttributes {
                adaptability: 0.0,
                ambition: 0.0,
                controversy: 0.0,
                loyalty: 0.0,
                pressure: 0.0,
                professionalism: 0.0,
                sportsmanship: 0.0,
                temperament: 0.0,
            },
            happiness: PlayerHappiness::new(),
            statuses: PlayerStatus { statuses: vec![] },
            skills: PlayerSkills {
                technical: Technical {
                    corners: 0.0,
                    crossing: 0.0,
                    dribbling: 0.0,
                    finishing: 0.0,
                    first_touch: 0.0,
                    free_kicks: 0.0,
                    heading: 0.0,
                    long_shots: 0.0,
                    long_throws: 0.0,
                    marking: 0.0,
                    passing: 0.0,
                    penalty_taking: 0.0,
                    tackling: 0.0,
                    technique: 0.0,
                },
                mental: Mental {
                    aggression: 0.0,
                    anticipation: 0.0,
                    bravery: 0.0,
                    composure: 0.0,
                    concentration: 0.0,
                    decisions: 0.0,
                    determination: 0.0,
                    flair: 0.0,
                    leadership: 0.0,
                    off_the_ball: 0.0,
                    positioning: 0.0,
                    teamwork: 0.0,
                    vision: 0.0,
                    work_rate: 0.0,
                },
                physical: Physical {
                    acceleration: 0.0,
                    agility: 0.0,
                    balance: 0.0,
                    jumping: 0.0,
                    natural_fitness: 0.0,
                    pace: 0.0,
                    stamina: 0.0,
                    strength: 0.0,
                    match_readiness: 0.0,
                },
            },
            contract: Option::None,
            positions,
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
            training_history: PlayerTrainingHistory::new(),
            relations: Relations::new(),
            statistics: PlayerStatistics::new(),
            statistics_history: PlayerStatisticsHistory::new(),
        }
    }
}
