use actix_web::dev::ServiceRequest;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpRequest, HttpResponse, Result};

const CSS_CONTENT_TYPE: &'static str = "text/css; charset=utf-8";
const JS_CONTENT_TYPE: &'static str = "text/javascript; charset=utf-8";
const STATIC_FILES_CACHE_CONTROL_HEADER: &'static str = "public, max-age=15552000";

pub const FONT_CSS: &[u8] = include_bytes!("fonts.css.gz");
pub const STYLE_CSS: &[u8] = include_bytes!("styles.css");
pub const IMAGES_CSS: &[u8] = include_bytes!("images.css");
pub const FLAG_ICONS_GZIPPED_CSS: &[u8] = include_bytes!("flags.css.gz");
pub const GRAPHICS_JS: &[u8] = include_bytes!("scripts/twojs.js");

pub fn static_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/styles").route(web::get().to(serve_styles)));
    cfg.service(web::resource("/images").route(web::get().to(serve_images)));
    cfg.service(web::resource("/images/{type}").route(web::get().to(serve_image)));
    cfg.service(web::resource("/js/graphics").route(web::get().to(serve_graphics_js)));
    cfg.service(web::resource("/fonts").route(web::get().to(serve_fonts)));
    cfg.service(web::resource("/flags-icons").route(web::get().to(serve_flags_css)));
}

pub async fn serve_styles() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", CSS_CONTENT_TYPE))
        .append_header(("Cache-Control", STATIC_FILES_CACHE_CONTROL_HEADER))
        .body(STYLE_CSS))
}

pub async fn serve_images() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", CSS_CONTENT_TYPE))
        .append_header(("Cache-Control", STATIC_FILES_CACHE_CONTROL_HEADER))
        .body(IMAGES_CSS))
}

pub const IMAGE_POLE_JPEG: &[u8] = include_bytes!("images/pole.png");
pub const IMAGE_BALL_JPEG: &[u8] = include_bytes!("images/ball.png");

pub async fn serve_image(req: HttpRequest) -> Result<HttpResponse> {
    let object_data = match req.match_info().get("type").unwrap() {
        "pole" => IMAGE_POLE_JPEG,
        "ball" => IMAGE_BALL_JPEG,
        _ => return Ok(HttpResponse::NotFound().finish()),
    };

    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "image/png"))
        .append_header(("Cache-Control", STATIC_FILES_CACHE_CONTROL_HEADER))
        .body(object_data))
}

pub async fn serve_fonts() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", CSS_CONTENT_TYPE))
        .append_header(("Content-Encoding", "gzip"))
        .append_header(("Cache-Control", STATIC_FILES_CACHE_CONTROL_HEADER))
        .body(FONT_CSS))
}

pub async fn serve_flags_css() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", CSS_CONTENT_TYPE))
        .append_header(("Content-Encoding", "gzip"))
        .append_header(("Cache-Control", STATIC_FILES_CACHE_CONTROL_HEADER))
        .body(FLAG_ICONS_GZIPPED_CSS))
}

pub async fn serve_graphics_js() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", JS_CONTENT_TYPE))
        //.append_header(("Content-Encoding", "gzip"))
        .append_header(("Cache-Control", STATIC_FILES_CACHE_CONTROL_HEADER))
        .body(GRAPHICS_JS))
}
