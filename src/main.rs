mod db;
mod ui;

use crate::db::{DatabaseEntity, DatabaseLoader};
use crate::ui::assets::static_routes;
use actix_files::Files;
use actix_web::{App, HttpServer};
use core::utils::TimeEstimation;
use core::SimulatorData;
use env_logger::Env;
use log::info;
use parking_lot::Mutex;
use std::sync::Arc;
use ui::*;

pub struct GameAppData {
    database: Arc<DatabaseEntity>,
    data: Arc<Mutex<Option<SimulatorData>>>,
}

impl Clone for GameAppData {
    fn clone(&self) -> Self {
        GameAppData {
            database: Arc::clone(&self.database),
            data: self.data.clone(),
        }
    }
}

#[actix_web::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let (database, estimated) = TimeEstimation::estimate(|| DatabaseLoader::load());

    info!("database loaded: {} ms", estimated);

    let data = GameAppData {
        database: Arc::new(database),
        data: Arc::new(Mutex::new(None)),
    };

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .service(Files::new("/assets", "src/ui/assets").show_files_listing())
            .configure(static_routes)
            .configure(index_routes)
            .configure(game_routes)
            .configure(country_routes)
            .configure(league_routes)
            .configure(team_routes)
            .configure(player_routes)
    })
    .bind("0.0.0.0:18000")
    .unwrap()
    .run()
    .await
    .unwrap()
}
