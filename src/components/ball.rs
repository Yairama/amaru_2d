use bevy::prelude::{Color, Component, Vec2, Vec3};
// ball
pub(crate) const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub(crate) const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
pub(crate) const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
pub(crate) const BALL_SPEED: f32 = 400.0;
pub(crate) const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

#[derive(Component)]
pub(crate) struct Ball {
    pub(crate) size: Vec2,
}

