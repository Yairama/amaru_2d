use crate::components::powerup::PowerUp;
use crate::components::{ball::*, brick::*, paddle::*, wall::*};
use crate::resources::scoreboard::*;
use crate::resources::textures::*;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(crate) fn setup(
    mut commands: Commands,
    paddle_textures: Res<PaddleTextures>,
    paddle_atlas_handler: Res<PaddleAtlasHandler>,
    brick_textures: ResMut<BrickTextures>,
    brick_atlas_handler: Res<BrickAtlasHandler>,
    ball_textures: ResMut<BallTextures>,
    ball_atlas_handler: Res<BallAtlasHandler>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());
    let paddle_type = paddle_textures
        .0
        .get(&(PaddleSize::M, PaddleColor::Red, PaddleType::Standard))
        .unwrap();

    // paddle
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: vec3(0., PADDLE_START_Y, 0.0),
                ..default()
            },
            sprite: TextureAtlasSprite::new(paddle_type.0), //15-21
            texture_atlas: paddle_atlas_handler.0.clone(),
            ..default()
        },
        Restitution::coefficient(1.0),
        Paddle,
        Friction::coefficient(0.0),
        Collider::polyline(PADDLE_SHAPE.to_vec(), Some(PADDLE_INDICES.to_vec())),
        Ccd::enabled(),
    ));

    let ball_type = ball_textures.0.get(&BallType::Purple).unwrap();

    //ball
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: TextureAtlasSprite::new(ball_type.0),
            texture_atlas: ball_atlas_handler.0.clone(),
            ..default()
        },
        RigidBody::Dynamic,
        Ball { size: BALL_SIZE },
        Restitution::coefficient(1.0),
        Collider::ball(BALL_SIZE.x / 2.0),
        Velocity {
            linvel: BALL_SPEED * BALL_INITIAL_DIRECTION,
            angvel: 0.0,
        },
        Friction::coefficient(0.0),
        GravityScale(0.0),
        Ccd::enabled(),
        ActiveEvents::COLLISION_EVENTS,
    ));

    // walls
    {
        let vertical_wall_size = vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
        let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

        // Left and right walls
        spawn_wall(&mut commands, vec3(LEFT_WALL, 0., 0.), vertical_wall_size);
        spawn_wall(&mut commands, vec3(RIGHT_WALL, 0., 0.), vertical_wall_size);

        // Bottom and top walls
        spawn_wall(
            &mut commands,
            vec3(0., BOTTOM_WALL, 0.),
            horizontal_wall_size,
        );
        spawn_wall(&mut commands, vec3(0., TOP_WALL, 0.), horizontal_wall_size);
    }

    // bricks
    {
        let offset_x = LEFT_WALL + GAP_BETWEEN_BRICKS_AND_SIDES + BRICK_SIZE.x * 0.5;
        let offset_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_BRICKS + BRICK_SIZE.y * 0.5;
        let bricks_total_width = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
        let bricks_total_height = (TOP_WALL - BOTTOM_WALL)
            - GAP_BETWEEN_BRICKS_AND_CEILING
            - GAP_BETWEEN_PADDLE_AND_BRICKS;
        let rows = (bricks_total_height / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as i32;
        let columns = (bricks_total_width / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as i32;

        let brick_type = brick_textures
            .0
            .get(&(BrickColor::Pink, BrickType::ThreeLife, TextureFrame(0)))
            .unwrap();

        for row in 0..rows {
            for column in 0..columns {
                let brick_pos = vec2(
                    offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                    offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
                );
                let powerup: PowerUp = rand::random();
                commands.spawn((
                    SpriteSheetBundle {
                        transform: Transform {
                            translation: brick_pos.extend(0.),
                            ..default()
                        },
                        sprite: TextureAtlasSprite::new(brick_type.0),
                        texture_atlas: brick_atlas_handler.0.clone(),
                        ..default()
                    },
                    Brick { health: 1 },
                    powerup,
                    Friction::coefficient(0.0),
                    Restitution::coefficient(1.0),
                    Collider::cuboid(BRICK_SIZE.x / 2., BRICK_SIZE.y / 2.),
                    RigidBody::Fixed,
                ));
            }
        }
    }

    //Scoreboard
    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "Score: ",
            TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            },
        ),
        TextSection::from_style(TextStyle {
            font_size: SCOREBOARD_FONT_SIZE,
            color: SCORE_COLOR,
            ..default()
        }),
    ])
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: SCOREBOARD_TEXT_PADDING,
        left: SCOREBOARD_TEXT_PADDING,
        ..default()
    }),));
}

fn spawn_wall(commands: &mut Commands, translation: Vec3, wall_size: Vec2) {
    commands.spawn((
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(wall_size),
                    ..Default::default()
                },
                ..Default::default()
            },
        },
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Collider::cuboid(wall_size.x / 2., wall_size.y / 2.),
        RigidBody::Fixed,
    ));
}
