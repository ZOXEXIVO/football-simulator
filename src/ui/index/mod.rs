use crate::GameAppData;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{web, HttpResponse, Result};
use askama::Template;
use serde::{Serialize};

pub fn index_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index_action)))
       .service(web::resource("/current/date").route(web::get().to(current_date_action)));
}

#[derive(Template)]
#[template(path = "index/index.html")]
pub struct IndexViewModel {}

pub async fn index_action(state: Data<GameAppData>) -> Result<HttpResponse> {
    let data = state.data.lock();

    if data.is_some() {
        Ok(HttpResponse::Found()
            .header("Location", "/countries")
            .finish())
    } else {
        let html = IndexViewModel::render(&IndexViewModel {}).unwrap();

        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    }
}

#[derive(Serialize)]
pub struct CurrentDateModel {
    pub date: String,
    pub time: String
}

pub async fn current_date_action(state: Data<GameAppData>) -> Result<HttpResponse> {
    let data = state.data.lock();

    if data.is_none() {
        Ok(HttpResponse::Ok().finish())
    }else {
        let date = data.as_ref().unwrap().date;
        
        let model = CurrentDateModel{
            date: date.format("%d %b %Y").to_string(),
            time: date.format("%a %R").to_string(),
        };

        Ok(HttpResponse::Ok().json(model))
    }
}
