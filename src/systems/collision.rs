use crate::components::ball::{Ball, BALL_SIZE, BALL_SPEED, BallType};
use crate::components::brick::Brick;
use crate::components::paddle::{Paddle, PADDLE_INDICES, PaddleColor, PaddleSize, PaddleType};
use crate::components::powerup::{PowerUp, POWERUP_DIRECTION, POWERUP_SIZE, POWERUP_SPEED, PowerUpBallEvent, PowerUpPaddleEvent, PowerUpState, TEMP_PADDLE_COLOR};
use crate::resources::scoreboard::Scoreboard;
use crate::resources::sounds::CollisionSound;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::resources::textures::{BallTextures, FoodTextures, PaddleTextures, PowerUpHandler};

pub fn check_ball_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query_bricks: Query<(&mut Brick, &Transform, Option<&PowerUp>)>,
    mut query_ball: Query<&mut Velocity, With<Ball>>,
    food_textures: Res<FoodTextures>,
    power_up_handler: Res<PowerUpHandler>,
    collision_sound: Res<CollisionSound>,
    mut score: ResMut<Scoreboard>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(_entity_1, _entity_2, _) => {}
            CollisionEvent::Stopped(entity_1, entity_2, _) => {
                if let Ok(mut velocity) = query_ball.get_mut(*entity_1) {
                    if velocity.linvel.length() != BALL_SPEED {
                        velocity.linvel = velocity.linvel.normalize() * BALL_SPEED;
                    }
                }

                if let Ok((mut brick, brick_transform, powerup)) = query_bricks.get_mut(*entity_2) {
                    brick.health -= 1;
                    if brick.health <= 0 {
                        commands.entity(*entity_2).despawn();
                        if let Some(powerup) = powerup {
                            commands.spawn((
                                SpriteSheetBundle {
                                    transform: Transform {
                                        translation: brick_transform.translation,
                                        ..default()
                                    },
                                    texture_atlas: power_up_handler.0.clone(),
                                    sprite: TextureAtlasSprite::new(*food_textures.0.get(powerup).unwrap()),
                                    ..default()
                                },
                                RigidBody::Dynamic,
                                Velocity {
                                    linvel: POWERUP_SPEED * POWERUP_DIRECTION,
                                    angvel: 0.0,
                                },
                                Collider::cuboid(8.0, 8.0),
                                Sensor,
                                ActiveEvents::COLLISION_EVENTS,
                                *powerup,
                            ));
                        }

                        score.score += 1;
                    }
                }

                // commands.spawn(AudioBundle {
                //     source: collision_sound.clone(),
                //     settings: PlaybackSettings::DESPAWN,
                // });
                println!("Received collision event: {:?}", entity_2);
            }
        }
    }
}

pub fn check_powerups_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut power_up_state: ResMut<PowerUpState>,
    mut power_up_ball_event: EventWriter<PowerUpBallEvent>,
    mut power_up_paddle_event: EventWriter<PowerUpPaddleEvent>,
    mut query_powerup: Query<&PowerUp>,
    mut query_paddle: Query<&TextureAtlasSprite, With<Paddle>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _) => {
                if let Ok(powerup) = query_powerup.get_mut(*entity_2) {
                    if let Ok(_paddle_sprite) = query_paddle.get_mut(*entity_1) {
                        fill_powerup_states(
                            powerup,
                            &mut *power_up_state,
                            &mut power_up_ball_event,
                            &mut power_up_paddle_event,
                        );
                        commands.entity(*entity_2).despawn();
                    }
                }
            }
            CollisionEvent::Stopped(_entity_1, _entity_2, _) => {}
        }
    }
}


fn fill_powerup_states(
    powerup: &PowerUp,
    power_up_state: &mut PowerUpState,
    power_up_ball_event: &mut EventWriter<PowerUpBallEvent>,
    power_up_paddle_event: &mut EventWriter<PowerUpPaddleEvent>,
) {
    use PowerUp::*;
    match powerup {
        BallRed | BallSkyBlue | BallGreen | BallOrange | BallYellow | BallPurple
        | BallWhite | BallBrown | BallPink | BallBlue | BallGhost | BallExplosive
        | BallGiant => {
            let ball_type = match powerup {
                BallRed => BallType::Red,
                BallSkyBlue => BallType::SkyBlue,
                BallGreen => BallType::Green,
                BallOrange => BallType::Orange,
                BallYellow => BallType::Yellow,
                BallPurple => BallType::Purple,
                BallWhite => BallType::White,
                BallBrown => BallType::Brown,
                BallPink => BallType::Pink,
                BallBlue => BallType::Blue,
                BallGhost => BallType::Ghost,
                BallExplosive => BallType::Explosive,
                BallGiant => BallType::Giant,
                _ => unreachable!(),
            };
            power_up_state.ball_type = ball_type;
            power_up_ball_event.send(PowerUpBallEvent);
        }
        PaddleXS | PaddleS | PaddleM | PaddleL | PaddleXL => {
            let paddle_size = match powerup {
                PaddleXS => PaddleSize::XS,
                PaddleS => PaddleSize::S,
                PaddleM => PaddleSize::M,
                PaddleL => PaddleSize::L,
                PaddleXL => PaddleSize::XL,
                _ => unreachable!(),
            };
            power_up_state.paddle_size = paddle_size;
            power_up_paddle_event.send(PowerUpPaddleEvent);
        }

        PaddleShooter | PaddleStandard => {
            let paddle_type = match powerup {
                PaddleShooter => PaddleType::Shooter,
                PaddleStandard => PaddleType::Standard,
                _ => unreachable!(),
            };
            power_up_state.paddle_type = paddle_type;
            power_up_paddle_event.send(PowerUpPaddleEvent);
        }

        PaddleRed | PaddleBlue | PaddleYellow | PaddleGreen => {
            let paddle_color = match powerup {
                PaddleRed => PaddleColor::Red,
                PaddleBlue => PaddleColor::Blue,
                PaddleYellow => PaddleColor::Yellow,
                PaddleGreen => PaddleColor::Green,
                _ => unreachable!(),
            };
            power_up_state.paddle_color = paddle_color;
            power_up_paddle_event.send(PowerUpPaddleEvent);
        }

        SpawnFiveBalls => { /* logic for spawning five balls */ }
        DuplicateBalls => { /* logic for duplicating balls */ }
    }
}

pub fn apply_ball_powerup(
    mut commands: Commands,
    mut power_up_state: ResMut<PowerUpState>,
    mut power_up_ball_event: EventReader<PowerUpBallEvent>,
    ball_textures: Res<BallTextures>,
    mut query_balls: Query<(&mut TextureAtlasSprite, &mut Collider), With<Ball>>,
)
{
    for _powerup in power_up_ball_event.read(){
        for (mut ball_sprite, mut collider) in query_balls.iter_mut() {
            if power_up_state.ball_type == BallType::Giant {
                *collider = Collider::ball(BALL_SIZE.x );
            } else {
                *collider = Collider::ball(BALL_SIZE.x/2.0 );
            }
            ball_sprite.index = ball_textures.0.get(&power_up_state.ball_type).unwrap().0;
        }
    }

}

pub fn apply_paddle_powerup(
    mut commands: Commands,
    mut power_up_state: ResMut<PowerUpState>,
    mut power_up_paddle_event: EventReader<PowerUpPaddleEvent>,
    paddle_textures: Res<PaddleTextures>,
    mut query_paddle: Query<(&mut TextureAtlasSprite, &mut Collider), With<Paddle>>,
){
    for _powerup in power_up_paddle_event.read(){
        for (mut paddle_sprite, mut collider) in query_paddle.iter_mut() {
            *collider = Collider::polyline(power_up_state.paddle_size.get_shape().to_vec(), Some(PADDLE_INDICES.to_vec()));
            paddle_sprite.index = paddle_textures.0.get(&(power_up_state.paddle_size, power_up_state.paddle_color, power_up_state.paddle_type)).unwrap().0;

        }
    }
}
