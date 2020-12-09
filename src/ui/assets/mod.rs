use actix_web::web::ServiceConfig;
use actix_web::{web, Result, HttpResponse};

const CSS_CONTENT_TYPE: &'static str = "text/css; charset=utf-8";

pub const FONT_CSS: &[u8] = include_bytes!("fonts.css.gz");
pub const STYLE_CSS: &[u8] = include_bytes!("styles.css");
pub const IMAGES_CSS: &[u8] = include_bytes!("images.css");
pub const FLAG_ICONS_GZIPPED_CSS: &[u8] = include_bytes!("flags.css.gz");

pub fn static_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/styles").route(web::get().to(serve_styles)));
    cfg.service(web::resource("/images").route(web::get().to(serve_images)));
    cfg.service(web::resource("/fonts").route(web::get().to(serve_fonts)));
    cfg.service(web::resource("/flags-icons").route(web::get().to(serve_flags_css)));
}

pub async fn serve_styles() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", CSS_CONTENT_TYPE)
        .body(STYLE_CSS))
}

pub async fn serve_images() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", CSS_CONTENT_TYPE)
        .body(IMAGES_CSS))
}

pub async fn serve_fonts() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", CSS_CONTENT_TYPE)
        .header("Content-Encoding", "gzip")
        .body(FONT_CSS))
}

pub async fn serve_flags_css() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .header("Content-Type", CSS_CONTENT_TYPE)
        .header("Content-Encoding", "gzip")
        .body(FLAG_ICONS_GZIPPED_CSS))
}