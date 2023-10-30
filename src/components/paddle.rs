
use super::wall::BOTTOM_WALL;
use bevy::prelude::*;
use bevy_editor_pls::egui::layers::PaintList;

// paddle
pub(crate) const PADDLE_START_Y: f32 = BOTTOM_WALL + 60.0;
pub(crate) const PADDLE_SIZE: Vec2 = Vec2::new(64.0, 16.0);
pub(crate) const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub(crate) const PADDLE_SPEED: f32 = 500.0;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum PaddleColor {
    Red,
    Blue,
    Yellow,
    Green,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum PaddleType {
    Standard,
    Shooter
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum PaddleSize {
    XS,
    S,
    M,
    L,
    XL,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct PaddleSprite {
    pub(crate) index:usize
}

#[derive(Component)]
pub(crate) struct Paddle;
