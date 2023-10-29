mod components;
mod resources;
mod systems;

use crate::resources::{scoreboard::Scoreboard, textures::{PaddleTextures, BrickTextures}};
use crate::systems::collision::check_ball_collisions;
use crate::systems::movement::{apply_velocity, move_paddle};
use crate::systems::scoring::update_scoreboard;
use crate::systems::startup::setup;
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy::utils::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((EditorPlugin::default(), components::RegisterPlugin))
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(PaddleTextures(HashMap::new()))
        .insert_resource(BrickTextures(HashMap::new()))
        .add_systems(Update, (bevy::window::close_on_esc, update_scoreboard))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                apply_velocity,
                check_ball_collisions.after(apply_velocity),
            ),
        )
        .run();
}
