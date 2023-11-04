use bevy::prelude::*;
use crate::components::ball::*;
use crate::components::paddle::*;

#[derive(Resource)]
struct  PowerUpState{
    paddle_type: PaddleType,
    paddle_size: PaddleSize,
    paddle_color: PaddleColor,
    ball_type: BallType
}