mod db;
mod ui;

use crate::db::{DatabaseEntity, DatabaseLoader};
use crate::ui::assets::static_routes;
use actix_files::Files;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use core::utils::TimeEstimation;
use core::SimulatorData;
use env_logger::Env;
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;
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
    color_eyre::install().unwrap();

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let (database, estimated) = TimeEstimation::estimate(DatabaseLoader::load);

    info!("database loaded: {} ms", estimated);

    let data = GameAppData {
        database: Arc::new(database),
        data: Arc::new(Mutex::new(None)),
    };

    info!("listen at: http://localhost:18000");

    const STATIC_ASSETS_DEBUG_FOLDER: &str = "src/ui/assets";

    HttpServer::new(move || {
        let mut data = App::new().app_data(Data::new(data.clone()));

        if std::path::Path::new(&STATIC_ASSETS_DEBUG_FOLDER).exists() {
            data = data
                .service(Files::new("/assets", STATIC_ASSETS_DEBUG_FOLDER).show_files_listing());
        }

        data = data
            .configure(static_routes)
            .configure(index_routes)
            .configure(game_routes)
            .configure(country_routes)
            .configure(league_routes)
            .configure(team_routes)
            .configure(player_routes);

        data
    })
    .bind("0.0.0.0:18000")
    .unwrap()
    .run()
    .await
    .unwrap()
}
