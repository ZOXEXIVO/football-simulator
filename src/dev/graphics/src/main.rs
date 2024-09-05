use core::r#match::ball::Ball;
use core::r#match::player::MatchPlayer;
use core::r#match::FootballEngine;
use core::r#match::MatchContext;
use core::r#match::MatchField;
use macroquad::prelude::*;
//tactics
use core::club::player::Player;
use core::club::player::PlayerPositionType;
use core::club::team::tactics::{Tactics, TacticsPositioning};
use core::r#match::squad::TeamSquad;
use core::r#match::MatchObjectsPositions;
use core::r#match::MatchPlayerCollection;
use std::time::Instant;

use core::NaiveDate;
use core::PlayerGenerator;

#[macroquad::main("FootballSimulatorTesting")]
async fn main() {
    let width = screen_width();
    let height = screen_height();

    //840, 545
    let mut ball = Ball::with_coord(500.0, 500.0);

    let home_squad = get_home_squad();
    let away_squad = get_away_squad();

    let players = MatchPlayerCollection::from_squads(&home_squad, &away_squad);

    let mut field = MatchField::new(width as usize, height as usize, home_squad, away_squad);

    let field_size = field.size.clone();

    let mut context = MatchContext::new(&field_size, players);

    let mut current_frame = 0u64;

    const average_fps_bucket_size: usize = 50;

    let mut max_fps: u128 = 0;

    let mut fps_data = [0u128; average_fps_bucket_size];

    loop {
        current_frame += 1;

        clear_background(Color::new(255.0, 238.0, 7.0, 65.0));

        draw_circle(ball.position.x, ball.position.y, 7.0, BLACK);

        let start = Instant::now();

        FootballEngine::<840, 545>::game_tick(&mut field, &mut context);

        let elapsed = start.elapsed();
        let fps_data_current_idx = (current_frame % average_fps_bucket_size as u64) as usize;

        let elapsed_mcs = elapsed.as_micros() as u128;

        fps_data[fps_data_current_idx] = elapsed.as_micros() as u128;

        if average_fps_bucket_size > 10 && elapsed_mcs > max_fps {
            max_fps = elapsed_mcs;
        }

        draw_text(
            &format!("FPS AVG: {} mcs", average(&fps_data)),
            10.0,
            20.0,
            20.0,
            BLACK,
        );

        draw_text(
            &format!("FPS MAX: {} mcs", max_fps),
            10.0,
            40.0,
            20.0,
            BLACK,
        );

        field.players.iter().for_each(|player| {
            let mut color = if player.is_home {
                Color::from_rgba(0, 184, 186, 255)
            } else {
                Color::from_rgba(208, 139, 255, 255)
            };

            if player.tactics_position == PlayerPositionType::Goalkeeper {
                color = YELLOW;
            }

            draw_circle(player.position.x, player.position.y, 13.0, color);
            draw_text(
                &player.tactics_position.get_short_name(),
                player.position.x - 7.0,
                player.position.y + 3.0,
                15.0,
                BLACK,
            );

            draw_text(
                &player.state.to_string(),
                player.position.x - 20.0,
                player.position.y + 20.0,
                10.0,
                DARKGRAY,
            );
        });

        ball.update(&mut context);

        if ball.position.x < 10.0 {
            ball.velocity.x = -ball.velocity.x;
        }

        if ball.position.x > 800.0 {
            ball.velocity.x = -ball.velocity.x;
        }

        if ball.position.y < 10.0 {
            ball.velocity.y = -ball.velocity.y;
        }

        if ball.position.y > 400.0 {
            ball.velocity.y = -ball.velocity.y;
        }

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
