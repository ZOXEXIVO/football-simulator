use actix_web::{web, HttpResponse, Result};
use crate::server::{GLOBAL_DATA};
use serde::{Serialize, Deserialize};
use crate::people::{PlayerPosition, PlayerPositionType};

#[derive(Deserialize)]
pub struct PlayerListRequest {
    game_id: String,
    club_id: u32
}

#[derive(Serialize)]
pub struct PlayerListResponse<'p> {
    pub players: Vec<PlayerListDto<'p>>
}

#[derive(Serialize)]
pub struct PlayerListDto<'p> {
    pub id: u32,
    // pub status: u32,
    // pub position: u8,
    pub first_name: &'p str,
    pub last_name: &'p str,
    pub middle_name: &'p str,
    // pub ability: u8,
    // pub potential: u8,
    // pub behaviour: u8,
    // pub condition: u8,
    // pub played_games: u8,
    // pub assists_goals: u8,
    // pub goals: u8,
    // pub value: u32    
}

pub async fn players_list_action(route_params: web::Path<PlayerListRequest>) -> Result<HttpResponse> {
    if !GLOBAL_DATA.contains_key(&route_params.game_id){
        return Ok(HttpResponse::NotFound().finish());
    }

    let simulator_data = GLOBAL_DATA.get(&route_params.game_id).unwrap();

    let players = simulator_data.continents.iter().flat_map(|c| &c.countries)
        .flat_map(|cn| &cn.leagues)
        .flat_map(|l| &l.clubs)
        .filter(|c| c.id == route_params.club_id)
        .flat_map(|cl| &cl.players.players);

    let result = PlayerListResponse{
        players: players.map(|p| PlayerListDto {
            id: p.id,
            first_name: &p.full_name.first_name,
            last_name: &p.full_name.last_name,
            middle_name: &p.full_name.middle_name
        }).collect()
    };

    Ok(HttpResponse::Ok().json(result))
}
