use core::r#match::ball::Ball;
use core::r#match::player::MatchPlayer;
use core::r#match::FootballEngine;
use core::r#match::MatchContext;
use core::r#match::MatchField;
use macroquad::prelude::*;
use core::Vector2;

//tactics
use core::club::player::Player;
use core::club::player::PlayerPositionType;
use core::club::team::tactics::{Tactics, TacticsPositioning};
use core::r#match::squad::TeamSquad;
use core::r#match::MatchObjectsPositions;
use core::r#match::MatchPlayerCollection;
use std::time::Instant;
use env_logger::Env;
use core::Vector3;

use core::NaiveDate;
use core::PlayerGenerator;
use core::r#match::PlayerSide;

#[macroquad::main(window_conf)]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let width = screen_width();
    let height = screen_height();

    let field_width = 840.0;
    let field_height = 545.0;

    // Define an offset to center the field
    let offset_x = (width - field_width) / 2.0;
    let offset_y = (height - field_height) / 2.0;

    //840, 545
    let mut ball = Ball::with_coord(field_width / 2.0, field_height / 2.0);

    let home_squad = get_home_squad();
    let away_squad = get_away_squad();

    let players = MatchPlayerCollection::from_squads(&home_squad, &away_squad);

    let mut field = MatchField::new(field_width as usize, field_height as usize, home_squad, away_squad);

    let field_size = field.size.clone();

    let mut context = MatchContext::new(&field_size, players, 0, 0);

    let mut current_frame = 0u64;

    const average_fps_bucket_size: usize = 50;

    let mut max_fps: u128 = 0;

    let mut fps_data = [0u128; average_fps_bucket_size];

    loop {
        current_frame += 1;

        clear_background(Color::new(255.0, 238.0, 7.0, 65.0));

        draw_rectangle_ex(offset_x , offset_y, field_width, field_height, DrawRectangleParams {
            color: Color::from_rgba(189, 255, 204, 255),
            offset: Vec2 {
                x: 0.0,
                y: 0.0,
            },
            rotation: 0.0,
        });

        draw_circle(offset_x + ball.position.x, offset_y + ball.position.y, 7.0, BLACK);

        let start = Instant::now();

        FootballEngine::<840, 545>::game_tick(&mut field, &mut context);

        let elapsed = start.elapsed();
        let fps_data_current_idx = (current_frame % average_fps_bucket_size as u64) as usize;

        let elapsed_mcs = elapsed.as_micros() as u128;

        fps_data[fps_data_current_idx] = elapsed.as_micros() as u128;

        if current_frame > 100 && elapsed_mcs > max_fps {
            max_fps = elapsed_mcs;
        }

        draw_text(
            &format!("FPS AVG: {} mcs", average(&fps_data)),
            offset_x + 10.0,
            offset_y + 20.0,
            20.0,
            BLACK,
        );

        draw_text(
            &format!("FPS MAX: {} mcs", max_fps),
            offset_x + 10.0,
            offset_y + 40.0,
            20.0,
            BLACK,
        );

        field.players.iter().for_each(|player| {
            let mut color = if player.side.unwrap() == PlayerSide::Left {
                Color::from_rgba(0, 184, 186, 255)
            } else {
                Color::from_rgba(208, 139, 255, 255)
            };

            if player.tactics_position == PlayerPositionType::Goalkeeper {
                color = YELLOW;
            }

            draw_circle(offset_x + player.position.x, offset_y + player.position.y, 16.0, color);
            draw_text(
                &player.tactics_position.get_short_name(),
                offset_x + player.position.x - 8.0,
                offset_y + player.position.y + 4.0,
                19.0,
                BLACK,
            );

            draw_text(
                &format!("{} ({})", player_state(player), distance(&ball.position, &player.position)) ,
                offset_x + player.position.x - 27.0,
                offset_y + player.position.y + 27.0,
                15.0,
                DARKGRAY,
            );
        });

        ball.update(&mut context);

        field.ball.position = ball.position;
        field.ball.velocity = ball.velocity;

        next_frame().await
    }
}

pub fn get_home_squad() -> TeamSquad {
    let players = [
        get_player(PlayerPositionType::Goalkeeper),
        get_player(PlayerPositionType::DefenderLeft),
        get_player(PlayerPositionType::DefenderCenter),
        get_player(PlayerPositionType::DefenderCenter),
        get_player(PlayerPositionType::DefenderRight),
        get_player(PlayerPositionType::MidfielderLeft),
        get_player(PlayerPositionType::MidfielderCenter),
        get_player(PlayerPositionType::MidfielderCenter),
        get_player(PlayerPositionType::MidfielderRight),
        get_player(PlayerPositionType::ForwardLeft),
        get_player(PlayerPositionType::ForwardRight),
    ];

    let match_players: Vec<MatchPlayer> = players
        .iter()
        .map(|player| MatchPlayer::from_player(1, player, player.position()))
        .collect();

    let home_squad = TeamSquad {
        team_id: 1,
        team_name: String::from("123"),
        tactics: Tactics::new(TacticsPositioning::T442),
        main_squad: match_players,
        substitutes: Vec::new(),
    };

    home_squad
}

pub fn get_away_squad() -> TeamSquad {
    let players = [
        get_player(PlayerPositionType::Goalkeeper),
        get_player(PlayerPositionType::DefenderLeft),
        get_player(PlayerPositionType::DefenderCenter),
        get_player(PlayerPositionType::DefenderCenter),
        get_player(PlayerPositionType::DefenderRight),
        get_player(PlayerPositionType::MidfielderLeft),
        get_player(PlayerPositionType::MidfielderCenter),
        get_player(PlayerPositionType::MidfielderCenter),
        get_player(PlayerPositionType::MidfielderRight),
        get_player(PlayerPositionType::ForwardLeft),
        get_player(PlayerPositionType::ForwardRight),
    ];

    let match_players: Vec<MatchPlayer> = players
        .iter()
        .map(|player| MatchPlayer::from_player(2, player, player.position()))
        .collect();

    let away_squad = TeamSquad {
        team_id: 2,
        team_name: String::from("321"),
        tactics: Tactics::new(TacticsPositioning::T442),
        main_squad: match_players,
        substitutes: Vec::new(),
    };

    away_squad
}

fn get_player(position: PlayerPositionType) -> Player {
    PlayerGenerator::generate(
        1,
        NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
        position,
        20,
    )
}

fn average(numbers: &[u128]) -> u128 {
    let sum: u128 = numbers.iter().sum();
    let count = numbers.len() as u128;
    sum / count
}

fn player_state(player: &MatchPlayer) -> String {
    let state = player.state.to_string();

    let cleaned_state = state.split(':').nth(1).unwrap_or(&state).trim();

    return cleaned_state.to_string();
}

fn distance(a: &Vector3<f32>, b: &Vector3<f32>) -> usize {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt() as usize
}

fn window_conf() -> Conf {
    Conf {
        window_title: "FootballSimulatorTesting".to_owned(),
        window_width: 1624,
        window_height: 1268,
        window_resizable: false,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}
