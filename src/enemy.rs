use crate::loading::TextureAssets;
use crate::physics::{PlayingAreaBorder, PLAY_AREA_BORDER_MARGIN};
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

const ENEMY_SPEED: f32 = 100.;
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
            .add_system(move_enemy_controller.in_set(OnUpdate(GameState::Playing)))
            .add_system(check_for_collisions.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let x = random::<f32>() * (window.width() - PLAY_AREA_BORDER_MARGIN - ENEMY_SIZE);
        let y = random::<f32>() * (window.height() - PLAY_AREA_BORDER_MARGIN - ENEMY_SIZE);
        commands
            .spawn(SpriteBundle {
                texture: textures.enemy_ball.clone(),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert(Collider::ball(ENEMY_SIZE / 2.0))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::linear(Vec2::new(
                random::<f32>() * 2. - 1.,
                random::<f32>() * 2. - 1.,
            )))
            .insert(GravityScale(0.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Enemy {
                direction: Vec2::new(random::<f32>() * 2. - 1., random::<f32>() * 2. - 1.),
            });
    }
}

fn move_enemy_controller(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Enemy, &mut Velocity), With<Enemy>>,
) {
    for (enemy, mut enemy_controller_velocity) in &mut enemy_query {
        enemy_controller_velocity.linvel = Vec2::new(
            enemy.direction.x * ENEMY_SPEED * time.delta_seconds(),
            enemy.direction.y * ENEMY_SPEED * time.delta_seconds(),
        ) * ENEMY_SPEED
            * time.delta_seconds()
            * 100.;
    }
}

fn check_for_collisions(
    mut enemy_collider_query: Query<
        (Entity, &mut Enemy, &mut Velocity),
        (With<Collider>, With<Enemy>),
    >,
    mut wall_collider_query: Query<Entity, (With<Collider>, With<PlayingAreaBorder>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut direction_changed_event: EventWriter<EnemyDirectionChangedEvent>,
    rapier_context: Res<RapierContext>,
) {
    for event in collision_events.iter() {
        println!("{:?}", event);
        match event {
            CollisionEvent::Started(a, b, _) => {
                let enemy = if let Ok(a) = enemy_collider_query.get_mut(*a) {
                    Some(a)
                } else if let Ok(b) = enemy_collider_query.get_mut(*b) {
                    Some(b)
                } else {
                    None
                };

                let mut wall = if let Ok(a) = wall_collider_query.get_mut(*a) {
                    Some(a)
                } else if let Ok(b) = wall_collider_query.get_mut(*b) {
                    Some(b)
                } else {
                    None
                };

                if enemy.is_some() && wall.is_some() {
                    println!("Enemy hit wall");
                    let (enemy_entity, mut enemy, _) = enemy.unwrap();

                    if let Some(contact_pair) =
                        rapier_context.contact_pair(enemy_entity, wall.unwrap())
                    {
                        let mut direction_changed = false;
                        for manifold in contact_pair.manifolds() {
                            if manifold.local_n1().x == 1. || manifold.local_n1().x == -1. {
                                direction_changed = true;
                                enemy.direction.x *= -1.;
                            } else if manifold.local_n1().y == 1. || manifold.local_n1().y == -1. {
                                direction_changed = true;
                                enemy.direction.y *= -1.;
                            }
                        }
                        if direction_changed {
                            direction_changed_event.send_default();
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
