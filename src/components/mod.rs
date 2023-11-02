use crate::components::ball::Ball;
use crate::components::brick::Brick;
use bevy::app::{App, Plugin};

pub mod ball;
pub mod brick;
pub mod paddle;
pub mod wall;

pub struct RegisterPlugin;

impl Plugin for RegisterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ball>().register_type::<Brick>();
    }
}
