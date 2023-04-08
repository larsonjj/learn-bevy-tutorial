use bevy::prelude::*;

use super::{events::GameOverEvent, resources::Score};

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn handle_game_over_event(mut game_over_events: EventReader<GameOverEvent>, score: Res<Score>) {
    if !game_over_events.is_empty() {
        // This prevents events staying active on the next frame.
        game_over_events.clear();
        println!("Game Over! Score: {}", score.value);
    }
}
