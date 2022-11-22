use crate::ui::league_get_action;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn league_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/leagues/{league_slug}").route(web::get().to(league_get_action)));
}
