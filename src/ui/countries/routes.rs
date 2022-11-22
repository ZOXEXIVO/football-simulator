use crate::ui::{country_get_action, country_list_action};
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn country_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/countries").route(web::get().to(country_list_action)))
        .service(
            web::resource("/countries/{country_slug}").route(web::get().to(country_get_action)),
        );
}
