use crate::components::ball::{Ball, BALL_SPEED};
use crate::components::brick::Brick;
use crate::components::paddle::Paddle;
use crate::components::powerup::{
    PowerUp, POWERUP_DIRECTION, POWERUP_SIZE, POWERUP_SPEED, TEMP_PADDLE_COLOR,
};
use crate::resources::scoreboard::Scoreboard;
use crate::resources::sounds::CollisionSound;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn check_ball_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query_bricks: Query<(&mut Brick, &Transform, Option<&PowerUp>)>,
    mut query_ball: Query<&mut Velocity, With<Ball>>,
    collision_sound: Res<CollisionSound>,
    mut score: ResMut<Scoreboard>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _) => {}
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
                                SpriteBundle {
                                    transform: Transform {
                                        translation: brick_transform.translation,
                                        scale: POWERUP_SIZE,
                                        ..default()
                                    },
                                    sprite: Sprite {
                                        color: TEMP_PADDLE_COLOR,
                                        ..default()
                                    },
                                    ..default()
                                },
                                RigidBody::Dynamic,
                                Velocity {
                                    linvel: POWERUP_SPEED * POWERUP_DIRECTION,
                                    angvel: 0.0,
                                },
                                Collider::cuboid(0.5, 0.5),
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
    mut query_powerup: Query<&PowerUp>,
    mut query_paddle: Query<&TextureAtlasSprite, With<Paddle>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1, entity_2, _) => {
                if let Ok(powerup) = query_powerup.get_mut(*entity_2) {
                    if let Ok(paddle_sprite) = query_paddle.get_mut(*entity_1) {
                        commands.entity(*entity_2).despawn();
                    }
                }
            }
            CollisionEvent::Stopped(entity_1, entity_2, _) => {}
        }
    }
}
