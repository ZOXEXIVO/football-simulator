use crate::Squad;
use std::mem;
use crate::club::PlayerPositionType;
use super::distributions::random_gamma;

pub struct FootballEngine<'s> {
    home_squad: Squad<'s>,
    away_squad: Squad<'s>,
}

const MATCH_ACTIONS: u16 = 100;

impl<'s> FootballEngine<'s> {
    pub fn new(home_squad: Squad<'s>, away_squad: Squad<'s>) -> Self {
        FootballEngine {
            home_squad,
            away_squad,
        }
    }

    pub fn play(&mut self) -> FootballMatchDetails {
        let mut field_zone = MatchFieldZone::Midfield;

        let mut result = FootballMatchDetails {
            score: Score { home: 0, away: 0 },
            player_changes: vec![],
        };

        let home_team = self.get_team_for_squad(&self.home_squad);
        let away_team = self.get_team_for_squad(&self.away_squad);

        let mut attacking_team = &home_team;
        let mut defending_team = &away_team;

        for i in 0..MATCH_ACTIONS {
            let winner_team = self.get_battle_winner(&attacking_team, &defending_team, &field_zone);

            if winner_team.id == attacking_team.id {
                if attacking_team.id == home_team.id {
                    if field_zone == MatchFieldZone::BGoal {
                        result.score.home += 1;
                        field_zone = MatchFieldZone::Midfield;

                        mem::swap(&mut attacking_team, &mut defending_team);
                    } else {
                        field_zone = Self::up_field(&field_zone);
                    }
                } else if field_zone == MatchFieldZone::AGoal {
                    result.score.away += 1;
                    field_zone = MatchFieldZone::Midfield;
                    mem::swap(&mut attacking_team, &mut defending_team);
                } else {
                    field_zone = Self::down_field(&field_zone);
                }
            } else {
                field_zone = MatchFieldZone::Midfield;
                mem::swap(&mut attacking_team, &mut defending_team);
            }
        }

        result
    }

    fn up_field(field: &MatchFieldZone) -> MatchFieldZone {
        match field {
            MatchFieldZone::AGoal => MatchFieldZone::AField,
            MatchFieldZone::AField => MatchFieldZone::Midfield,
            MatchFieldZone::Midfield => MatchFieldZone::BField,
            MatchFieldZone::BField => MatchFieldZone::BGoal,
            MatchFieldZone::BGoal => MatchFieldZone::BField,
            _ => MatchFieldZone::Midfield,
        }
    }

    fn down_field(field: &MatchFieldZone) -> MatchFieldZone {
        match field {
            MatchFieldZone::BGoal => MatchFieldZone::BField,
            MatchFieldZone::BField => MatchFieldZone::Midfield,
            MatchFieldZone::Midfield => MatchFieldZone::AField,
            MatchFieldZone::AField => MatchFieldZone::AGoal,
            MatchFieldZone::AGoal => MatchFieldZone::AField,
            _ => MatchFieldZone::Midfield,
        }
    }

    fn get_battle_winner<'a>(
        &self,
        attacking_team: &'a MatchTeam,
        defending_team: &'a MatchTeam,
        current_zone: &MatchFieldZone,
    ) -> &'a MatchTeam {
        let mut attacking_team_skill = 0.0;
        let mut defending_team_skill = 0.0;

        match current_zone {
            MatchFieldZone::AField | MatchFieldZone::BField => {
                attacking_team_skill = attacking_team.striker_skill;
                defending_team_skill = defending_team.defender_skill;
            }
            MatchFieldZone::AGoal | MatchFieldZone::BGoal => {
                attacking_team_skill = attacking_team.defender_skill;
                defending_team_skill = defending_team.striker_skill;
            }
            MatchFieldZone::Midfield => {
                attacking_team_skill = attacking_team.midfielder_skill;
                defending_team_skill = defending_team.midfielder_skill;
            }
            _ => {}
        }

        let random_a = random_gamma(attacking_team_skill as f64, 0.5);
        let random_d = random_gamma(defending_team_skill as f64, 0.5);

        if random_a > random_d {
            attacking_team
        } else {
            defending_team
        }
    }

    fn get_team_for_squad(&self, squad: &Squad) -> MatchTeam {
        let mut team = MatchTeam::new(squad.club_id);

        for player in squad.players.iter().map(|p| &p.player) {
            match &player.position() {
                PlayerPositionType::Goalkeeper => {
                    team.goalkeeping_skill += player.get_skill() as f32;
                }
                PlayerPositionType::Defender => {
                    team.defender_skill += player.get_skill() as f32;
                }
                PlayerPositionType::Midfielder => {
                    team.defender_skill += 0.5 * player.get_skill() as f32;
                    team.midfielder_skill += player.get_skill() as f32;
                    team.striker_skill += 0.5 * player.get_skill() as f32;
                }
                PlayerPositionType::Forward => {
                    team.striker_skill += player.get_skill() as f32;
                }
                _ => {}
            }
        }

        team
    }
}

struct MatchTeam {
    pub id: u32,

    pub goalkeeping_skill: f32,
    pub defender_skill: f32,
    pub midfielder_skill: f32,
    pub striker_skill: f32,
}

impl MatchTeam {
    pub fn new(id: u32) -> Self {
        MatchTeam {
            id,
            goalkeeping_skill: 0.0,
            defender_skill: 0.0,
            midfielder_skill: 0.0,
            striker_skill: 0.0,
        }
    }
}

pub struct FootballMatchDetails {
    pub score: Score,
    pub player_changes: Vec<PlayerChanges>,
}

pub struct Score {
    pub home: u8,
    pub away: u8,
}

#[derive(Debug, PartialEq)]
pub enum MatchFieldZone {
    AGoal,
    AField,
    Midfield,
    BField,
    BGoal,
}

pub struct PlayerChanges {}
