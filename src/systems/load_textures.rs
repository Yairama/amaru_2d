use bevy::ecs::bundle::DynamicBundle;
use crate::components::ball::*;
use crate::components::brick::*;
use crate::components::paddle::*;
use crate::resources::textures::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::components::powerup::PowerUp;

pub struct TexturesPlugin;
impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_image_textures)
            .add_systems(
                Startup,
                (
                    generate_paddle_sprites,
                    generate_brick_textures,
                    generate_ball_textures,
                    generate_food_textures
                ),
            );
    }
}

pub fn load_image_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let general_texture_handler = asset_server.load("textures/breakout_assets.png");
    let food_texture_handler = asset_server.load("textures/food.png");
    let textures_handler = TexturesHandler {
        general_textures: general_texture_handler,
        food_textures: food_texture_handler,
    };

    commands.insert_resource(textures_handler);
    commands.insert_resource(PaddleTextures(HashMap::new()));
    commands.insert_resource(BrickTextures(HashMap::new()));
    commands.insert_resource(BallTextures(HashMap::new()));
    commands.insert_resource(FoodTextures(HashMap::new()));
}

pub fn generate_paddle_sprites(
    mut commands: Commands,
    textures_handler: ResMut<TexturesHandler>,
    mut paddle_textures: ResMut<PaddleTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut texture_atlas = TextureAtlas::new_empty(
        textures_handler.general_textures.clone(),
        Vec2::new(767., 511.),
    );

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
    let handler = texture_atlases.add(texture_atlas);
    commands.insert_resource(PaddleAtlasHandler(handler));
}

pub fn generate_brick_textures(
    mut commands: Commands,
    textures_handler: ResMut<TexturesHandler>,
    mut brick_textures: ResMut<BrickTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut texture_atlas = TextureAtlas::new_empty(
        textures_handler.general_textures.clone(),
        Vec2::new(767., 511.),
    );

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
        &mut brick_textures,
        &default_color_config,
        &default_size_config,
        BrickType::Default,
    );
    // Two Life Bricks
    process_bricks_textures(
        &mut texture_atlas,
        &mut brick_textures,
        &life_color_config,
        &two_life_size_config,
        BrickType::TwoLife,
    );
    // Three Life Bricks
    process_bricks_textures(
        &mut texture_atlas,
        &mut brick_textures,
        &life_color_config,
        &three_life_size_config,
        BrickType::ThreeLife,
    );
    // Five Life Bricks
    process_bricks_textures(
        &mut texture_atlas,
        &mut brick_textures,
        &five_life_color_config,
        &five_life_size_config,
        BrickType::FiveLife,
    );

    let handler = texture_atlases.add(texture_atlas);
    commands.insert_resource(BrickAtlasHandler(handler));
}

pub fn process_bricks_textures(
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

pub fn generate_ball_textures(
    mut commands: Commands,
    textures_handler: ResMut<TexturesHandler>,
    mut ball_textures: ResMut<BallTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut texture_atlas = TextureAtlas::new_empty(
        textures_handler.general_textures.clone(),
        Vec2::new(767., 511.),
    );
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

    let handler = texture_atlases.add(texture_atlas);
    commands.insert_resource(BallAtlasHandler(handler));
}

pub fn generate_food_textures(
    mut commands: Commands,
    textures_handler: ResMut<TexturesHandler>,
    mut food_textures: ResMut<FoodTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures_handler.food_textures.clone(),
        Vec2::new(16.0, 16.0),
        8,
        8,
        None,
        None
    );

    let powerup_array = PowerUp::get_array();

    for (index, powerup) in powerup_array.iter().enumerate() {
        food_textures.0.insert(*powerup, index);
    }

    let handler = texture_atlases.add(texture_atlas);
    commands.insert_resource(PowerUpHandler(handler))

}
