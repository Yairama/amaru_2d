use crate::components::ball::BallType;
use crate::components::brick::{BrickColor, BrickType};
use crate::components::paddle::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource)]
pub(crate) struct PaddleTextures(
    pub(crate) HashMap<(PaddleSize, PaddleColor, PaddleType), (usize, Rect)>,
);

#[derive(Resource)]
pub(crate) struct BrickTextures(
    pub(crate) HashMap<(BrickColor, BrickType, TextureFrame), (usize, Rect)>,
);

#[derive(Resource)]
pub(crate) struct BallTextures(pub(crate) HashMap<BallType, (usize, Rect)>);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct TextureFrame(pub(crate) usize);

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PaddleTextures(HashMap::new()))
            .insert_resource(BrickTextures(HashMap::new()))
            .insert_resource(BallTextures(HashMap::new()));
    }
}