use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::GameState;

const PLAY_AREA_BORDER_MARGIN: f32 = 50.0;

pub struct InternalPhysicsPlugin;

#[derive(Component)]
pub struct PlayingAreaBorder;

// Bevy Plugin for handling rapier physics
impl Plugin for InternalPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_system(setup_play_area.in_schedule(OnEnter(GameState::Playing)));
    }
}

// This system is responsible for setting up the play area pyhsics bounds
fn setup_play_area(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    // Create the play area bounds
    let play_area_width = window.width() - PLAY_AREA_BORDER_MARGIN;
    let play_area_height = window.height() - PLAY_AREA_BORDER_MARGIN;

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
                Vect::new(PLAY_AREA_BORDER_MARGIN, PLAY_AREA_BORDER_MARGIN),
                Vect::new(play_area_width, PLAY_AREA_BORDER_MARGIN),
                Vect::new(play_area_width, play_area_height),
                Vect::new(PLAY_AREA_BORDER_MARGIN, play_area_height),
            ],
            Some(vec![[0, 1], [1, 2], [2, 3], [3, 0]]),
        ))
        .insert(PlayingAreaBorder);
}
