use crate::ui::{team_get_action, team_schedule_get_action};
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn team_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/teams/{team_id}").route(web::get().to(team_get_action)))
        .service(
            web::resource("/teams/{team_id}/schedule")
                .route(web::get().to(team_schedule_get_action)),
        );
}
