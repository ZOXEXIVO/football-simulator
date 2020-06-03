// use actix_web::{web, HttpResponse, Result};
// use crate::server::{GLOBAL_DATA};
// use serde::{Serialize, Deserialize};
//
// #[derive(Deserialize)]
// pub struct PlayerListRequest {
//     game_id: String
// }
//
// #[derive(Serialize)]
// pub struct PlayerListResponse<'p> {
//     pub players: Vec<PlayerDto<'p>>
// }
//
// pub async fn players_list_action(route_params: web::Path<PlayerListRequest>) -> Result<HttpResponse> {
//     if !GLOBAL_DATA.contains_key(&route_params.game_id){
//         return Ok(HttpResponse::NotFound().finish());
//     }
//
//     let simulator_data = GLOBAL_DATA.get(&route_params.game_id).unwrap();
//
//     let players = simulator_data.continents.iter().flat_map(|c| &c.countries)
//         .flat_map(|cn| &cn.leagues)
//         .flat_map(|l| &l.clubs)
//         .flat_map(|cl| &cl.players.players);
//    
//     let result = PlayerListResponse{
//         players: players.map(|p| PlayerDto {
//             first_name: &p.full_name.first_name,
//             last_name: &p.full_name.last_name,
//             middle_name: &p.full_name.middle_name
//         }).collect()
//     };
//    
//     Ok(HttpResponse::Ok().json(result))
// }
