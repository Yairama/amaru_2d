use crate::resources::sounds::CollisionSound;
use bevy::prelude::*;

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_sounds);
    }
}

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    //sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));
}
