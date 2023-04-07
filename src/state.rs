use bevy::app::AppExit;
use bevy::prelude::*;

use crate::{actions::resources::Actions, star::StarPickupEvent, GameState};

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

#[derive(Default)]
pub struct GameOverEvent {
    pub score: u32,
}

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .init_resource::<Score>()
            .add_system(update_score)
            .add_system(control_game_exit_event)
            .add_system(handle_game_over_event.in_set(OnUpdate(GameState::Playing)))
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
        // This prevents events staying active on the next frame.
        star_pickup_events.clear();

        score.value += 1;
    }
}

fn control_game_exit_event(actions: Res<Actions>, mut app_exit_events: EventWriter<AppExit>) {
    if actions.exit_game {
        app_exit_events.send(AppExit);
    }
}

fn handle_game_over_event(mut game_over_events: EventReader<GameOverEvent>) {
    if !game_over_events.is_empty() {
        for event in game_over_events.iter() {
            println!("Game Over! Score: {}", event.score);
        }
    }
}
