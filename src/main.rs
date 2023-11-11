mod components;
mod resources;
mod systems;

use crate::resources::scoreboard::Scoreboard;

use crate::systems::collision::{check_ball_collisions, check_powerups_collisions};
use crate::systems::load_sounds::SoundsPlugin;
use crate::systems::load_textures::TexturesPlugin;
use crate::systems::movement::move_paddle_with_mouse;
use crate::systems::scoring::update_scoreboard;
use crate::systems::startup::setup;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((components::RegisterPlugin, TexturesPlugin, SoundsPlugin))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Update, (bevy::window::close_on_esc, update_scoreboard))
        .add_systems(PostStartup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_paddle_with_mouse,
                check_ball_collisions,
                check_powerups_collisions,
            ),
        )
        .run();
}
