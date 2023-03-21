use crate::countries::country_routes;
use crate::date::current_date_routes;
use crate::game::game_routes;
use crate::leagues::league_routes;
use crate::player::player_routes;
use crate::r#match::routes::match_routes;
use crate::teams::team_routes;
use crate::GameAppData;
use axum::routing::get_service;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub struct ServerRoutes;

impl ServerRoutes {
    pub fn create() -> Router<GameAppData> {
        let routes = Router::<GameAppData>::new()
            .merge(country_routes())
            .merge(game_routes())
            .merge(league_routes())
            .merge(team_routes())
            .merge(player_routes())
            .merge(match_routes())
            .merge(current_date_routes());

        #[cfg(debug_assertions)]
        let client_app_dir = "ui/dist";
        #[cfg(debug_assertions)]
        let client_app_index_file = "./ui/dist/index.html";

        #[cfg(not(debug_assertions))]
        let client_app_dir = "dist";
        #[cfg(not(debug_assertions))]
        let client_app_index_file = "dist/index.html";

        Router::new()
            .fallback(get_service(ServeFile::new(client_app_index_file)))
            .merge(routes)
            .nest_service(
                "/dist",
                ServeDir::new(client_app_dir).fallback(ServeFile::new(client_app_index_file)),
            )
    }
}
