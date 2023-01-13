use bevy::prelude::{Commands, Input, KeyCode, MouseButton, Res};
use iyes_loopless::state::NextState;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum AppState {
    Launching(LaunchingState),
    InGame(InGameState),
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum LaunchingState {
    Loading,
    ReadyToLaunch,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub enum InGameState {
    Initialization,
    ReadyToStart,
    Playing,
}

pub fn change_state_to_playing_on_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
        commands.insert_resource(NextState(AppState::InGame(InGameState::Playing)))
    }
}
