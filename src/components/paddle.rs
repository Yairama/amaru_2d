use super::wall::BOTTOM_WALL;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Vect;

// paddle
pub(crate) const PADDLE_START_Y: f32 = BOTTOM_WALL + 60.0;
pub(crate) const PADDLE_SIZE: Vec2 = Vec2::new(64.0, 16.0);
pub(crate) const PADDLE_SPEED: f32 = 500.0;

pub const PADDLE_SHAPE: [Vect; 6] = [
    Vect::new(-12., 8.), // x -> Plain zone
    Vect::new(12., 8.), // x -> Plain zone
    Vect::new(32., 2.), // y -> limit of diagonal zone
    Vect::new(32., -8.),
    Vect::new(-32., -8.),
    Vect::new(-32., 2.), // y -> limit of diagonal zone
];

pub const PADDLE_INDICES: [[u32; 2]; 6] = [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 0]];

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
    Shooter,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum PaddleSize {
    XS,
    S,
    M,
    L,
    XL,
}

#[derive(Component)]
pub(crate) struct Paddle;
