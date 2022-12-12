use crate::club::{Player, PlayerPositionType, Staff};
use crate::{SquadPlayer, Team};
use std::collections::HashSet;

pub struct SquadSelector;

const DEFAULT_SQUAD_SIZE: usize = 11;
const DEFAULT_BENCH_SIZE: usize = 6;

const POSITIONS: &[PlayerPositionType; 8] = &[
    PlayerPositionType::Goalkeeper,
    PlayerPositionType::DefenderLeft,
    PlayerPositionType::DefenderCenter,
    PlayerPositionType::DefenderRight,
    PlayerPositionType::MidfielderLeft,
    PlayerPositionType::MidfielderCenter,
    PlayerPositionType::MidfielderRight,
    PlayerPositionType::Striker,
];

pub struct PlayerSelectionResult<'s> {
    pub main_squad: Vec<SquadPlayer<'s>>,
    pub substitutes: Vec<SquadPlayer<'s>>,
}

impl SquadSelector {
    pub fn select<'c>(team: &'c Team, staff: &Staff) -> PlayerSelectionResult<'c> {
        let current_tactics = team.tactics.as_ref().unwrap();

        let mut main_squad: Vec<SquadPlayer<'c>> = Vec::with_capacity(DEFAULT_SQUAD_SIZE);

        let mut selected_players = HashSet::new();

        for player_position in current_tactics.positions() {
            for position_player in select_by_type(team, player_position) {
                if staff
                    .relations
                    .is_favorite_player(position_player.player.id)
                {
                    main_squad.push(SquadPlayer::new(&position_player.player, *player_position))
                } else {
                    // TODO
                    main_squad.push(SquadPlayer::new(&position_player.player, *player_position))
                }

                selected_players.insert(position_player.player.id);
            }
        }

        let mut substitutes: Vec<SquadPlayer<'c>> = Vec::with_capacity(DEFAULT_BENCH_SIZE);

        return PlayerSelectionResult {
            main_squad,
            substitutes,
        };

        // helpers
        fn select_by_type<'c>(
            team: &'c Team,
            position: &PlayerPositionType,
        ) -> Vec<SquadPlayer<'c>> {
            let mut result: Vec<SquadPlayer<'c>> = Vec::with_capacity(5);

            let mut players_on_position = team.players.by_position(position);

            players_on_position.sort_by(|a, b| {
                a.player_attributes
                    .condition
                    .cmp(&b.player_attributes.condition)
            });

            for player in players_on_position.iter().take(5) {
                result.push(SquadPlayer::new(player, *position));
            }

            result
        }
    }
}
