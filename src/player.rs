use crate::actions::Actions;
use crate::enemy::Enemy;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Default)]
pub struct PlayerDiedEvent;

const PLAYER_SPEED: f32 = 300.;
const PLAYER_SIZE: f32 = 64.;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDiedEvent>()
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_player_controller.in_set(OnUpdate(GameState::Playing)))
            .add_system(check_for_world_collisions.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_player(
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
        // .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController {
            slide: true,
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

fn move_player_controller(
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

// Detects when player collides with another collider (ONLY DETECTS WHILE MOVING)
// fn detect_collisions(
//     mut commands: Commands,
//     mut character_controller_query: Query<
//         (&mut KinematicCharacterControllerOutput, Entity),
//         With<Player>,
//     >,
//     mut enemy_query: Query<&mut Enemy>,
// ) {
//     for (player_controller, player) in character_controller_query.iter() {
//         for collision in &player_controller.collisions {
//             if let Ok(selected_enemy) = enemy_query.get(collision.entity) {
//                 println!("Player collided with enemy: {:?}", selected_enemy);
//                 commands.entity(player).despawn();
//             }
//         }
//     }
// }

fn check_for_world_collisions(
    mut commands: Commands,
    mut enemy_collider_query: Query<(Entity, &mut Enemy), (With<Collider>, With<Enemy>)>,
    mut player_collider_query: Query<(Entity, &Player), (With<Collider>, With<Player>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_died_event: EventWriter<PlayerDiedEvent>,
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

                let player = if let Ok(a) = player_collider_query.get_mut(*a) {
                    Some(a)
                } else if let Ok(b) = player_collider_query.get_mut(*b) {
                    Some(b)
                } else {
                    None
                };

                if enemy.is_some() && player.is_some() {
                    // Play death sound
                    player_died_event.send_default();
                    // Despawn the player
                    commands.entity(player.unwrap().0).despawn();
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
