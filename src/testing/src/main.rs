use core::r#match::ball::Ball;
use core::r#match::player::MatchPlayer;
use core::r#match::MatchContext;
use core::r#match::MatchField;
use macroquad::prelude::*;
//tactics
use core::club::player::Player;
use core::club::player::PlayerAttributes;
use core::club::player::PlayerPositionType;
use core::club::player::PlayerSkills;
use core::club::team::tactics::{Tactics, TacticsPositioning};
use core::country::country::*;
use core::r#match::squad::TeamSquad;
use core::r#match::MatchObjectsPositions;

use core::NaiveDate;
use core::PeopleNameGeneratorData;
use core::PlayerGenerator;

#[macroquad::main("FootballSimulatorTesting")]
async fn main() {
    let width = screen_width();
    let height = screen_height();

    //840, 545
    let mut ball = Ball::with_coord(width / 2.0, height / 2.0);

    let player = PlayerGenerator::generate(
        1,
        NaiveDate::from_ymd(2021, 1, 1),
        PlayerPositionType::Striker,
        1,
    );

    let mut player = MatchPlayer::from_player(&player, PlayerPositionType::Striker);

    let home_squad = TeamSquad {
        team_id: 1,
        team_name: String::from("123"),
        tactics: Tactics::new(TacticsPositioning::T442),
        main_squad: vec![player],
        substitutes: Vec::new(),
    };

    let mut field = MatchField::new(
        width as usize,
        height as usize,
        home_squad.clone(),
        home_squad.clone(),
    );

    let mut context = MatchContext::new(&field.size);

    loop {
        clear_background(Color::new(255.0, 238.0, 7.0, 65.0));

        draw_circle(ball.position.x, ball.position.y, 15.0, GREEN);
        draw_circle(player.position.x, player.position.y, 15.0, RED);

        ball.update(&mut context);

        player.update(&mut context, &MatchObjectsPositions::from(&field));

        println!("player: {:?}", player.position);

        next_frame().await
    }
}
