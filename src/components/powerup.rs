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


#[derive(Event)]
pub struct PowerUpBallEvent;

#[derive(Event)]
pub struct PowerUpPaddleEvent;

#[derive(Resource)]
pub struct PowerUpState {
    pub paddle_type: PaddleType,
    pub paddle_size: PaddleSize,
    pub paddle_color: PaddleColor,
    pub ball_type: BallType,
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

impl PowerUp{
    pub fn get_array() -> [PowerUp;26]{
        [
            PowerUp::BallRed,
            PowerUp::BallSkyBlue,
            PowerUp::BallGreen,
            PowerUp::BallOrange,
            PowerUp::BallYellow,
            PowerUp::BallPurple,
            PowerUp::BallWhite,
            PowerUp::BallBrown,
            PowerUp::BallPink,
            PowerUp::BallBlue,
            PowerUp::BallGhost,
            PowerUp::BallExplosive,
            PowerUp::BallGiant,
            PowerUp::PaddleXS,
            PowerUp::PaddleS,
            PowerUp::PaddleM,
            PowerUp::PaddleL,
            PowerUp::PaddleXL,
            PowerUp::PaddleShooter,
            PowerUp::PaddleStandard,
            PowerUp::PaddleRed,
            PowerUp::PaddleBlue,
            PowerUp::PaddleYellow,
            PowerUp::PaddleGreen,
            PowerUp::SpawnFiveBalls,
            PowerUp::DuplicateBalls
        ]
    }
}

impl Distribution<PowerUp> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUp {
        PowerUp::get_array()[rng.gen_range(0..=25)]
    }
}
