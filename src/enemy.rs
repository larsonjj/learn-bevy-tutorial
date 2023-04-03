use crate::loading::TextureAssets;
use crate::physics::PLAY_AREA_BORDER_MARGIN;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

const ENEMY_SPEED: f32 = 150.;
const ENEMY_SIZE: f32 = 64.;
const NUMBER_OF_ENEMIES: usize = 5;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    direction: Vec2,
}

#[derive(Default)]
pub struct EnemyDirectionChangedEvent;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDirectionChangedEvent>()
            .add_system(spawn_enemies.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update_enemy_movement.in_set(OnUpdate(GameState::Playing)))
            .add_system(update_enemy_direction.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let x = random::<f32>() * (window.width() - PLAY_AREA_BORDER_MARGIN);
        let y = random::<f32>() * (window.height() - PLAY_AREA_BORDER_MARGIN);
        commands
            .spawn(SpriteBundle {
                texture: textures.enemy_ball.clone(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert(Collider::ball(ENEMY_SIZE / 2.0))
            .insert(RigidBody::Dynamic)
            .insert(GravityScale(0.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Enemy {
                direction: Vec2::new(random::<f32>() * 2. - 1., random::<f32>() * 2. - 1.),
            });
    }
}

pub fn update_enemy_movement(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform), With<Enemy>>,
) {
    for (enemy, mut enemy_transform) in &mut enemy_query {
        let speed = ENEMY_SPEED;
        let movement = Vec3::new(
            enemy.direction.x * speed * time.delta_seconds(),
            enemy.direction.y * speed * time.delta_seconds(),
            0.,
        );
        enemy_transform.translation += movement;
    }
}

// Flip the enemy's direction if it hits the window's edge
pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Enemy, &mut Transform), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut direction_changed_event: EventWriter<EnemyDirectionChangedEvent>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (mut enemy, enemy_transform) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = enemy_transform.translation;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.;
            direction_changed = true;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.;
            direction_changed = true;
        }

        // Play sound effect if direction changed
        if direction_changed {
            direction_changed_event.send_default();
        }
    }
}
