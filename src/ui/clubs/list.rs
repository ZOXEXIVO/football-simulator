// use actix_web::{web, HttpResponse, Result};
// use crate::server::{GLOBAL_DATA};
// use serde::{Serialize, Deserialize};
// 
// #[derive(Serialize)]
// pub struct ClubListDto<'c> {
//     pub id: u32,
//     pub name: &'c str
// }
// 
// #[derive(Deserialize)]
// pub struct ClubListRequest {
//     game_id: String
// }
// 
// #[derive(Serialize)]
// pub struct ClubListResponse<'c> {
//     pub clubs: Vec<ClubListDto<'c>>
// }
// 
// pub async fn club_list_action(route_params: web::Path<ClubListRequest>) -> Result<HttpResponse> {
//     if !GLOBAL_DATA.contains_key(&route_params.game_id){
//         return Ok(HttpResponse::NotFound().finish());
//     }
// 
//     let simulator_data = GLOBAL_DATA.get(&route_params.game_id).unwrap();
// 
//     let clubs = simulator_data.continents.iter().flat_map(|c| &c.countries)
//         .flat_map(|cn| &cn.leagues)
//         .flat_map(|l| &l.clubs);
//     
//     let result = ClubListResponse {
//         clubs: clubs.map(|c| ClubListDto {
//             id: c.id,
//             name: &c.name
//         }).collect()
//     };
//     
//     Ok(HttpResponse::Ok().json(result))
// }
