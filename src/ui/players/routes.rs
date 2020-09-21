use actix_web::web::ServiceConfig;
use actix_web::{web};
use crate::ui::player_get_action;

pub fn player_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/clubs/{club_id}/players/{player_id}").route(web::get().to(player_get_action)));    
}