use actix_web::web::ServiceConfig;
use actix_web::{web};
use crate::ui::league_get_action;

pub fn league_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/leagues/{league_id}").route(web::get().to(league_get_action)));
}

