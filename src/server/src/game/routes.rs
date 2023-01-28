use crate::game::{game_create_action, game_process_action};
use crate::GameAppData;
use axum::routing::get;
use axum::Router;

pub fn game_routes() -> Router<GameAppData> {
    Router::new()
        .route("/game/create", get(game_create_action))
        .route("/game/process", get(game_process_action))
}
