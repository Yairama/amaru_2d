use bevy::prelude::*;

//bricks
pub(crate) const BRICK_SIZE: Vec2 = Vec2::new(32., 16.);
pub(crate) const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
pub(crate) const GAP_BETWEEN_BRICKS: f32 = 1.0;
pub(crate) const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
pub(crate) const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum BrickColor {
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
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum BrickType {
    Default,
    TwoLife,
    ThreeLife,
    FiveLife,
    Immortal,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct Brick {
    pub(crate) health: i32,
}
