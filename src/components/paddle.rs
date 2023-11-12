use super::wall::BOTTOM_WALL;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Vect;

// paddle
pub(crate) const PADDLE_START_Y: f32 = BOTTOM_WALL + 60.0;

pub const PADDLE_INDICES: [[u32; 2]; 5] = [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5]];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PaddleColor {
    Red,
    Blue,
    Yellow,
    Green,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PaddleType {
    Standard,
    Shooter,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PaddleSize {
    XS,
    S,
    M,
    L,
    XL,
}

impl PaddleSize {
    pub fn get_shape(self) -> [Vect; 6] {
        match self {
            PaddleSize::XS => {
                [
                    Vect::new(-16., -8.),
                    Vect::new(-16., 2.), // y -> limit of diagonal zone
                    Vect::new(-6., 8.),  // x -> Plain zone
                    Vect::new(6., 8.),   // x -> Plain zone
                    Vect::new(16., 2.),  // y -> limit of diagonal zone
                    Vect::new(16., -8.),
                ]
            }
            PaddleSize::S => {
                [
                    Vect::new(-24., -8.),
                    Vect::new(-24., 2.), // y -> limit of diagonal zone
                    Vect::new(-9., 8.),  // x -> Plain zone
                    Vect::new(9., 8.),   // x -> Plain zone
                    Vect::new(24., 2.),  // y -> limit of diagonal zone
                    Vect::new(24., -8.),
                ]
            }
            PaddleSize::M => {
                [
                    Vect::new(-32., -8.),
                    Vect::new(-32., 2.), // y -> limit of diagonal zone
                    Vect::new(-12., 8.), // x -> Plain zone
                    Vect::new(12., 8.),  // x -> Plain zone
                    Vect::new(32., 2.),  // y -> limit of diagonal zone
                    Vect::new(32., -8.),
                ]
            }
            PaddleSize::L => {
                [
                    Vect::new(-40., -8.),
                    Vect::new(-40., 2.), // y -> limit of diagonal zone
                    Vect::new(-15., 8.), // x -> Plain zone
                    Vect::new(15., 8.),  // x -> Plain zone
                    Vect::new(40., 2.),  // y -> limit of diagonal zone
                    Vect::new(40., -8.),
                ]
            }
            PaddleSize::XL => {
                [
                    Vect::new(-48., -8.),
                    Vect::new(-48., 2.), // y -> limit of diagonal zone
                    Vect::new(-18., 8.), // x -> Plain zone
                    Vect::new(18., 8.),  // x -> Plain zone
                    Vect::new(48., 2.),  // y -> limit of diagonal zone
                    Vect::new(48., -8.),
                ]
            }
        }
    }

    pub fn get_width(self) -> f32 {
        let shape = self.get_shape()[5];
        shape.x * 2.0
    }
}

#[derive(Component)]
pub struct Paddle;
