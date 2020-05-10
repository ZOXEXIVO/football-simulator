use actix_web::{HttpResponse, Result};
use crate::server::{GAMES};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct GameListRequest {
}

#[derive(Serialize)]
pub struct GameListResponse {
    pub games: Vec<GameDto>
}

#[derive(Serialize)]
pub struct GameDto {
    pub id: String,
    pub date_time: String,
}

pub async fn game_list_action() -> Result<HttpResponse> {
    let games_list = GAMES.lock().unwrap();

    let mut result = GameListResponse{
        games: Vec::with_capacity(games_list.len())
    };

    for (id, date) in games_list.iter() {
        result.games.push(GameDto{
            id: id.clone(),
            date_time: date.clone()
        })
    }

    Ok(HttpResponse::Ok().json(result))
}
