use crate::club::team::behaviour::{PlayerRelationshipChangeResult, TeamBehaviourResult};
use crate::{Player, PlayerCollection, StaffCollection};

pub struct TeamBehaviour;

impl TeamBehaviour {
    pub fn simulate(
        players: &mut PlayerCollection,
        _staffs: &mut StaffCollection,
    ) -> TeamBehaviourResult {
        let mut result = TeamBehaviourResult::new();

        for i in 0..players.players.len() {
            for j in i + 1..players.players.len() {
                let player_i = &players.players[i];
                let player_j = &players.players[j];

                let temperament_i = player_i.attributes.temperament;
                let temperament_j = player_j.attributes.temperament;

                if temperament_i > 0.5 && temperament_j > 0.5 {
                    result
                        .players
                        .relationship_result
                        .push(PlayerRelationshipChangeResult {
                            from_player_id: player_i.id,
                            to_player_id: player_j.id,
                            relationship_change: calculate_player_happiness(player_i),
                        });

                    result
                        .players
                        .relationship_result
                        .push(PlayerRelationshipChangeResult {
                            from_player_id: player_j.id,
                            to_player_id: player_i.id,
                            relationship_change: calculate_player_happiness(player_j),
                        });
                }
            }
        }

        result
    }
}

fn calculate_player_happiness(player: &Player) -> f32 {
    let mut happiness = 0.0;

    happiness += player
        .contract
        .as_ref()
        .map(|c| c.salary as f32 / 100.0)
        .unwrap_or(0.0);

    happiness += player.statistics.played as f32 / 20.0;

    happiness += player.statistics.goals as f32 / 10.0;

    happiness += player.attributes.temperament / 100.0;

    happiness = happiness.min(1.0).max(-1.0);

    happiness
}

pub fn calculate_relationship(player_i: &Player, player_j: &Player) -> f32 {
    let mut relationship = 0.0;

    let temperament_i = player_i.attributes.temperament;
    let temperament_j = player_j.attributes.temperament;
    let professionalism_i = player_i.attributes.professionalism;
    let professionalism_j = player_j.attributes.professionalism;
    let adaptability_i = player_i.attributes.adaptability;
    let adaptability_j = player_j.attributes.adaptability;
    let sportsmanship_i = player_i.attributes.sportsmanship;
    let sportsmanship_j = player_j.attributes.sportsmanship;
    let loyalty_i = player_i.attributes.loyalty;
    let loyalty_j = player_j.attributes.loyalty;
    let controversy_i = player_i.attributes.controversy;
    let controversy_j = player_j.attributes.controversy;

    let players_relationship_value = player_i.relations.get_player(player_j.id).unwrap_or(0.0);

    // let staff_relationship = player_i.relations.staff_relations.get(&player_j.id);
    // let staff_relationship_value = staff_relationship.unwrap_or(&0.0);

    const TEMPERAMENT_WEIGHT: f32 = -0.15;
    if temperament_i > 0.5 && temperament_j > 0.5 {
        relationship += -TEMPERAMENT_WEIGHT;
    } else if temperament_i < 0.5 && temperament_j < 0.5 {
        relationship += TEMPERAMENT_WEIGHT;
    }

    const PROFESSIONALISM_WEIGHT: f32 = 0.2;
    if professionalism_i > 0.5 && professionalism_j > 0.5 {
        relationship += PROFESSIONALISM_WEIGHT;
    } else if professionalism_i < 0.5 && professionalism_j < 0.5 {
        relationship += -PROFESSIONALISM_WEIGHT;
    }

    const ADAPTABILITY_WEIGHT: f32 = 0.15;
    if adaptability_i > 0.5 && adaptability_j > 0.5 {
        relationship += ADAPTABILITY_WEIGHT;
    } else if adaptability_i < 0.5 && adaptability_j < 0.5 {
        relationship += -ADAPTABILITY_WEIGHT;
    }

    const SPORTSMANSHIP_WEIGHT: f32 = 0.15;
    if sportsmanship_i > 0.5 && sportsmanship_j > 0.5 {
        relationship += SPORTSMANSHIP_WEIGHT;
    } else if sportsmanship_i < 0.5 && sportsmanship_j < 0.5 {
        relationship += -SPORTSMANSHIP_WEIGHT;
    }

    const LOYALTY_WEIGHT: f32 = 0.1;
    if loyalty_i > 0.5 && loyalty_j > 0.5 {
        relationship += LOYALTY_WEIGHT;
    } else if loyalty_i < 0.5 && loyalty_j < 0.5 {
        relationship += -LOYALTY_WEIGHT;
    }

    const CONTROVERSY_WEIGHT: f32 = -0.1;
    if controversy_i > 0.5 && controversy_j > 0.5 {
        relationship += -CONTROVERSY_WEIGHT;
    } else if controversy_i < 0.5 && controversy_j < 0.5 {
        relationship += CONTROVERSY_WEIGHT;
    }

    relationship += players_relationship_value;
    //relationship += *staff_relationship_value;

    relationship
}

pub struct PlayerBehaviour {
    pub players: PlayerBehaviourResult,
}

impl PlayerBehaviour {
    pub fn new() -> Self {
        PlayerBehaviour {
            players: PlayerBehaviourResult::new(),
        }
    }
}

pub struct PlayerBehaviourResult {
    pub players: Vec<Player>,
}

impl PlayerBehaviourResult {
    pub fn new() -> Self {
        PlayerBehaviourResult {
            players: Vec::new(),
        }
    }
}
