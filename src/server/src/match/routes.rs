use crate::r#match::data::match_get_action;
use crate::GameAppData;
use axum::routing::get;
use axum::Router;

pub fn match_routes() -> Router<GameAppData> {
    Router::new()
        .route("/api/match/:league_slug/:match_id", get(match_get_action))
        .route("/api/match/:league_slug/:match_id/data", get(match_get_action))
}
