use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::components::paddle::*;

#[derive(Resource)]
pub(crate) struct PaddleTextures (pub(crate) HashMap<(PaddleSize, PaddleColor, PaddleType), (usize, Rect)>);

