use super::states::SimulationState;
use crate::{actions::resources::Actions, states::AppState};
use bevy::prelude::*;

use super::{events::GameOverEvent, resources::Score};

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    actions: Res<Actions>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if actions.pause {
        println!("Pause button pressed. {:?}", actions.pause);
        if simulation_state.0 == SimulationState::Running {
            simulation_state_next_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
        }
        if simulation_state.0 == SimulationState::Paused {
            simulation_state_next_state.set(SimulationState::Running);
            println!("Simulation Running.");
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn handle_game_over_event(
    mut game_over_events: EventReader<GameOverEvent>,
    score: Res<Score>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if !game_over_events.is_empty() {
        // This prevents events staying active on the next frame.
        game_over_events.clear();
        println!("Game Over! Score: {}", score.value);
        app_state_next_state.set(AppState::GameOver);
    }
}
