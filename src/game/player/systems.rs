use super::components::*;
use super::events::*;
use super::{PLAYER_SIZE, PLAYER_SPEED};
use crate::actions::resources::Actions;
use crate::asset_loader::resources::*;
use crate::game::enemy::components::Enemy;
use crate::game::events::GameOverEvent;
use crate::game::resources::Score;
use crate::game::star::components::Star;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands
        .spawn(SpriteBundle {
            texture: textures.texture_ball.clone(),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        })
        .insert(Player)
        .insert(KinematicCharacterController {
            slide: true,
            filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
            ..default()
        })
        .insert(Collider::ball(PLAYER_SIZE / 2.0))
        .insert(Velocity::linear(Vec2::ZERO))
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(ActiveEvents::COLLISION_EVENTS);
}

pub fn move_player_controller(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    for mut player_controller in &mut player_query {
        player_controller.translation = Some(Vec2::new(
            actions.player_movement.unwrap().x * PLAYER_SPEED * time.delta_seconds(),
            actions.player_movement.unwrap().y * PLAYER_SPEED * time.delta_seconds(),
        ));
    }
}

pub fn check_for_world_collisions(
    mut commands: Commands,
    mut enemy_collider_query: Query<(Entity, &mut Enemy), (With<Collider>, With<Enemy>)>,
    mut player_collider_query: Query<(Entity, &Player), (With<Collider>, With<Player>)>,
    mut star_collider_query: Query<(Entity, &Star), (With<Collider>, With<Star>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_died_event: EventWriter<PlayerHitEnemyEvent>,
    mut game_over_event: EventWriter<GameOverEvent>,
    mut star_pickup_event: EventWriter<PlayerStarPickupEvent>,
    mut score: ResMut<Score>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                let player = if let Ok(a) = player_collider_query.get_mut(*a) {
                    Some(a)
                } else if let Ok(b) = player_collider_query.get_mut(*b) {
                    Some(b)
                } else {
                    None
                };

                let enemy = if let Ok(a) = enemy_collider_query.get_mut(*a) {
                    Some(a)
                } else if let Ok(b) = enemy_collider_query.get_mut(*b) {
                    Some(b)
                } else {
                    None
                };

                let star = if let Ok(a) = star_collider_query.get_mut(*a) {
                    Some(a)
                } else if let Ok(b) = star_collider_query.get_mut(*b) {
                    Some(b)
                } else {
                    None
                };

                if enemy.is_some() && player.is_some() {
                    // Send player died event
                    player_died_event.send_default();

                    // Send game over event
                    game_over_event.send_default();

                    // Despawn the player
                    commands.entity(player.unwrap().0).despawn();
                } else if player.is_some() && star.is_some() {
                    // Send star pickup event
                    star_pickup_event.send_default();

                    // Increment score
                    score.value += 1;

                    // Despawn the star
                    commands.entity(star.unwrap().0).despawn();
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

pub fn play_player_hit_enemy_sound(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut player_died_events: EventReader<PlayerHitEnemyEvent>,
) {
    if !player_died_events.is_empty() {
        // This prevents events staying active on the next frame.
        player_died_events.clear();

        // Randomely play one of the two sounds
        audio.play_with_settings(
            audio_assets.player_died.clone(),
            PlaybackSettings {
                volume: 0.5,
                ..Default::default()
            },
        );
    }
}

pub fn play_star_pickup_sound(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut star_pickup_events: EventReader<PlayerStarPickupEvent>,
) {
    if !star_pickup_events.is_empty() {
        // This prevents events staying active on the next frame.
        star_pickup_events.clear();

        // Randomely play one of the two sounds
        audio.play_with_settings(
            audio_assets.star_pickup.clone(),
            PlaybackSettings {
                volume: 0.5,
                ..Default::default()
            },
        );
    }
}
