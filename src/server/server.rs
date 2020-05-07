use crate::server::{game_process_action, game_index_action, club_list_action, players_list_action};
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::simulator::SimulatorData;
use std::sync::Mutex;
use std::sync::RwLock;

lazy_static!{
    pub static ref GLOBAL_DATA: RwLock<HashMap<String, Mutex<SimulatorData>>> = RwLock::new(HashMap::new());
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
                .service(web::resource("/{game_id}/clubs").route(web::get().to(club_list_action)))
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
