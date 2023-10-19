use crate::components::{
    ball::Ball,
    brick::Brick,
    physics_components::{Collider, Velocity},
};
use crate::resources::{scoreboard::Scoreboard, sounds::CollisionSound};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;

pub(crate) fn check_ball_collisions(
    mut commands: Commands,
    mut score: ResMut<Scoreboard>,
    collision_sound: Res<CollisionSound>,
    mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
    mut collider_query: Query<(Entity, &Transform, &Collider, Option<&mut Brick>, Option<&mut Sprite>)>,
) {
    for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
        for (other_entity, transform, other, mut opt_brick, mut opt_sprite) in &mut collider_query {
            let collision = collide(
                ball_transform.translation,
                ball.size,
                transform.translation,
                other.size,
            );

            let mut reflect_x = false;
            let mut reflect_y = false;
            if let Some(collision) = collision {
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => {}
                }

                if reflect_x {
                    ball_velocity.x *= -1.0;
                }
                if reflect_y {
                    ball_velocity.y *= -1.0;
                }

                if let Some(brick) = opt_brick.as_mut() {
                    brick.health -= 1;
                    if brick.health <= 0 {
                        score.score += 1;
                        commands.entity(other_entity).despawn();
                    } else if let Some(sprite) = opt_sprite.as_mut() {
                        sprite.color = Color::rgb(0.5, (brick.health as f32)/10.0, 0.1);
                    }
                }

                //play sound
                commands.spawn(AudioBundle {
                    source: collision_sound.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
        }
    }
}
