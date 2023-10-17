use bevy::prelude::{AudioSource, Deref, DerefMut, Handle, Resource};

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct CollisionSound(pub(crate) Handle<AudioSource>);