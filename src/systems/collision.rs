use crate::components::brick::Brick;
use crate::resources::scoreboard::Scoreboard;
use crate::resources::sounds::CollisionSound;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn check_ball_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query_bricks: Query<&mut Brick>,
    collision_sound: Res<CollisionSound>,
    mut score: ResMut<Scoreboard>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(_ball, _entity, _) => {}
            CollisionEvent::Stopped(_ball, entity, _) => {
                if let Ok(mut brick) = query_bricks.get_mut(*entity) {
                    brick.health -= 1;
                    if brick.health <= 0 {
                        commands.entity(*entity).despawn();
                        score.score += 1;
                    }
                }
                commands.spawn(AudioBundle {
                    source: collision_sound.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
                println!("Received collision event: {:?}", entity);
            }
        }
    }
}
