use crate::server::{game_process_action, club_list_action, players_list_action, country_list_action, club_get_action, game_list_action, game_create_action, league_get_action, country_get_action, league_routes, country_routes, club_routes, player_routes};
use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;
use crate::simulator::SimulatorData;
use std::sync::Mutex;
use chashmap::*;

lazy_static!{
    pub static ref GAMES: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
    pub static ref GLOBAL_DATA: CHashMap<String, SimulatorData> = CHashMap::new();
}

pub struct Server {
    bind_address: &'static str,
}

impl Server {
    pub fn new(bind_address: &'static str) -> Self {
        Server { bind_address }
    }

    pub async fn start(&self) {
        //std::env::set_var("RUST_LOG", "actix_web=info");
        //env_logger::init();
        
        HttpServer::new(move || {
            App::new()//.wrap(Logger::default())                
                .service(web::resource("/api/game").route(web::get().to(game_list_action)))
                .service(web::resource("/api/game/create").route(web::post().to(game_create_action)))
                .service(web::resource("/api/game/{game_id}/process").route(web::post().to(game_process_action)))
                .configure(country_routes)
                .configure(league_routes)
                .configure(club_routes)
                .configure(player_routes)
        })
        .bind(self.bind_address)
        .unwrap()
        .run()
        .await
        .unwrap()
    }
}
