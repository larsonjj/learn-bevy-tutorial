use crate::loading::TextureAssets;
use crate::physics::PLAY_AREA_BORDER_MARGIN;
use crate::player::Player;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

const STAR_SIZE: f32 = 30.;
const STAR_SPAWN_TIME: f32 = 5.0;

pub struct StarPlugin;

#[derive(Component, Debug)]
pub struct Star;

#[derive(Default)]
pub struct StarPickupEvent;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_event::<StarPickupEvent>()
            // .add_system(spawn_stars.in_schedule(OnEnter(GameState::Playing)))
            .add_system(tick_star_spawn_timer.in_set(OnUpdate(GameState::Playing)))
            .add_system(check_for_world_collisions.in_set(OnUpdate(GameState::Playing)))
            .add_system(spawn_stars_over_time.in_set(OnUpdate(GameState::Playing)));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    textures: Res<TextureAssets>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * (window.width() - PLAY_AREA_BORDER_MARGIN - STAR_SIZE);
        let random_y = random::<f32>() * (window.height() - PLAY_AREA_BORDER_MARGIN - STAR_SIZE);

        commands
            .spawn(SpriteBundle {
                texture: textures.star.clone(),
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(STAR_SIZE / 2.0))
            .insert(Sensor)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Star);
    }
}

fn check_for_world_collisions(
    mut commands: Commands,
    mut player_collider_query: Query<(Entity, &mut Player), (With<Collider>, With<Player>)>,
    mut star_collider_query: Query<(Entity, &Star), (With<Collider>, With<Star>)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut star_pickup_event: EventWriter<StarPickupEvent>,
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

                let star = if let Ok(a) = star_collider_query.get_mut(*a) {
                    Some(a)
                } else if let Ok(b) = star_collider_query.get_mut(*b) {
                    Some(b)
                } else {
                    None
                };

                if player.is_some() && star.is_some() {
                    // Play death sound and update score
                    star_pickup_event.send_default();
                    // Despawn the player
                    commands.entity(star.unwrap().0).despawn();
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
