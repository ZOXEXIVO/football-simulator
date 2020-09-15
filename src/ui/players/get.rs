// use actix_web::{web, HttpResponse, Result};
// use crate::server::{GLOBAL_DATA};
// use serde::{Serialize, Deserialize};
// use crate::club::Player;
// 
// #[derive(Deserialize)]
// pub struct PlayerGetRequest {
//     game_id: String,
//     club_id: u32,
//     player_id: u32,
// }
// 
// #[derive(Serialize)]
// pub struct PlayerGetResponse<'p> {
//     pub player: PlayerDto<'p>
// }
// 
// #[derive(Serialize)]
// pub struct PlayerDto<'p> {
//     pub id: u32,
//     // pub status: u32,
// // pub position: u8,
//     pub first_name: &'p str,
//     pub last_name: &'p str,
//     pub middle_name: &'p str,
// 
//     pub club_name: &'p str,
// 
//     pub skills: PlayerSkillsDto,
// }
// 
// 
// #[derive(Serialize)]
// pub struct PlayerSkillsDto {
//     pub technical: TechnicalDto,
//     pub mental: MentalDto,
//     pub physical: PhysicalDto,
// }
// 
// #[derive(Serialize)]
// pub struct TechnicalDto {
//     pub corners: u8,
//     pub crossing: u8,
//     pub dribbling: u8,
//     pub finishing: u8,
//     pub first_touch: u8,
//     pub free_kick_taking: u8,
//     pub heading: u8,
//     pub long_shots: u8,
//     pub long_throws: u8,
//     pub marking: u8,
//     pub passing: u8,
//     pub penalty_taking: u8,
//     pub tackling: u8,
//     pub technique: u8,
// }
// 
// #[derive(Serialize)]
// pub struct MentalDto {
//     pub aggression: u8,
//     pub anticipation: u8,
//     pub bravery: u8,
//     pub composure: u8,
//     pub concentration: u8,
//     pub decisions: u8,
//     pub determination: u8,
//     pub flair: u8,
//     pub leadership: u8,
//     pub off_the_ball: u8,
//     pub positioning: u8,
//     pub teamwork: u8,
//     pub vision: u8,
//     pub work_rate: u8,
// }
// 
// #[derive(Serialize)]
// pub struct PhysicalDto {
//     pub acceleration: u8,
//     pub agility: u8,
//     pub balance: u8,
//     pub jumping_reach: u8,
//     pub natural_fitness: u8,
//     pub pace: u8,
//     pub stamina: u8,
//     pub strength: u8,
// 
//     pub match_readiness: u8,
// }
// 
// pub async fn player_get_action(route_params: web::Path<PlayerGetRequest>) -> Result<HttpResponse> {
//     if !GLOBAL_DATA.contains_key(&route_params.game_id) {
//         return Ok(HttpResponse::NotFound().finish());
//     }
// 
//     let simulator_data = GLOBAL_DATA.get(&route_params.game_id).unwrap();
// 
//     let player: &Player = simulator_data.continents.iter().flat_map(|c| &c.countries)
//         .flat_map(|cn| &cn.leagues)
//         .flat_map(|l| &l.clubs)
//         .filter(|club| club.id == route_params.club_id)
//         .flat_map(|c| c.players())
//         .find(|p| p.id == route_params.player_id)
//         .unwrap();
// 
//     let result = PlayerGetResponse {
//         player: PlayerDto {
//             id: player.id,
//             first_name: &player.full_name.first_name,
//             last_name: &player.full_name.last_name,
//             middle_name: &player.full_name.middle_name,
//             club_name: &"Juventus",
//             skills: PlayerSkillsDto {
//                 technical: TechnicalDto{
//                     corners: player.skills.technical.corners,
//                     crossing: player.skills.technical.crossing,
//                     dribbling: player.skills.technical.dribbling,
//                     finishing: player.skills.technical.finishing,
//                     first_touch: player.skills.technical.first_touch,
//                     free_kick_taking: player.skills.technical.free_kick_taking,
//                     heading: player.skills.technical.heading,
//                     long_shots: player.skills.technical.long_shots,
//                     long_throws: player.skills.technical.long_throws,
//                     marking: player.skills.technical.marking,
//                     passing: player.skills.technical.passing,
//                     penalty_taking: player.skills.technical.penalty_taking,
//                     tackling: player.skills.technical.tackling,
//                     technique: player.skills.technical.technique
//                 },
//                 mental: MentalDto {
//                     aggression: player.skills.mental.aggression,
//                     anticipation: player.skills.mental.anticipation,
//                     bravery: player.skills.mental.bravery,
//                     composure: player.skills.mental.composure,
//                     concentration: player.skills.mental.concentration,
//                     decisions: player.skills.mental.decisions,
//                     determination: player.skills.mental.determination,
//                     flair: player.skills.mental.flair,
//                     leadership: player.skills.mental.leadership,
//                     off_the_ball: player.skills.mental.off_the_ball,
//                     positioning: player.skills.mental.positioning,
//                     teamwork: player.skills.mental.teamwork,
//                     vision: player.skills.mental.vision,
//                     work_rate: player.skills.mental.work_rate,
//                 },
//                 physical: PhysicalDto {
//                     acceleration: player.skills.physical.acceleration,
//                     agility: player.skills.physical.agility,
//                     balance: player.skills.physical.balance,
//                     jumping_reach: player.skills.physical.jumping_reach,
//                     natural_fitness: player.skills.physical.natural_fitness,
//                     pace: player.skills.physical.pace,
//                     stamina: player.skills.physical.stamina,
//                     strength: player.skills.physical.strength,
//                     match_readiness: player.skills.physical.match_readiness
//                 }
//             }
//         }
//     };
// 
//     Ok(HttpResponse::Ok().json(result))
// }
