use super::components::*;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

// This system is responsible for setting up the play area pyhsics bounds
pub fn spawn_walls(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    // Create the play area bounds
    let play_area_width = window.width();
    let play_area_height = window.height();

    commands
        .spawn(SpatialBundle::default())
        .insert(RigidBody::Fixed)
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Collider::polyline(
            vec![
                Vect::new(0., 0.),
                Vect::new(play_area_width, 0.),
                Vect::new(play_area_width, play_area_height),
                Vect::new(0., play_area_height),
            ],
            Some(vec![[0, 1], [1, 2], [2, 3], [3, 0]]),
        ))
        .insert(Walls);
}

pub fn despawn_walls(mut commands: Commands, walls_query: Query<Entity, With<Walls>>) {
    if let Ok(walls_entity) = walls_query.get_single() {
        commands.entity(walls_entity).despawn_recursive();
    }
}
