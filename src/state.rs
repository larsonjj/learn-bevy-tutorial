use bevy::prelude::*;

use crate::{star::StarPickupEvent, GameState};

pub struct StatePlugin;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_system(update_score)
            .add_system(control_star_pickup_event.in_set(OnUpdate(GameState::Playing)));
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

fn control_star_pickup_event(
    mut star_pickup_events: EventReader<StarPickupEvent>,
    mut score: ResMut<Score>,
) {
    if !star_pickup_events.is_empty() {
        println!("Star picked up!");
        // This prevents events staying active on the next frame.
        star_pickup_events.clear();

        score.value += 1;
    }
}
