use actix_web::web::ServiceConfig;
use actix_web::{web};
use crate::ui::{game_create_action, game_process_action};

pub fn game_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/game/create").route(web::post().to(game_create_action)))
    .service(web::resource("/game/process").route(web::post().to(game_process_action)));
}