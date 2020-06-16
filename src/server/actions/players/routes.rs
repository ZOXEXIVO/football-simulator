use actix_web::web::ServiceConfig;
use actix_web::{web};
use crate::server::{players_list_action, player_get_action};

pub fn player_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/api/game/{game_id}/club/{club_id}/players").route(web::get().to(players_list_action)));
    cfg.service(web::resource("/api/game/{game_id}/club/{club_id}/players/{player_id}").route(web::get().to(player_get_action)));    
}