use actix_web::{HttpResponse, Result};
use crate::server::{GAMES};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GameListResponse {
    pub games: Vec<GameDto>
}

#[derive(Serialize)]
pub struct GameDto {
    pub id: String,
    pub date: String,
}

pub async fn game_list_action() -> Result<HttpResponse> {
    let games_list = GAMES.lock().unwrap();

    let mut result = GameListResponse{
        games: Vec::with_capacity(games_list.len())
    };

    for (id, date) in games_list.iter() {
        result.games.push(GameDto{
            id: id.clone(),
            date: date.clone()
        })
    }

    Ok(HttpResponse::Ok().json(result))
}
