use crate::ui::player_get_action;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn player_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/teams/{team_slug}/players/{player_id}")
            .route(web::get().to(player_get_action)),
    );
}
