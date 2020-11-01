mod ui;
mod db;

use ui::*;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_files::Files;
use std::sync::{Arc};
use core::SimulatorData;
use crate::ui::assets::static_routes;
use parking_lot::Mutex;
use crate::db::{DatabaseLoader, DatabaseEntity};
use core::utils::TimeEstimation;

pub struct GameAppData {
    database: Arc<DatabaseEntity>,
    data: Arc<Mutex<Option<SimulatorData>>>
}

impl Clone for GameAppData{
    fn clone(&self) -> Self {
        GameAppData {
            database: Arc::clone(&self.database),
            data: self.data.clone()
        }
    }
}

#[actix_web::main]
async fn main() {
    let (database, estimated) = TimeEstimation::estimate(|| DatabaseLoader::load());

    println!("database loaded: {} ms", estimated);
    
    let data = GameAppData {
        database: Arc::new(database),
        data: Arc::new(Mutex::new(None))
    };
    
    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .service(Files::new("/assets", "src/ui/assets").show_files_listing())
            
            .wrap(Logger::default())
            .configure(static_routes)
            .configure(index_routes)
            .configure(game_routes)        
            .configure(country_routes)
            .configure(league_routes)
            .configure(club_routes)
            .configure(player_routes)
    }).bind("0.0.0.0:18000")
      .unwrap()
      .run()
      .await
      .unwrap()
}
