use crate::GameAppData;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use askama::Template;
use core::{PlayerPositionType, SimulatorData, Team};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MatchPlayGetRequest {
    pub league_slug: String,
    pub match_id: String,
}

#[derive(Template)]
#[template(path = "match/play/play.html")]
pub struct MatchPlayGetViewModel<'s> {
    pub league_slug: &'s str,
    pub match_id: &'s str,
}

pub async fn match_play_get_action(
    route_params: web::Path<MatchPlayGetRequest>,
) -> Result<HttpResponse> {
    let model = MatchPlayGetViewModel {
        league_slug: &route_params.league_slug,
        match_id: &route_params.match_id,
    };

    let html = MatchPlayGetViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
