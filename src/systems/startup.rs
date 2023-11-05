use crate::components::powerup::PowerUp;
use crate::components::{ball::*, brick::*, paddle::*, wall::*};
use crate::resources::textures::{BallTextures, BrickTextures, PaddleTextures, TextureFrame};
use crate::resources::{scoreboard::*, sounds::*};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(crate) fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut paddle_textures: ResMut<PaddleTextures>,
    mut brick_textures: ResMut<BrickTextures>,
    mut ball_textures: ResMut<BallTextures>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // paddle textures
    let texture_handle = asset_server.load("textures/breakout_assets.png");
    let texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(767., 511.));
    let texture_atlas = generate_paddle_sprites(texture_atlas, &mut paddle_textures); //texture_atlases.add(texture_atlas);
    let texture_atlas = generate_brick_textures(texture_atlas, &mut brick_textures);
    let texture_atlas = generate_ball_textures(texture_atlas, &mut ball_textures);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let result = paddle_textures
        .0
        .get(&(PaddleSize::M, PaddleColor::Red, PaddleType::Standard))
        .unwrap();
    //sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // paddle
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: vec3(0., PADDLE_START_Y, 0.0),
                ..default()
            },
            sprite: TextureAtlasSprite::new(result.0), //15-21
            texture_atlas: texture_atlas_handle.clone(),
            ..default()
        },
        Restitution::coefficient(1.0),
        Paddle,
        Friction::coefficient(0.0),
        Collider::polyline(PADDLE_SHAPE.to_vec(), Some(PADDLE_INDICES.to_vec())),
        Ccd::enabled(),
    ));

    //ball
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: TextureAtlasSprite::new(210),
            texture_atlas: texture_atlas_handle.clone(),
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

        for row in 0..rows {
            for column in 0..columns {
                let brick_pos = vec2(
                    offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                    offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
                );

                commands.spawn((
                    SpriteSheetBundle {
                        transform: Transform {
                            translation: brick_pos.extend(0.),
                            ..default()
                        },
                        sprite: TextureAtlasSprite::new(60),
                        texture_atlas: texture_atlas_handle.clone(),
                        ..default()
                    },
                    Brick { health: 1 },
                    PowerUp::BallFire,
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

// X [> -> paint: if dimension are 10 a 12, use 10 to 13
// Y [> -> paint: if dimension are 10 a 12, use 10 to 13
fn generate_paddle_sprites(
    mut texture_atlas: TextureAtlas,
    paddle_textures: &mut ResMut<PaddleTextures>,
) -> TextureAtlas {
    // paddle
    let size_configs = [
        (PaddleSize::XS, [0., 32.]),
        (PaddleSize::S, [40., 88.]),
        (PaddleSize::M, [96., 160.]),
        (PaddleSize::L, [168., 248.]),
        (PaddleSize::XL, [256., 352.]),
    ];

    let color_configs = [
        (PaddleColor::Red, [[384., 400.], [400., 416.]]),
        (PaddleColor::Blue, [[416., 432.], [432., 448.]]),
        (PaddleColor::Yellow, [[448., 464.], [464., 480.]]),
        (PaddleColor::Green, [[480., 496.], [496., 512.]]),
    ];

    let type_configs = [(PaddleType::Shooter, 0), (PaddleType::Standard, 1)];

    for &(size, size_x) in size_configs.iter() {
        for &(color, [vec1, vec2]) in color_configs.iter() {
            for &(typ, index) in type_configs.iter() {
                let size_y = if index == 0 { vec1 } else { vec2 };
                let rect = Rect {
                    min: Vec2::new(size_x[0], size_y[0]),
                    max: Vec2::new(size_x[1], size_y[1]),
                };

                let texture_index = texture_atlas.add_texture(rect);
                paddle_textures
                    .0
                    .insert((size, color, typ), (texture_index, rect));
            }
        }
    }

    texture_atlas
}

fn process_bricks_textures(
    texture_atlas: &mut TextureAtlas,
    brick_textures: &mut ResMut<BrickTextures>,
    color_config: &[(BrickColor, [f32; 2])],
    size_config: &[[f32; 2]],
    brick_type: BrickType,
) {
    for &(color, vec_y) in color_config.iter() {
        for (index, &vec_x) in size_config.iter().enumerate() {
            let rect = Rect {
                min: Vec2::new(vec_x[0], vec_y[0]),
                max: Vec2::new(vec_x[1], vec_y[1]),
            };
            let texture_index = texture_atlas.add_texture(rect);
            brick_textures.0.insert(
                (color, brick_type, TextureFrame(index)),
                (texture_index, rect),
            );
        }
    }
}

fn generate_brick_textures(
    mut texture_atlas: TextureAtlas,
    brick_textures: &mut ResMut<BrickTextures>,
) -> TextureAtlas {
    // Bricks

    let default_size_config = [
        [0., 32.],
        [32., 64.],
        [64., 96.],
        [96., 128.],
        [128., 160.],
        [160., 192.],
    ];
    let default_color_config = [
        (BrickColor::Red, [0., 16.]),
        (BrickColor::SkyBlue, [16., 32.]),
        (BrickColor::Green, [32., 48.]),
        (BrickColor::Orange, [48., 64.]),
        (BrickColor::Yellow, [64., 80.]),
        (BrickColor::Purple, [80., 96.]),
        (BrickColor::White, [96., 112.]),
        (BrickColor::Brown, [112., 128.]),
        (BrickColor::Pink, [128., 144.]),
        (BrickColor::Blue, [144., 160.]),
    ];

    let two_life_size_config = [[0., 32.], [64., 96.]];
    let three_life_size_config = [[128., 160.], [192., 224.], [256., 288.]];
    let life_color_config = [
        (BrickColor::Red, [192., 208.]),
        (BrickColor::SkyBlue, [208., 224.]),
        (BrickColor::Green, [224., 240.]),
        (BrickColor::Orange, [240., 256.]),
        (BrickColor::Yellow, [256., 272.]),
        (BrickColor::Purple, [272., 288.]),
        (BrickColor::White, [288., 304.]),
        (BrickColor::Brown, [304., 320.]),
        (BrickColor::Pink, [320., 336.]),
        (BrickColor::Blue, [336., 352.]),
    ];

    let five_life_size_config = [
        [448., 512.],
        [512., 576.],
        [576., 640.],
        [640., 704.],
        [704., 768.],
    ];
    let five_life_color_config = [
        (BrickColor::Red, [0., 32.]),
        (BrickColor::SkyBlue, [32., 64.]),
        (BrickColor::Green, [64., 96.]),
        (BrickColor::Orange, [96., 128.]),
        (BrickColor::Yellow, [128., 160.]),
        (BrickColor::Purple, [160., 192.]),
        (BrickColor::White, [192., 224.]),
        (BrickColor::Brown, [224., 256.]),
        (BrickColor::Pink, [256., 288.]),
        (BrickColor::Blue, [288., 320.]),
    ];

    let immortal_size_confing = [[320., 352.], [352., 384.]];
    let immortal_color_config = [
        ([192., 224.], [BrickColor::Red, BrickColor::Purple]),
        ([224., 256.], [BrickColor::SkyBlue, BrickColor::White]),
        ([256., 288.], [BrickColor::Green, BrickColor::Brown]),
        ([288., 320.], [BrickColor::Orange, BrickColor::Pink]),
        ([320., 352.], [BrickColor::Yellow, BrickColor::Blue]),
    ];
    for (index, &vec_x) in immortal_size_confing.iter().enumerate() {
        for (vec_y, color_array) in immortal_color_config {
            let rect = Rect {
                min: Vec2::new(vec_x[0], vec_y[0]),
                max: Vec2::new(vec_x[1], vec_y[1]),
            };
            let texture_index = texture_atlas.add_texture(rect);
            brick_textures.0.insert(
                (color_array[index], BrickType::Immortal, TextureFrame(0)),
                (texture_index, rect),
            );
        }
    }

    // Default Bricks
    process_bricks_textures(
        &mut texture_atlas,
        brick_textures,
        &default_color_config,
        &default_size_config,
        BrickType::Default,
    );
    // Two Life Bricks
    process_bricks_textures(
        &mut texture_atlas,
        brick_textures,
        &life_color_config,
        &two_life_size_config,
        BrickType::TwoLife,
    );
    // Three Life Bricks
    process_bricks_textures(
        &mut texture_atlas,
        brick_textures,
        &life_color_config,
        &three_life_size_config,
        BrickType::ThreeLife,
    );
    // Five Life Bricks
    process_bricks_textures(
        &mut texture_atlas,
        brick_textures,
        &five_life_color_config,
        &five_life_size_config,
        BrickType::FiveLife,
    );

    texture_atlas
}

fn generate_ball_textures(
    mut texture_atlas: TextureAtlas,
    ball_textures: &mut ResMut<BallTextures>,
) -> TextureAtlas {
    let size_config = [[384., 400.], [400., 416.]];
    let color_config = [
        ([384., 400.], [BallType::Red, BallType::SkyBlue]),
        ([400., 416.], [BallType::Green, BallType::Orange]),
        ([416., 432.], [BallType::Yellow, BallType::Purple]),
        ([432., 448.], [BallType::White, BallType::Brown]),
        ([448., 464.], [BallType::Pink, BallType::Blue]),
        ([464., 480.], [BallType::Ghost, BallType::Explosive]),
    ];

    for (index, &vec_x) in size_config.iter().enumerate() {
        for (vec_y, color_array) in color_config {
            let rect = Rect {
                min: Vec2::new(vec_x[0], vec_y[0]),
                max: Vec2::new(vec_x[1], vec_y[1]),
            };
            let texture_index = texture_atlas.add_texture(rect);
            ball_textures
                .0
                .insert(color_array[index], (texture_index, rect));
        }
    }

    // adding giant ball
    let giant_rect = Rect {
        min: Vec2::new(384., 480.),
        max: Vec2::new(416., 512.),
    };
    let giant_index = texture_atlas.add_texture(giant_rect);
    ball_textures
        .0
        .insert(BallType::Giant, (giant_index, giant_rect));

    texture_atlas
}
