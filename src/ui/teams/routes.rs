use actix_web::web::ServiceConfig;
use actix_web::{web};
use crate::ui::team_get_action;

pub fn team_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/teams/{team_id}").route(web::get().to(team_get_action)));
}