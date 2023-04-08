use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod systems;

use systems::*;

type RapierPlugin = RapierPhysicsPlugin<NoUserData>;

pub struct InternalPhysicsPlugin;

// Bevy Plugin for handling rapier physics
impl Plugin for InternalPhysicsPlugin {
    fn build(&self, app: &mut App) {
        let rapier_config = RapierConfiguration {
            gravity: Vec2::ZERO,
            // timestep_mode: TimestepMode::Fixed {
            //     dt: PHYSICS_TIMESTEP,
            //     substeps: 1,
            // },
            ..default()
        };

        app.insert_resource(rapier_config)
            .add_plugin(RapierPlugin::pixels_per_meter(100.))
            .add_plugin(RapierDebugRenderPlugin {
                always_on_top: true,
                enabled: true,
                ..default()
            });

        #[cfg(debug_assertions)]
        {
            // app.add_system(display_collision_events.in_set(OnUpdate(GameState::Playing)));
        }
    }
}
