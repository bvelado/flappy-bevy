mod animations;
mod app_states;
mod assets;
mod camera;
mod consts;
mod game;
mod player;
mod world;

use animations::animate_sprite;
use bevy::{prelude::*, window::PresentMode};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use consts::{BASE_GAME_SPEED, GAME_HEIGHT, GAME_WIDTH, GRAVITY};
use game::{ground_buffer_swap, move_game_elements_horizontal, update_game_speed, GameSpeed};
use iyes_loopless::prelude::*;

use app_states::{AppState, InGameState, LaunchingState};
use assets::{load_game_assets, start_game_when_assets_loaded};
use camera::spawn_camera;
use player::{player_jump, spawn_player};
use world::spawn_world_ground;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("dff6f5").unwrap()))
        .insert_resource(GameSpeed {
            factor: BASE_GAME_SPEED,
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, GRAVITY),
            ..Default::default()
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Flappy Bevy 🐦".to_string(),
                        height: GAME_HEIGHT,
                        width: GAME_WIDTH,
                        present_mode: PresentMode::AutoVsync,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(36.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_loopless_state(AppState::Launching(LaunchingState::Loading))
        .add_startup_system(load_game_assets)
        .add_system(
            start_game_when_assets_loaded
                .run_in_state(AppState::Launching(LaunchingState::Loading)),
        )
        .add_system(animate_sprite.run_in_state(AppState::InGame(InGameState::Playing)))
        .add_system(
            move_game_elements_horizontal.run_in_state(AppState::InGame(InGameState::Playing)),
        )
        .add_system(update_game_speed.run_in_state(AppState::InGame(InGameState::Playing)))
        .add_system(ground_buffer_swap.run_in_state(AppState::InGame(InGameState::Playing)))
        .add_system(player_jump.run_in_state(AppState::InGame(InGameState::Playing)))
        // .add_enter_system(
        //     AppState::InGame(InGameState::Playing),
        //     spawn_world_background,
        // )
        .add_enter_system(AppState::InGame(InGameState::Playing), spawn_world_ground)
        .add_enter_system(AppState::InGame(InGameState::Playing), spawn_player)
        .add_enter_system(AppState::InGame(InGameState::Playing), spawn_camera)
        .run();
}
