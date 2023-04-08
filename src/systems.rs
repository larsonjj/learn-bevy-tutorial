use bevy::{app::AppExit, prelude::*};

use crate::actions::resources::Actions;

pub fn control_game_exit_event(actions: Res<Actions>, mut app_exit_events: EventWriter<AppExit>) {
    if actions.exit_game {
        app_exit_events.send(AppExit);
    }
}
