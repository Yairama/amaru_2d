use crate::components::ball::BallType;
use crate::components::brick::{BrickColor, BrickType};
use crate::components::paddle::*;
use crate::components::powerup::PowerUp;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct PaddleTextures(pub HashMap<(PaddleSize, PaddleColor, PaddleType), (usize, Rect)>);

#[derive(Resource)]
pub struct BrickTextures(pub HashMap<(BrickColor, BrickType, TextureFrame), (usize, Rect)>);

#[derive(Resource)]
pub struct BallTextures(pub HashMap<BallType, (usize, Rect)>);

#[derive(Resource)]
pub struct FoodTextures(pub HashMap<PowerUp, usize>);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct TextureFrame(pub usize);

#[derive(Resource)]
pub struct PaddleAtlasHandler(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct BallAtlasHandler(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct BrickAtlasHandler(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct PowerUpHandler(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct TexturesHandler {
    pub general_textures: Handle<Image>,
    pub food_textures: Handle<Image>,
}
