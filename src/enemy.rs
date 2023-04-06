use crate::loading::TextureAssets;
use crate::physics::PLAY_AREA_BORDER_MARGIN;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

const ENEMY_SPEED: f32 = 150.;
const ENEMY_SIZE: f32 = 64.;
const NUMBER_OF_ENEMIES: usize = 3;

pub struct EnemyPlugin;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Default)]
pub struct EnemyDirectionChangedEvent;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDirectionChangedEvent>()
            .add_system(spawn_enemies.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_enemy_controller.in_set(OnUpdate(GameState::Playing)));
        // .add_system(detect_collisions.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * (window.width() - PLAY_AREA_BORDER_MARGIN);
        let random_y = random::<f32>() * (window.height() - PLAY_AREA_BORDER_MARGIN);
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
            // .insert(ColliderMassProperties::Density(1.0))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Enemy);
    }
}

fn move_enemy_controller(mut enemy_query: Query<&mut Velocity, With<Enemy>>) {
    for mut enemy_velocity in &mut enemy_query {
        // Keep constant speed at all times for enemies
        enemy_velocity.linvel = enemy_velocity.linvel.normalize() * ENEMY_SPEED;
    }
}

// fn detect_collisions(
//     mut character_controller_outputs: Query<
//         (&mut Enemy, &mut KinematicCharacterControllerOutput),
//         With<Enemy>,
//     >,
//     mut direction_changed_event: EventWriter<EnemyDirectionChangedEvent>,
// ) {
//     for (mut enemy, mut output) in character_controller_outputs.iter_mut() {
//         for collision in &output.collisions {
//             let mut direction_changed = false;
//             if collision.toi.normal1.x == 1. || collision.toi.normal1.x == -1. {
//                 direction_changed = true;
//                 enemy.direction.x *= -1.;
//             } else if collision.toi.normal1.y == 1. || collision.toi.normal1.y == -1. {
//                 direction_changed = true;
//                 enemy.direction.y *= -1.;
//             }
//             println!("Direction {:?}", enemy.direction)
//             // if direction_changed {
//             //     direction_changed_event.send_default();
//             // }
//         }
//     }
// }
