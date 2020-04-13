use crate::server::index_action;
use actix_web::{web, App, HttpServer};

pub struct Server {
    bind_address: &'static str,
}

impl Server {
    pub fn new(bind_address: &'static str) -> Self {
        Server { bind_address }
    }

    pub async fn start(&self) {
        HttpServer::new(move || {
            App::new().service(web::resource("/").route(web::get().to(index_action)))
        })
        .bind(self.bind_address)
        .unwrap()
        .run()
        .await
        .unwrap()
    }
}
