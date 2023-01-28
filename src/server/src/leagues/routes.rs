use crate::leagues::league_get_action;
use crate::GameAppData;
use axum::routing::get;
use axum::Router;

pub fn league_routes() -> Router<GameAppData> {
    Router::new().route("/leagues/{league_slug}", get(league_get_action))
}
