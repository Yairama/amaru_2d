use super::wall::BOTTOM_WALL;
use bevy::prelude::{Color, Component, Vec2};

// paddle
pub(crate) const PADDLE_START_Y: f32 = BOTTOM_WALL + 60.0;
pub(crate) const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
pub(crate) const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub(crate) const PADDLE_SPEED: f32 = 500.0;

#[derive(Component)]
pub(crate) struct Paddle;
