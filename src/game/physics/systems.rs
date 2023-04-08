use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn run_simulation(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = true;
}

pub fn pause_simulation(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = false;
}

pub fn display_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
