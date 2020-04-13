use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};

pub async fn index_action() -> Result<HttpResponse> {
    let body = "OK";
    
    Ok(HttpResponse::Ok().body(body))
}
