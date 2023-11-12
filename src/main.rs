mod components;
mod resources;
mod systems;

use crate::resources::scoreboard::Scoreboard;

use crate::systems::collision::{apply_ball_powerup, apply_paddle_powerup, check_ball_collisions, check_powerups_collisions};
use crate::systems::load_sounds::SoundsPlugin;
use crate::systems::load_textures::TexturesPlugin;
use crate::systems::movement::move_paddle_with_mouse;
use crate::systems::scoring::update_scoreboard;
use crate::systems::startup::setup;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::ball::BallType;
use crate::components::paddle::{PaddleColor, PaddleSize, PaddleType};
use crate::components::powerup::{PowerUpBallEvent, PowerUpPaddleEvent, PowerUpState};

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
        .add_event::<PowerUpBallEvent>()
        .add_event::<PowerUpPaddleEvent>()
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(PowerUpState{
            ball_type: BallType::Red,
            paddle_type: PaddleType::Standard,
            paddle_color: PaddleColor::Red,
            paddle_size: PaddleSize::M
        })
        .add_systems(Update, (bevy::window::close_on_esc, update_scoreboard))
        .add_systems(PostStartup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_paddle_with_mouse,
                check_ball_collisions,
                check_powerups_collisions,
            )
        )
        .add_systems(Update,
                     (
                         apply_ball_powerup,
                         apply_paddle_powerup
                     ))
        .run();
}
