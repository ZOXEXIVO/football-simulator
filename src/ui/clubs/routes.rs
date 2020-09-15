// use actix_web::web::ServiceConfig;
// use actix_web::{web};
// use crate::server::{club_list_action, club_get_action};
// 
// pub fn club_routes(cfg: &mut ServiceConfig) {
//     cfg.service(web::resource("/api/game/{game_id}/clubs").route(web::get().to(club_list_action)))
//         .service(web::resource("/api/game/{game_id}/club/{club_id}").route(web::get().to(club_get_action)));
// }