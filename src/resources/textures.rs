use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::components::brick::{BrickColor, BrickType};
use crate::components::paddle::*;

#[derive(Resource)]
pub(crate) struct PaddleTextures (pub(crate) HashMap<(PaddleSize, PaddleColor, PaddleType), (usize, Rect)>);

#[derive(Resource)]
pub(crate) struct BrickTextures (pub(crate) HashMap<(BrickColor, BrickType, TextureFrame), (usize, Rect)>);
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) struct TextureFrame(pub(crate) usize);