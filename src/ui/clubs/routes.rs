use actix_web::web::ServiceConfig;
use actix_web::{web};
use crate::ui::club_get_action;

pub fn club_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/clubs/{club_id}").route(web::get().to(club_get_action)));
}