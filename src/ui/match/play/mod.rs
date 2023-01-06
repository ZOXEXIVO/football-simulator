use crate::GameAppData;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use askama::Template;
use core::{PlayerPositionType, SimulatorData, Team};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MatchPlayGetRequest {
    pub match_id: String,
}

#[derive(Template)]
#[template(path = "match/play/play.html")]
pub struct MatchPlayGetViewModel {}

pub async fn match_play_get_action(
    state: Data<GameAppData>,
    route_params: web::Path<MatchPlayGetRequest>,
) -> Result<HttpResponse> {
    // let guard = state.data.lock().await;
    //
    // let simulator_data = guard.as_ref().unwrap();

    let model = MatchPlayGetViewModel {};

    let html = MatchPlayGetViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
