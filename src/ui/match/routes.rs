use crate::ui::r#match::details::match_details_action;
use crate::ui::r#match::match_play_get_action;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn match_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/match/{league_id}/{match_id}").route(web::get().to(match_play_get_action)),
    );
    cfg.service(
        web::resource("/match/{league_id}/{match_id}/details")
            .route(web::get().to(match_details_action)),
    );
}
