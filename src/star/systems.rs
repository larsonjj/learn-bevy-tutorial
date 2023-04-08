use super::components::*;
use super::resources::*;
use crate::asset_loader::resources::*;
use crate::walls::systems::WALLS_MARGIN;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

const STAR_SIZE: f32 = 30.;

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    star_spawn_timer: ResMut<StarSpawnTimer>,
    textures: Res<TextureAssets>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * (window.width() - WALLS_MARGIN - STAR_SIZE);
        let random_y = random::<f32>() * (window.height() - WALLS_MARGIN - STAR_SIZE);

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
