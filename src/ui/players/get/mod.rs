use crate::GameAppData;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use askama::Template;
use core::{Person, Player, SimulatorData, Team};
use serde::Deserialize;
use core::utils::FormattingUtils;

#[derive(Deserialize)]
pub struct PlayerGetRequest {
    team_id: u32,
    player_id: u32,
}

#[derive(Template)]
#[template(path = "players/get/get.html")]
pub struct PlayerGetViewModel<'p> {
    pub id: u32,
    pub first_name: &'p str,
    pub last_name: &'p str,
    pub middle_name: &'p str,
    pub position: &'p str,
    pub contract: Option<PlayerContractDto>,
    pub birth_date: String,
    pub age: u8,
    pub team_id: u32,
    pub team_name: &'p str,
    pub country_id: u32,
    pub country_code: &'p str,
    pub country_name: &'p str,
    pub skills: PlayerSkillsDto,
    pub conditions: u8,
    pub current_ability: u8,
    pub potential_ability: u8,

    pub value: &'p str,
    
    pub preferred_foot: &'p str,
    
    pub player_attributes: PlayerAttributesDto,
    
    pub neighbor_teams: Vec<ClubTeam<'p>>,
    
    pub statistics: PlayerStatistics,
}

pub struct PlayerStatistics {
    pub played: u16,
    pub played_subs: u16,

    pub goals: u16,
    pub assists: u16,
    pub penalties: u16,
    pub player_of_the_match: u8,
    pub yellow_cards: u8,
    pub red_cards: u8,

    pub shots_on_target: f32,
    pub tackling: f32,
    pub passes: u8,

    pub average_rating: f32,
}

pub struct ClubTeam<'c> {
    pub id: u32,
    pub name: &'c str,
    pub reputation: u16,
}

pub struct PlayerContractDto {
    pub salary: u32,
    pub expiration: String,
    pub squad_status: String,
}

pub struct PlayerSkillsDto {
    pub technical: TechnicalDto,
    pub mental: MentalDto,
    pub physical: PhysicalDto,
}

pub struct TechnicalDto {
    pub corners: u8,
    pub crossing: u8,
    pub dribbling: u8,
    pub finishing: u8,
    pub first_touch: u8,
    pub free_kick_taking: u8,
    pub heading: u8,
    pub long_shots: u8,
    pub long_throws: u8,
    pub marking: u8,
    pub passing: u8,
    pub penalty_taking: u8,
    pub tackling: u8,
    pub technique: u8,
}

pub struct MentalDto {
    pub aggression: u8,
    pub anticipation: u8,
    pub bravery: u8,
    pub composure: u8,
    pub concentration: u8,
    pub decisions: u8,
    pub determination: u8,
    pub flair: u8,
    pub leadership: u8,
    pub off_the_ball: u8,
    pub positioning: u8,
    pub teamwork: u8,
    pub vision: u8,
    pub work_rate: u8,
}

pub struct PhysicalDto {
    pub acceleration: u8,
    pub agility: u8,
    pub balance: u8,
    pub jumping_reach: u8,
    pub natural_fitness: u8,
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,

    pub match_readiness: u8,
}

pub struct PlayerAttributesDto {
    pub international_apps: u16,
    pub international_goals: u16,

    pub under_21_international_apps: u16,
    pub under_21_international_goals: u16,
}

pub async fn player_get_action(
    state: Data<GameAppData>,
    route_params: web::Path<PlayerGetRequest>,
) -> Result<HttpResponse> {
    let guard = state.data.lock();

    let simulator_data = guard.as_ref().unwrap();

    let team: &Team = simulator_data.team(route_params.team_id).unwrap();

    let player: &Player = team
        .players
        .players()
        .iter()
        .find(|p| p.id == route_params.player_id)
        .unwrap();

    let country = simulator_data.country(player.country_id).unwrap();

    let now = simulator_data.date.date();
    
    let mut model = PlayerGetViewModel {
        id: player.id,
        first_name: &player.full_name.first_name,
        last_name: &player.full_name.last_name,
        middle_name: &player.full_name.middle_name,
        position: player.position().get_short_name(),
        contract: Option::None,
        birth_date: player.birth_date.format("%d.%m.%Y").to_string(),
        age: player.age(simulator_data.date.date()) as u8,
        team_id: team.id,
        team_name: &team.name,
        country_id: country.id,
        country_code: &country.code,
        country_name: &country.name,
        skills: get_skills(player),
        conditions: get_conditions(player),
        current_ability: get_current_ability_stars(player),
        potential_ability: get_potential_ability_stars(player),
        value: &FormattingUtils::short_money_str(player.value(now)),
        preferred_foot: player.preferred_foot_str(),
        player_attributes: get_attributes(player),
        neighbor_teams: get_neighbor_teams(team.club_id, simulator_data),
        statistics: get_statistics(player)
    };

    if let Some(contract) = &player.contract {
        model.contract = Some(PlayerContractDto {
            salary: (contract.salary / 1000) as u32,
            expiration: contract.expiration.format("%d.%m.%Y").to_string(),
            squad_status: String::from("First team player"),
        });
    }

    let html = PlayerGetViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

fn get_attributes(player: &Player) -> PlayerAttributesDto {
    PlayerAttributesDto {
        international_apps: player.player_attributes.international_apps,
        international_goals: player.player_attributes.international_goals,
        under_21_international_apps: player.player_attributes.under_21_international_apps,
        under_21_international_goals: player.player_attributes.under_21_international_goals
    }
}

fn get_skills(player: &Player) -> PlayerSkillsDto {
    PlayerSkillsDto {
        technical: TechnicalDto {
            corners: player.skills.technical.corners,
            crossing: player.skills.technical.crossing,
            dribbling: player.skills.technical.dribbling,
            finishing: player.skills.technical.finishing,
            first_touch: player.skills.technical.first_touch,
            free_kick_taking: player.skills.technical.free_kick_taking,
            heading: player.skills.technical.heading,
            long_shots: player.skills.technical.long_shots,
            long_throws: player.skills.technical.long_throws,
            marking: player.skills.technical.marking,
            passing: player.skills.technical.passing,
            penalty_taking: player.skills.technical.penalty_taking,
            tackling: player.skills.technical.tackling,
            technique: player.skills.technical.technique,
        },
        mental: MentalDto {
            aggression: player.skills.mental.aggression,
            anticipation: player.skills.mental.anticipation,
            bravery: player.skills.mental.bravery,
            composure: player.skills.mental.composure,
            concentration: player.skills.mental.concentration,
            decisions: player.skills.mental.decisions,
            determination: player.skills.mental.determination,
            flair: player.skills.mental.flair,
            leadership: player.skills.mental.leadership,
            off_the_ball: player.skills.mental.off_the_ball,
            positioning: player.skills.mental.positioning,
            teamwork: player.skills.mental.teamwork,
            vision: player.skills.mental.vision,
            work_rate: player.skills.mental.work_rate,
        },
        physical: PhysicalDto {
            acceleration: player.skills.physical.acceleration,
            agility: player.skills.physical.agility,
            balance: player.skills.physical.balance,
            jumping_reach: player.skills.physical.jumping_reach,
            natural_fitness: player.skills.physical.natural_fitness,
            pace: player.skills.physical.pace,
            stamina: player.skills.physical.stamina,
            strength: player.skills.physical.strength,
            match_readiness: player.skills.physical.match_readiness,
        },
    }
}

fn get_neighbor_teams(club_id: u32, data: &SimulatorData) -> Vec<ClubTeam> {
    let club = data.club(club_id).unwrap();

    let mut teams: Vec<ClubTeam> = club
        .teams
        .iter()
        .map(|team| ClubTeam {
            id: team.id,
            name: &team.name,
            reputation: team.reputation.world,
        })
        .collect();

    teams.sort_by(|a, b| b.reputation.cmp(&a.reputation));

    teams
}

fn get_statistics(player: &Player) -> PlayerStatistics {
    PlayerStatistics {
        played: player.statistics.played,
        played_subs: player.statistics.played_subs,
        goals: player.statistics.goals,
        assists: player.statistics.assists,
        penalties: player.statistics.penalties,
        player_of_the_match: player.statistics.player_of_the_match,
        yellow_cards: player.statistics.yellow_cards,
        red_cards: player.statistics.red_cards,
        shots_on_target: player.statistics.shots_on_target,
        tackling: player.statistics.tackling,
        passes: player.statistics.passes,
        average_rating: player.statistics.average_rating
    }
}

pub fn get_conditions(player: &Player) -> u8 {
    (100f32 * ((player.player_attributes.condition as f32) / 10000.0)) as u8
}

pub fn get_current_ability_stars(player: &Player) -> u8 {
    (5.0f32 * ((player.player_attributes.current_ability as f32) / 200.0)) as u8
}

pub fn get_potential_ability_stars(player: &Player) -> u8 {
    (5.0f32 * ((player.player_attributes.potential_ability as f32) / 200.0)) as u8
}
