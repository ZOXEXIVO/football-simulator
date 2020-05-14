use crate::server::{game_process_action, club_list_action, players_list_action, league_list_action, country_list_action, 
                    club_get_action, game_list_action, game_create_action, league_get_action};
use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;
use crate::simulator::SimulatorData;
use std::sync::Mutex;
use chashmap::*;
use actix_web::middleware::Logger;

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
                .service(web::resource("/api/game/{game_id}/countries").route(web::get().to(country_list_action)))
                .service(web::resource("/api/game/{game_id}/leagues").route(web::get().to(league_list_action)))
                .service(web::resource("/api/game/{game_id}/league/{league_id}").route(web::get().to(league_get_action)))
                .service(web::resource("/api/game/{game_id}/clubs").route(web::get().to(club_list_action)))
                .service(web::resource("/api/game/{game_id}/club/{club_id}").route(web::get().to(club_get_action)))
                .service(web::resource("/api/game/{game_id}/players").route(web::get().to(players_list_action)))             
        })
        .bind(self.bind_address)
        .unwrap()
        .run()
        .await
        .unwrap()
    }
}
