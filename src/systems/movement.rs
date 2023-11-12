use crate::components::powerup::PowerUpState;
use crate::components::{
    paddle::Paddle,
    wall::{LEFT_WALL, RIGHT_WALL, WALL_THICKNESS},
};
use bevy::prelude::*;

pub(crate) fn move_paddle_with_mouse(
    mut query: Query<&mut Transform, With<Paddle>>,
    power_up_state: Res<PowerUpState>,
    window: Query<&Window>,
) {
    let mut paddle_transform = query.single_mut();
    let window = window.single();
    let paddle_width = power_up_state.paddle_size.get_width();

    if let Some(cursor_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let paddle_offset = Vec2::ZERO;
        let mut new_x = cursor_position.x - window_size.x / 2.0;
        new_x = new_x.min(RIGHT_WALL - (WALL_THICKNESS + paddle_width) * 0.5);
        new_x = new_x.max(LEFT_WALL + (WALL_THICKNESS + paddle_width) * 0.5);
        paddle_transform.translation.x = new_x + paddle_offset.x;
    }
}
