use crate::server::{game_process_action, game_index_action, club_list_action, players_list_action, continent_list_action, league_list_action, country_list_action, club_get_action};
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::simulator::SimulatorData;
use std::sync::Mutex;
use chashmap::*;

lazy_static!{
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
        HttpServer::new(move || {
            App::new()
                .service(web::resource("/{game_id}").route(web::get().to(game_process_action)))
                .service(web::resource("/{game_id}/continents").route(web::get().to(continent_list_action)))
                .service(web::resource("/{game_id}/countries").route(web::get().to(country_list_action)))
                .service(web::resource("/{game_id}/leagues").route(web::get().to(league_list_action)))
                .service(web::resource("/{game_id}/clubs").route(web::get().to(club_list_action)))
                .service(web::resource("/{game_id}/clubs/{club_id}").route(web::get().to(club_get_action)))
                .service(web::resource("/{game_id}/players").route(web::get().to(players_list_action)))            
                .service(web::resource("/").route(web::get().to(game_index_action)))
        })
        .bind(self.bind_address)
        .unwrap()
        .run()
        .await
        .unwrap()
    }
}
