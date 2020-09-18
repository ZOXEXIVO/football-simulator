use actix_web::web::ServiceConfig;
use actix_web::{web, Result, HttpResponse};

pub const FONT_CSS: &[u8] = include_bytes!("fonts.css");
pub const STYLE_CSS: &[u8] = include_bytes!("styles.css");

pub fn static_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/styles").route(web::get().to(serve_styles)));
    cfg.service(web::resource("/fonts").route(web::get().to(serve_fonts)));
}

pub async fn serve_styles() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(STYLE_CSS))
}

pub async fn serve_fonts() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(FONT_CSS))
}
