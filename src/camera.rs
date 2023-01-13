use bevy::prelude::{Camera2dBundle, Commands, Component};

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
