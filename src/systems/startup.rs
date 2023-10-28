use crate::components::{
    ball::*,
    brick::*,
    paddle::*,
    physics_components::{Collider, Velocity},
    wall::*,
};
use crate::resources::{scoreboard::*, sounds::*};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use rand::Rng;
use bevy::utils::HashMap;
use crate::resources::textures::PaddleTextures;


pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>, texture_atlases: ResMut<Assets<TextureAtlas>>, mut paddle_textures: ResMut<PaddleTextures>) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // paddle textures
    let texture_handle = asset_server.load("textures/breakout_assets.png");
    let texture_atlas =
        TextureAtlas::new_empty(texture_handle, Vec2::new(767., 511.));
    let texture_atlas_handle = generate_sprites(texture_atlases, texture_atlas, &mut paddle_textures);//texture_atlases.add(texture_atlas);

    let result = paddle_textures.0.get(&(PaddleSize::M, PaddleColor::Red, PaddleType::Standard)).unwrap();
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
            sprite: TextureAtlasSprite::new(result.0),//15-21
            texture_atlas: texture_atlas_handle,
            ..default()
        },
        Paddle,
        Collider { size: PADDLE_SIZE },
    ));

    let ball_texture = asset_server.load("textures/circle.png");

    //ball
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(BALL_SIZE),
                ..default()
            },
            texture: ball_texture,
            ..default()
        },
        Ball { size: BALL_SIZE },
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
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

        let mut rng = rand::thread_rng();

        for row in 0..rows {
            for column in 0..columns {
                let brick_pos = vec2(
                    offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                    offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
                );
                let n: i32 = rng.gen_range(0..=10);

                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: brick_pos.extend(0.),
                            ..default()
                        },
                        sprite: Sprite {
                            color: Color::rgb(0.5, (n as f32) / 10.0, 0.1),
                            custom_size: Some(BRICK_SIZE),
                            ..default()
                        },
                        ..default()
                    },
                    Brick { health: n },
                    Collider { size: BRICK_SIZE },
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
    commands.spawn(WallBundle {
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
        collider: Collider { size: wall_size },
    });
}



// X [> -> paint: if dimension are 10 a 12, use 10 to 13
// Y [> -> paint: if dimension are 10 a 12, use 10 to 13
fn generate_sprites(mut texture_atlases: ResMut<Assets<TextureAtlas>>, mut texture_atlas: TextureAtlas, mut paddle_textures: &mut ResMut<PaddleTextures>) -> Handle<TextureAtlas>{

    let size_configs = [
        (PaddleSize::XS, [0., 32.]),
        (PaddleSize::S, [40., 88.]),
        (PaddleSize::M, [96., 160.]),
        (PaddleSize::L, [168., 248.]),
        (PaddleSize::XL, [256., 352.]),
    ];

    let color_configs = [
        (PaddleColor::Red, ([384., 400.], [400., 416.])),
        (PaddleColor::Blue, ([416., 432.], [432., 448.])),
        (PaddleColor::Yellow, ([448., 464.], [464., 480.])),
        (PaddleColor::Green, ([480., 496.], [496., 512.])),
    ];

    let type_configs = [
        (PaddleType::Shooter,0),
        (PaddleType::Standard,1)
    ];
    let mut map: HashMap<(PaddleSize, PaddleColor, PaddleType), (usize, Rect)> = HashMap::new();

    for &(size, size_x) in size_configs.iter() {
        for &(color, (vec1, vec2)) in color_configs.iter() {
            for &(typ, index) in type_configs.iter() {
                let size_y = if index == 0 { vec1 } else { vec2 };
                let rect = Rect {
                    min: Vec2::new(size_x[0], size_y[0]),
                    max: Vec2::new(size_x[1], size_y[1])
                };

                let texture_index = texture_atlas.add_texture(rect);
                paddle_textures.0.insert((size, color, typ), (texture_index, rect));
            }
        }
    }

    texture_atlases.add(texture_atlas)

}
