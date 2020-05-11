use crate::server::{game_process_action, club_list_action, players_list_action, continent_list_action, 
                    league_list_action, country_list_action, club_get_action, game_list_action, game_create_action};
use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;
use crate::simulator::SimulatorData;
use std::sync::Mutex;
use chashmap::*;
use chrono::NaiveDateTime;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::http::header::{ACCEPT, CONTENT_TYPE};

lazy_static!{
    pub static ref GAMES: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
    pub static ref GLOBAL_DATA: CHashMap<String, Mutex<SimulatorData>> = CHashMap::new();
}

pub struct Server {
    bind_address: &'static str,
}

impl Server {
    pub fn new(bind_address: &'static str) -> Self {
        Server { bind_address }
    }

    pub async fn start(&self) {
        std::env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();
        
        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::new()
                        .max_age(3600)
                        .finish(),
                ).wrap(Logger::default())                
                .service(web::resource("/games").route(web::get().to(game_list_action)))
                .service(web::resource("/games/create").route(web::post().to(game_create_action)))
                .service(web::resource("/{game_id}").route(web::get().to(game_process_action)))
                .service(web::resource("/{game_id}/continents").route(web::get().to(continent_list_action)))
                .service(web::resource("/{game_id}/countries").route(web::get().to(country_list_action)))
                .service(web::resource("/{game_id}/leagues").route(web::get().to(league_list_action)))
                .service(web::resource("/{game_id}/clubs").route(web::get().to(club_list_action)))
                .service(web::resource("/{game_id}/clubs/{club_id}").route(web::get().to(club_get_action)))
                .service(web::resource("/{game_id}/players").route(web::get().to(players_list_action)))             
        })
        .bind(self.bind_address)
        .unwrap()
        .run()
        .await
        .unwrap()
    }
}
