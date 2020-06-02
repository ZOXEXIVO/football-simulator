use actix_web::web::ServiceConfig;
use actix_web::{web};
use crate::server::{country_list_action, country_get_action};

pub fn country_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/api/game/{game_id}/countries").route(web::get().to(country_list_action)))
        .service(web::resource("/api/game/{game_id}/countries/{country_id}").route(web::get().to(country_get_action)));
}

