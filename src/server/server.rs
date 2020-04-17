use crate::server::{index_action, process_action};
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
                .service(web::resource("/{game_id}").route(web::get().to(process_action)))
                .service(web::resource("/").route(web::get().to(index_action)))
        })
        .bind(self.bind_address)
        .unwrap()
        .run()
        .await
        .unwrap()
    }
}
