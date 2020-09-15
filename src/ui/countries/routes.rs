use actix_web::web::ServiceConfig;
use actix_web::{web};
use crate::ui::country_list_action;

pub fn country_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/countries").route(web::get().to(country_list_action)));
       // .service(web::resource("/countries/{country_id}").route(web::get().to(country_get_action)));
}

