use crate::components::ball::*;
use crate::components::paddle::*;
use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub(crate) const POWERUP_SIZE: Vec3 = Vec3::new(16., 16., 0.);
pub(crate) const TEMP_PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub(crate) const POWERUP_SPEED: f32 = 50.0;
pub(crate) const POWERUP_DIRECTION: Vec2 = Vec2::new(0.0, -1.0);

#[derive(Resource)]
struct PowerUpState {
    paddle_type: PaddleType,
    paddle_size: PaddleSize,
    paddle_color: PaddleColor,
    ball_type: BallType,
}

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PowerUp {
    BallRed,
    BallSkyBlue,
    BallGreen,
    BallOrange,
    BallYellow,
    BallPurple,
    BallWhite,
    BallBrown,
    BallPink,
    BallBlue,
    BallGhost,
    BallExplosive,
    BallGiant,
    PaddleXS,
    PaddleS,
    PaddleM,
    PaddleL,
    PaddleXL,
    PaddleShooter,
    PaddleStandard,
    PaddleRed,
    PaddleBlue,
    PaddleYellow,
    PaddleGreen,
    SpawnFiveBalls,
    DuplicateBalls,
}

impl Distribution<PowerUp> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUp {
        match rng.gen_range(0..=25) {
            0 => PowerUp::BallRed,
            1 => PowerUp::BallSkyBlue,
            2 => PowerUp::BallGreen,
            3 => PowerUp::BallOrange,
            4 => PowerUp::BallYellow,
            5 => PowerUp::BallPurple,
            6 => PowerUp::BallWhite,
            7 => PowerUp::BallBrown,
            8 => PowerUp::BallPink,
            9 => PowerUp::BallBlue,
            10 => PowerUp::BallGhost,
            11 => PowerUp::BallExplosive,
            12 => PowerUp::BallGiant,
            13 => PowerUp::PaddleXS,
            14 => PowerUp::PaddleS,
            15 => PowerUp::PaddleM,
            16 => PowerUp::PaddleL,
            17 => PowerUp::PaddleXL,
            18 => PowerUp::PaddleShooter,
            19 => PowerUp::PaddleStandard,
            20 => PowerUp::PaddleRed,
            21 => PowerUp::PaddleBlue,
            22 => PowerUp::PaddleYellow,
            23 => PowerUp::PaddleGreen,
            24 => PowerUp::SpawnFiveBalls,
            25 => PowerUp::DuplicateBalls,
            _ => PowerUp::DuplicateBalls,
        }
    }
}
