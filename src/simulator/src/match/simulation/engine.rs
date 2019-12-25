use crate::club::squad::Squad;
use crate::player::player::PlayerPositionType;
use crate::r#match::simulation::random_gamma;
use std::rc::Rc;

pub struct FootballEngine {
    home_squad: Squad,
    away_squad: Squad,
}

const MATCH_DURATION_SECS: u16 = 90 * 60;
const MATCH_ACTIONS: u16 = 1000;

impl FootballEngine {
    pub fn new(home_squad: Squad, away_squad: Squad) -> Self {
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

        let mut home_team = Rc::new(self.get_team_for_squad(&self.home_squad));
        let mut away_team = Rc::new(self.get_team_for_squad(&self.away_squad));

        let attacking_team = Rc::clone(&home_team);
        let defending_team = Rc::clone(&away_team);

        for i in 0..MATCH_ACTIONS {
            let winner_team = self.get_battle_winner(&attacking_team, &defending_team, &field_zone);

            if winner_team.id == attacking_team.id {
                if attacking_team.id == home_team.id {
                    if field_zone == MatchFieldZone::BGoal {
                        result.score.home += 1;
                        field_zone = MatchFieldZone::Midfield;
                        self.swap_ball();
                    } else {
                        field_zone = Self::up_field(&field_zone);
                    }
                } else {
                    if field_zone == MatchFieldZone::AGoal {
                        result.score.away += 1;
                        field_zone = MatchFieldZone::Midfield;
                        self.swap_ball();
                    } else {
                        field_zone = Self::down_field(&field_zone);
                    }
                }
            } else {
                field_zone = MatchFieldZone::Midfield;
                self.swap_ball();
            }
        }

        result
    }

    fn up_field(field: &MatchFieldZone) -> MatchFieldZone {
        return match field {
            MatchFieldZone::AGoal => MatchFieldZone::AField,
            MatchFieldZone::AField => MatchFieldZone::Midfield,
            MatchFieldZone::Midfield => MatchFieldZone::BField,
            MatchFieldZone::BGoal => MatchFieldZone::BField,
            _ => MatchFieldZone::Midfield,
        };
    }

    fn down_field(field: &MatchFieldZone) -> MatchFieldZone {
        return match field {
            MatchFieldZone::BGoal => MatchFieldZone::BField,
            MatchFieldZone::BField => MatchFieldZone::Midfield,
            MatchFieldZone::Midfield => MatchFieldZone::AField,
            MatchFieldZone::AField => MatchFieldZone::Midfield,
            MatchFieldZone::AGoal => MatchFieldZone::AField,
            _ => MatchFieldZone::Midfield,
        };
    }

    fn swap_ball(&self) {}

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
            return attacking_team;
        } else {
            return defending_team;
        }
    }

    fn get_team_for_squad(&self, squad: &Squad) -> MatchTeam {
        let mut team = MatchTeam::new(squad.club_id);

        for player in squad.players.iter().map(|p| &p.1) {
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
                PlayerPositionType::Striker => {
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

#[derive(PartialEq)]
pub enum MatchFieldZone {
    AGoal,
    AField,
    Midfield,
    BField,
    BGoal,
}

pub struct PlayerChanges {}
