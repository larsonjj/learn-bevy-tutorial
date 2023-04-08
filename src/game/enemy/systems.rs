use super::components::*;
use super::events::*;
use super::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES};
use crate::game::walls::components::Walls;
use crate::loading::resources::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

pub fn spawn_enemies(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * (window.width() - ENEMY_SIZE);
        let random_y = random::<f32>() * (window.height() - ENEMY_SIZE);
        let velocity = Vec2::new(random::<f32>(), random::<f32>()).normalize() * ENEMY_SPEED;
        commands
            .spawn(SpriteBundle {
                texture: textures.enemy_ball.clone(),
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(ENEMY_SIZE / 2.0))
            .insert(Velocity::linear(velocity))
            .insert(Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(GravityScale(0.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Enemy);
    }
}

pub fn move_enemy_controller(mut enemy_query: Query<&mut Velocity, With<Enemy>>) {
    for mut enemy_velocity in &mut enemy_query {
        // Keep constant speed at all times for enemies
        enemy_velocity.linvel = enemy_velocity.linvel.normalize() * ENEMY_SPEED;
    }
}

pub fn check_for_world_collisions(
    mut enemy_collider_query: Query<(Entity, &mut Enemy), (With<Collider>, With<Enemy>)>,
    mut wall_collider_query: Query<(Entity, &Walls), (With<Collider>, With<Walls>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut enemy_hit_wall_event: EventWriter<EnemyHitWallEvent>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                let enemy = if let Ok(a) = enemy_collider_query.get_mut(*a) {
                    Some(a)
                } else if let Ok(b) = enemy_collider_query.get_mut(*b) {
                    Some(b)
                } else {
                    None
                };

                let wall = if let Ok(a) = wall_collider_query.get_mut(*a) {
                    Some(a)
                } else if let Ok(b) = wall_collider_query.get_mut(*b) {
                    Some(b)
                } else {
                    None
                };

                if enemy.is_some() && wall.is_some() {
                    // Enemy collided with wall
                    enemy_hit_wall_event.send_default();
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

pub fn play_enemy_wall_hit_sound(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut enemy_hit_wall_events: EventReader<EnemyHitWallEvent>,
) {
    if !enemy_hit_wall_events.is_empty() {
        // This prevents events staying active on the next frame.
        enemy_hit_wall_events.clear();

        // Randomely play one of the two sounds
        let sound_effect = if rand::random() {
            audio_assets.enemy_hit_wall_1.clone()
        } else {
            audio_assets.enemy_hit_wall_2.clone()
        };
        audio.play_with_settings(
            sound_effect,
            PlaybackSettings {
                volume: 0.5,
                ..Default::default()
            },
        );
    }
}
