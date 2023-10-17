use bevy::prelude::{*};
use bevy::sprite::collide_aabb::*;
use crate::components::{physics_components::{Velocity, Collider}, ball::Ball, brick::Brick};
use crate::resources::{scoreboard::Scoreboard, sounds::CollisionSound};

pub(crate) fn check_ball_collisions(
    mut commands: Commands,
    mut score: ResMut<Scoreboard>,
    collision_sound: Res<CollisionSound>,
    mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
    collider_query: Query<(Entity, &Transform, &Collider, Option<&Brick>)>,
) {
    for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
        for (other_entity, transform, other, opt_brick) in &collider_query {
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

                if opt_brick.is_some() {
                    score.score += 1;
                    commands.entity(other_entity).despawn();
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