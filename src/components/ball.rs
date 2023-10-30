use bevy::prelude::*;

// ball
pub(crate) const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
pub(crate) const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
pub(crate) const BALL_SPEED: f32 = 400.0;
pub(crate) const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum BallType {
    Red,
    SkyBlue,
    Green,
    Orange,
    Yellow,
    Purple,
    White,
    Brown,
    Pink,
    Blue,
    Ghost,
    Explosive,
    Giant,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct Ball {
    pub(crate) size: Vec2,
}
