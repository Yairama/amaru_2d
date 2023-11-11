use crate::components::{
    paddle::{Paddle, PADDLE_SIZE},
    wall::{LEFT_WALL, RIGHT_WALL, WALL_THICKNESS},
};
use bevy::prelude::*;
pub(crate) fn move_paddle_with_mouse(
    mut query: Query<&mut Transform, With<Paddle>>,
    window: Query<&Window>,
) {
    let mut paddle_transform = query.single_mut();
    let window = window.single();

    if let Some(cursor_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let paddle_offset = Vec2::ZERO;
        let mut new_x = cursor_position.x - window_size.x / 2.0;
        new_x = new_x.min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
        new_x = new_x.max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
        paddle_transform.translation.x = new_x + paddle_offset.x;
    }
}
