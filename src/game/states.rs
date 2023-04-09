use bevy::prelude::*;

// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum SimulationState {
    Running,
    Paused,
    #[default]
    Stopped,
}
