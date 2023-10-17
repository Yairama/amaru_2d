
use bevy::prelude::{Bundle, Color, Component, SpriteBundle, Vec2, Vec3};
use super::physics_components::Collider;
// wall
pub(crate) const LEFT_WALL: f32 = -450.0;
pub(crate) const RIGHT_WALL: f32 = 450.0;
pub(crate) const BOTTOM_WALL: f32 = -300.0;
pub(crate) const TOP_WALL: f32 = 300.0;
pub(crate) const WALL_THICKNESS: f32 = 10.0;
pub(crate) const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
pub(crate) const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
pub(crate) const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);


#[derive(Bundle)]
pub(crate) struct WallBundle {
    pub(crate) sprite_bundle: SpriteBundle,
    pub(crate) collider: Collider,
}