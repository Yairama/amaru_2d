use bevy::prelude::{AudioSource, Deref, DerefMut, Handle, Resource};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct CollisionSound(pub Handle<AudioSource>);
