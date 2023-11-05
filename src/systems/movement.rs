use crate::components::{
    paddle::{Paddle, PADDLE_SIZE},
    wall::{LEFT_WALL, RIGHT_WALL, WALL_THICKNESS},
};
use bevy::prelude::*;

// pub(crate) fn move_paddle(
//     input: Res<Input<KeyCode>>,
//     time_step: Res<FixedTime>,
//     mut query: Query<&mut Transform, With<Paddle>>,
// ) {
//     let mut paddle_transform = query.single_mut();
//
//     let mut direction = 0.0;
//     if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
//         direction -= 1.0;
//     }
//
//     if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
//         direction += 1.0;
//     }
//
//     let mut new_x =
//         paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.period.as_secs_f32();
//
//     new_x = new_x.min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
//
//     new_x = new_x.max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
//
//     paddle_transform.translation.x = new_x;
// }

pub(crate) fn move_paddle_with_mouse(
    mut query: Query<&mut Transform, With<Paddle>>,
    window: Query<&Window>,
) {
    let mut paddle_transform = query.single_mut();
    let window = window.single();

    if let Some(cursor_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let paddle_offset = Vec2::ZERO;
        let mut new_x = cursor_position.x - window_size.x / 2.0;
        new_x = new_x.min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
        new_x = new_x.max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
        paddle_transform.translation.x = new_x + paddle_offset.x;
    }
}
