use crate::club::{PlayerPositionType, Staff};
use crate::r#match::player::MatchPlayer;
use crate::{Player, Team};
use std::collections::HashSet;

pub struct SquadSelector;

const DEFAULT_SQUAD_SIZE: usize = 11;
const DEFAULT_BENCH_SIZE: usize = 6;

pub struct PlayerSelectionResult {
    pub main_squad: Vec<MatchPlayer>,
    pub substitutes: Vec<MatchPlayer>,
}

impl SquadSelector {
    pub fn select(team: &Team, staff: &Staff) -> PlayerSelectionResult {
        let current_tactics = team.tactics();

        let mut main_squad: Vec<MatchPlayer> = Vec::with_capacity(DEFAULT_SQUAD_SIZE);

        let mut selected_players = HashSet::new();

        for player_position in current_tactics.positions() {
            for position_player in select_by_type(team, player_position) {
                if staff.relations.is_favorite_player(position_player.id) {
                    main_squad.push(MatchPlayer::from_player(&position_player, *player_position))
                } else {
                    // TODO
                    main_squad.push(MatchPlayer::from_player(&position_player, *player_position))
                }

                selected_players.insert(position_player.id);
            }
        }

        let substitutes: Vec<MatchPlayer> = Vec::with_capacity(DEFAULT_BENCH_SIZE);

        return PlayerSelectionResult {
            main_squad,
            substitutes,
        };

        // helpers
        fn select_by_type<'p>(team: &'p Team, position: &PlayerPositionType) -> Vec<&'p Player> {
            let mut result: Vec<&Player> = Vec::with_capacity(5);

            let mut players_on_position = team.players.by_position(position);

            players_on_position.sort_by(|a, b| {
                a.player_attributes
                    .condition
                    .cmp(&b.player_attributes.condition)
            });

            for &player in players_on_position.iter().take(1) {
                result.push(player);
            }

            result
        }
    }
}

// use crate::club::{PlayerPositionType, Staff};
// use crate::{Player, SquadPlayer, Tactics, Team};
// use std::collections::HashSet;
//
// pub struct SquadSelector;
//
// const DEFAULT_SQUAD_SIZE: usize = 11;
// const DEFAULT_BENCH_SIZE: usize = 6;
//
// const POSITIONS: &[PlayerPositionType; 8] = &[
//     PlayerPositionType::Goalkeeper,
//     PlayerPositionType::DefenderLeft,
//     PlayerPositionType::DefenderCenter,
//     PlayerPositionType::DefenderRight,
//     PlayerPositionType::MidfielderLeft,
//     PlayerPositionType::MidfielderCenter,
//     PlayerPositionType::MidfielderRight,
//     PlayerPositionType::Striker,
// ];
//
// pub struct PlayerSelectionResult<'s> {
//     pub main_squad: Vec<SquadPlayer<'s>>,
//     pub substitutes: Vec<SquadPlayer<'s>>,
// }
//
// impl SquadSelector {
//     pub fn select<'c>(team: &'c Team, staff: &Staff) -> PlayerSelectionResult<'c> {
//         let current_tactics = team.tactics.as_ref().unwrap();
//
//         let mut main_squad: Vec<SquadPlayer<'c>> = Vec::with_capacity(DEFAULT_SQUAD_SIZE);
//         let mut substitutes: Vec<SquadPlayer<'c>> = Vec::with_capacity(DEFAULT_BENCH_SIZE);
//
//         return PlayerSelectionResult {
//             main_squad,
//             substitutes,
//         };
//     }
//
//     fn calculate_player_rating(player: &Player, tactics: &Tactics) -> f32 {
//         let mut rating = 0.0;
//
//         // for (skill, weight) in &tactics.skills_weights {
//         //     rating += player.skills.get_skill_level(skill) * weight;
//         // }
//         //
//         // for (attribute, weight) in &tactics.attributes_weights {
//         //     rating += player.player_attributes.get_attribute_value(attribute) * weight;
//         // }
//         //
//         // // Учитываем предпочтения тренера (если они есть).
//         // if let Some(preferred_players) = staff.preferred_players.as_ref() {
//         //     if preferred_players.contains(&player.id) {
//         //         rating += PREFERRED_PLAYER_BONUS;
//         //     }
//         // }
//
//         rating
//     }
// }
