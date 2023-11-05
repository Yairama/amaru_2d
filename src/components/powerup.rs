use crate::components::ball::*;
use crate::components::paddle::*;
use bevy::prelude::*;

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
    BallFire,
    BallIce,
    BallNature,
}
