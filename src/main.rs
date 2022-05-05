use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 4.0;
const PLAYER_HEIGHT: f32 = 150.0;
const BOUNCE_DISTANCE: f32 = 70.0;
const START_POSITION_x: f32 = 395.0;
const START_BALL_SPEED: f32 = 3.7;

#[macroquad::main("BasicShapes")]
async fn main() {
    let background = load_texture("pong.png").await.unwrap();

    let mut ball_speed: f32 = START_BALL_SPEED;

    let mut ball_x = START_POSITION_x;
    let mut ball_y = 130.0;
    let mut ball_speed_x = ball_speed;
    let mut ball_speed_y = -3.0;

    let mut player_1 = 150.0;
    let mut player_2 = 140.0;
    let mut over_line = 30.0;
    let mut under_line = 570.0;

    let mut player_points_1 = 0;
    let mut player_points_2 = 0;

    loop {
        clear_background(WHITE);
        draw_texture_ex(
            background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // update ball position
        ball_x += ball_speed_x;
        ball_y += ball_speed_y;

        // studskod
        if ball_x > screen_width() - BOUNCE_DISTANCE {
            if player_2 < ball_y && player_2 + PLAYER_HEIGHT > ball_y {
                ball_speed_x = -ball_speed;
                ball_speed += 0.5;
            } else {
                ball_x = START_POSITION_x;
                ball_speed = START_BALL_SPEED;
                ball_speed_x = -ball_speed;
                player_points_1 += 1;
            }
        }
        if ball_x < BOUNCE_DISTANCE + 10.0 {
            if player_1 < ball_y && player_1 + PLAYER_HEIGHT > ball_y {
                ball_speed_x = ball_speed;
                ball_speed += 0.5;
            } else {
                ball_speed_x = ball_speed;
                ball_speed = START_BALL_SPEED;
                ball_x = START_POSITION_x;
                player_points_2 += 1;
            }
        }
        if ball_y < over_line + 15.0 {
            ball_speed_y = ball_speed;
        }
        if ball_y > under_line - 12.0 {
            ball_speed_y = -ball_speed;
        }

        draw_text(
            &format!("spelare 1: {player_points_1}    spelare 2: {player_points_2}"),
            200.0,
            20.0,
            10.0,
            DARKGRAY,
        );
        if player_points_1 > 4 {
            draw_text(
                &format!("GRATTIS SPELARE 1 VANN"),
                200.0,
                100.0,
                30.0,
                VIOLET,
            );
        }
        if player_points_2 > 4 {
            draw_text(
                &format!("GRATTIS SPELARE 2 VANN"),
                200.0,
                100.0,
                30.0,
                DARKPURPLE,
            );
        }

        // player input
        if macroquad::input::is_key_down(KeyCode::W) {
            player_1 -= PLAYER_SPEED;
        }
        if macroquad::input::is_key_down(KeyCode::S) {
            player_1 += PLAYER_SPEED;
        }

        if macroquad::input::is_key_down(KeyCode::Up) {
            player_2 -= PLAYER_SPEED;
        }
        if macroquad::input::is_key_down(KeyCode::Down) {
            player_2 += PLAYER_SPEED;
        }

        draw_circle(ball_x, ball_y, 20.0, VIOLET);
        draw_line(50.0, player_1, 50.0, player_1 + PLAYER_HEIGHT, 10.0, BLACK);
        draw_line(
            750.0,
            player_2,
            750.0,
            player_2 + PLAYER_HEIGHT,
            10.0,
            BLACK,
        );
        draw_line(80.0, over_line, 720.0, over_line, 10.0, BLACK); //övre linjen
        draw_line(80.0, under_line, 720.0, under_line, 10.0, BLACK); //undre linjen
        draw_line(395.0, 30.0, 395.0, 570.0, 10.0, BLACK); //mitten linjen
        next_frame().await
    }
}
