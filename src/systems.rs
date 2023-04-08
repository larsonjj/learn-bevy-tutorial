use bevy::{app::AppExit, prelude::*};

use crate::{actions::resources::Actions, events::*, resources::*};

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn control_game_exit_event(actions: Res<Actions>, mut app_exit_events: EventWriter<AppExit>) {
    if actions.exit_game {
        app_exit_events.send(AppExit);
    }
}

pub fn handle_game_over_event(mut game_over_events: EventReader<GameOverEvent>, score: Res<Score>) {
    if !game_over_events.is_empty() {
        // This prevents events staying active on the next frame.
        game_over_events.clear();
        println!("Game Over! Score: {}", score.value);
    }
}
