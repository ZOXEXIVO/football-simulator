use crate::shared::FullName;
use crate::utils::IntegerUtils;
use crate::{
    Mental, PersonAttributes, PersonBehaviour, PersonBehaviourState, Physical, Player,
    PlayerAttributes, PlayerHappiness, PlayerMailbox, PlayerPositions, PlayerPreferredFoot,
    PlayerSkills, PlayerStatistics, PlayerStatisticsHistory, PlayerStatus, PlayerTraining,
    PlayerTrainingHistory, Relations, Technical,
};
use chrono::{Datelike, NaiveDate};

pub struct PlayerGenerator;

impl PlayerGenerator {
    pub fn generate(country_id: u32, now: NaiveDate) -> Player {
        let year = IntegerUtils::random(now.year() - 14, now.year() - 16) as u32;
        let month = IntegerUtils::random(1, 12) as u32;
        let day = IntegerUtils::random(1, 29) as u32;

        Player {
            id: 0,
            full_name: FullName::with_full("".to_string(), "".to_string(), "".to_string()),
            birth_date: NaiveDate::from_ymd_opt(year as i32, month, day).unwrap(),
            country_id,
            behaviour: PersonBehaviour {
                state: PersonBehaviourState::Poor,
            },
            attributes: PersonAttributes {
                adaptability: 0.0f32,
                ambition: 0.0f32,
                controversy: 0.0f32,
                loyalty: 0.0f32,
                pressure: 0.0f32,
                professionalism: 0.0f32,
                sportsmanship: 0.0f32,
                temperament: 0.0f32,
            },
            happiness: PlayerHappiness::new(),
            statuses: PlayerStatus { statuses: vec![] },
            skills: PlayerSkills {
                technical: Technical {
                    corners: 0.0f32,
                    crossing: 0.0f32,
                    dribbling: 0.0f32,
                    finishing: 0.0f32,
                    first_touch: 0.0f32,
                    free_kicks: 0.0f32,
                    heading: 0.0f32,
                    long_shots: 0.0f32,
                    long_throws: 0.0f32,
                    marking: 0.0f32,
                    passing: 0.0f32,
                    penalty_taking: 0.0f32,
                    tackling: 0.0f32,
                    technique: 0.0f32,
                },
                mental: Mental {
                    aggression: 0.0f32,
                    anticipation: 0.0f32,
                    bravery: 0.0f32,
                    composure: 0.0f32,
                    concentration: 0.0f32,
                    decisions: 0.0f32,
                    determination: 0.0f32,
                    flair: 0.0f32,
                    leadership: 0.0f32,
                    off_the_ball: 0.0f32,
                    positioning: 0.0f32,
                    teamwork: 0.0f32,
                    vision: 0.0f32,
                    work_rate: 0.0f32,
                },
                physical: Physical {
                    acceleration: 0.0f32,
                    agility: 0.0f32,
                    balance: 0.0f32,
                    jumping: 0.0f32,
                    natural_fitness: 0.0f32,
                    pace: 0.0f32,
                    stamina: 0.0f32,
                    strength: 0.0f32,
                    match_readiness: 0.0f32,
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
            training_history: PlayerTrainingHistory::new(),
            relations: Relations::new(),
            statistics: PlayerStatistics::new(),
            statistics_history: PlayerStatisticsHistory::new(),
        }
    }
}
