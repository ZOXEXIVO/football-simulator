use crate::club::{PlayerPositionType, Staff};
use crate::r#match::player::MatchPlayer;
use crate::{Player, Tactics, Team};
use std::borrow::Borrow;

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

        let mut players: Vec<&Player> = team
            .players
            .players()
            .iter()
            .filter(|&&p| !p.player_attributes.is_injured && !p.player_attributes.is_banned)
            .map(|p| *p)
            .collect();

        PlayerSelectionResult {
            main_squad: SquadSelector::select_main_squad(
                &mut players,
                staff,
                current_tactics.borrow(),
            ),
            substitutes: SquadSelector::select_substitutes(
                &mut players,
                staff,
                current_tactics.borrow(),
            ),
        }
    }

    pub fn select_main_squad(
        players: &mut Vec<&Player>,
        staff: &Staff,
        tactics: &Tactics,
    ) -> Vec<MatchPlayer> {
        let mut squad: Vec<MatchPlayer> = Vec::with_capacity(DEFAULT_SQUAD_SIZE);

        for player_position in tactics.positions() {
            let mut best_player = None;
            let mut best_player_rating = -1.0;

            for &player in players
                .iter()
                .filter(|p| p.positions.has_position(*player_position))
            {
                let player_rating =
                    SquadSelector::calculate_player_rating(player, staff, player_position);

                if player_rating > best_player_rating {
                    best_player = Some(player);
                    best_player_rating = player_rating;
                }
            }

            if let Some(player) = best_player {
                squad.push(MatchPlayer::from_player(player, *player_position));
                players.retain(|p| p.id != player.id);
            }
        }

        squad
    }

    pub fn select_substitutes(
        players: &mut Vec<&Player>,
        staff: &Staff,
        tactics: &Tactics,
    ) -> Vec<MatchPlayer> {
        let mut squad: Vec<MatchPlayer> = Vec::with_capacity(DEFAULT_BENCH_SIZE);

        let goalkeeper = players
            .iter()
            .find(|player| player.positions.is_goalkeeper());

        if let Some(goalkeeper) = goalkeeper {
            squad.push(MatchPlayer::from_player(
                goalkeeper,
                PlayerPositionType::Goalkeeper,
            ));
            players.retain(|p| p.id != goalkeeper.id);
        }

        for player_position in tactics.positions() {
            let mut best_player = None;
            let mut best_player_rating = 0.0;

            for player in players.iter() {
                let player_rating =
                    SquadSelector::calculate_player_rating(player, staff, player_position);

                if player_rating > best_player_rating {
                    best_player = Some(player);
                    best_player_rating = player_rating;
                }
            }

            if let Some(player) = best_player {
                squad.push(MatchPlayer::from_player(player, *player_position));
                players.retain(|p| p.id != player.id);
            }
        }

        return squad;
    }

    fn calculate_player_rating(
        player: &Player,
        staff: &Staff,
        position: &PlayerPositionType,
    ) -> f32 {
        let mut rating = 0.0;

        rating += player.positions.get_level(*position) as f32 / 20.0;

        rating += player.player_attributes.condition as f32 / 10000.0;

        rating += 0.3 * (player.player_attributes.world_reputation as f32 / 10000.0);
        rating += 0.2 * (player.player_attributes.home_reputation as f32 / 10000.0);

        if staff.relations.is_favorite_player(player.id) {
            rating += 0.5;
        }

        rating
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r#match::PositionType;
    use crate::shared::FullName;
    use crate::{
        IntegerUtils, PlayerClubContract, PlayerCollection, PlayerGenerator, StaffCollection,
        StaffStub, TacticsPositioning, TeamReputation, TeamType, TrainingSchedule,
        TACTICS_POSITIONS,
    };
    use chrono::{NaiveDate, NaiveTime, Utc};

    #[test]
    fn select_is_correct() {
        let team = generate_team();

        let staff = StaffStub::default();

        let squad = SquadSelector::select(&team, &staff);

        // TODO
        //assert_eq!(11, squad.main_squad.len())
    }

    // helpers

    fn generate_team() -> Team {
        let mut team = Team::new(
            0,
            0,
            0,
            "".to_string(),
            "".to_string(),
            TeamType::Main,
            TrainingSchedule::new(
                NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            ),
            TeamReputation::new(0, 0, 0),
            PlayerCollection::new(generate_players()),
            StaffCollection::new(Vec::new()),
        );

        team.tactics = Some(Tactics::new(TacticsPositioning::T442));

        team
    }

    fn generate_players() -> Vec<Player> {
        let tactics = TACTICS_POSITIONS
            .iter()
            .find(|(tp, pt)| *tp == TacticsPositioning::T442)
            .map(|(tp, pt)| pt)
            .unwrap();

        let mut players = Vec::with_capacity(50);

        for tactic_position in tactics {
            for i in 0..5 {
                let level = IntegerUtils::random(10, 20) as u8;
                let player =
                    PlayerGenerator::generate(0, Utc::now().date_naive(), *tactic_position, level);

                players.push(player);
            }
        }

        players
    }
}
