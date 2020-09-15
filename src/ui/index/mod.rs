use actix_web::{web, HttpResponse, Result};
use actix_web::web::{ServiceConfig, Data};
use askama::Template;
use crate::GameAppData;

pub fn index_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index_action)));
}

#[derive(Template)]
#[template(path = "index/index.html")]
pub struct IndexViewModel {   
}

pub async fn index_action(state: Data<GameAppData>) -> Result<HttpResponse> {
    let data = state.data.lock().unwrap();
    
    if data.is_some() {
        Ok(HttpResponse::Found().header("Location", "/countries").finish())
    }
    else {
        let html = IndexViewModel::render(&IndexViewModel{ }).unwrap();

        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    }  
}
