use bevy::prelude::{Component, Deref, DerefMut, Vec2};

#[derive(Component)]
pub(crate) struct Collider {
    pub(crate) size: Vec2,
}

#[derive(Component, Deref, DerefMut)]
pub(crate) struct Velocity(pub(crate) Vec2);

