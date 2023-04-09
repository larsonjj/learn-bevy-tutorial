use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod systems;

use systems::*;

use crate::states::AppState;

use super::states::SimulationState;

type RapierPlugin = RapierPhysicsPlugin<NoUserData>;

pub struct GamePhysicsPlugin;

// Bevy Plugin for handling rapier physics
impl Plugin for GamePhysicsPlugin {
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
            .add_system(run_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_system(
                run_simulation
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(
                pause_simulation
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Paused)),
            )
            .add_system(pause_simulation.in_schedule(OnExit(AppState::Game)));

        #[cfg(debug_assertions)]
        {
            app.add_plugin(RapierDebugRenderPlugin {
                always_on_top: true,
                enabled: true,
                ..default()
            })
            .add_system(display_collision_events.in_set(OnUpdate(AppState::Game)));
        }
    }
}
