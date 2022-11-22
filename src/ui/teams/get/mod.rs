use crate::{GameAppData, PlayerStatusDto};
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use askama::Template;
use core::utils::FormattingUtils;
use core::{Player, PlayerPositionType, SimulatorData, Team};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TeamGetRequest {
    pub team_slug: String,
}

#[derive(Template)]
#[template(path = "teams/get/get.html")]
pub struct TeamGetViewModel<'c> {
    pub slug: &'c str,
    pub name: &'c str,
    pub league_id: u32,
    pub league_name: &'c str,
    pub balance: TeamBalance,
    pub players: Vec<TeamPlayer<'c>>,
    pub neighbor_teams: Vec<ClubTeam<'c>>,
}

pub struct ClubTeam<'c> {
    pub slug: &'c str,
    pub name: &'c str,
    pub reputation: u16,
}

pub struct TeamBalance {
    pub amount: i32,
    pub income: i32,
    pub outcome: i32,
}

pub struct TeamPlayer<'cp> {
    pub id: u32,
    pub last_name: &'cp str,
    pub first_name: &'cp str,

    pub behaviour: &'cp str,

    pub position: String,
    pub position_sort: PlayerPositionType,

    pub value: String,

    pub injured: bool,

    pub country_id: u32,
    pub country_code: &'cp str,
    pub country_name: &'cp str,

    pub conditions: u8,
    pub current_ability: u8,
    pub potential_ability: u8,

    pub status: PlayerStatusDto,
}

pub async fn team_get_action(
    state: Data<GameAppData>,
    route_params: web::Path<TeamGetRequest>,
) -> Result<HttpResponse> {
    let guard = state.data.lock().await;

    let simulator_data = guard.as_ref().unwrap();

    let team_id = simulator_data
        .team_id_by_slug(&route_params.team_slug)
        .unwrap();

    let team: &Team = simulator_data.team(team_id).unwrap();

    let league = simulator_data.league(team.league_id).unwrap();

    let now = simulator_data.date.date();

    let mut players: Vec<TeamPlayer> = team
        .players()
        .iter()
        .map(|p| {
            let country = simulator_data.country(p.country_id).unwrap();

            let position = p.positions.display_positions().join(", ");

            TeamPlayer {
                id: p.id,
                first_name: &p.full_name.first_name,
                position_sort: p.position(),
                position,
                behaviour: p.behaviour.as_str(),
                injured: p.player_attributes.is_injured,
                country_id: country.id,
                country_code: &country.code,
                country_name: &country.name,
                last_name: &p.full_name.last_name,
                conditions: get_conditions(p),
                value: FormattingUtils::short_money_str(p.value(now)),
                current_ability: get_current_ability_stars(p),
                potential_ability: get_potential_ability_stars(p),
                status: PlayerStatusDto::new(p.statuses.get()),
            }
        })
        .collect();

    players.sort_by(|a, b| a.position_sort.partial_cmp(&b.position_sort).unwrap());

    let model = TeamGetViewModel {
        slug: &team.slug,
        name: &team.name,
        league_id: league.id,
        league_name: &league.name,
        balance: TeamBalance {
            amount: 0,
            income: 0,
            outcome: 0,
        },
        players,
        neighbor_teams: get_neighbor_teams(team.club_id, simulator_data),
    };

    let html = TeamGetViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

fn get_neighbor_teams(club_id: u32, data: &SimulatorData) -> Vec<ClubTeam> {
    let club = data.club(club_id).unwrap();

    let mut teams: Vec<ClubTeam> = club
        .teams
        .teams
        .iter()
        .map(|team| ClubTeam {
            slug: &team.slug,
            name: &team.name,
            reputation: team.reputation.world,
        })
        .collect();

    teams.sort_by(|a, b| b.reputation.cmp(&a.reputation));

    teams
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
