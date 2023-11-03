mod components;
mod resources;
mod systems;

use crate::resources::{scoreboard::Scoreboard, textures::TexturesPlugin};

use crate::systems::collision::check_ball_collisions;
use crate::systems::movement::move_paddle;
use crate::systems::scoring::update_scoreboard;
use crate::systems::startup::setup;
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            EditorPlugin::default(),
            components::RegisterPlugin,
            TexturesPlugin,
        ))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Update, (bevy::window::close_on_esc, update_scoreboard))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_paddle, check_ball_collisions))
        .run();
}
