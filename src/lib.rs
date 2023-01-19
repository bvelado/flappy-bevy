mod animations;
mod app_states;
mod assets;
mod camera;
mod consts;
mod events;
mod game;
mod obstacles;
mod player;
mod ui;
mod world;

// use bevy_prototype_lyon::prelude::*;
use bevy::{prelude::*, window::PresentMode};
use bevy_rapier2d::{
    prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
};
use bevy_turborand::prelude::*;
use events::{
    dispatch_collision_events, handle_game_event_player_hit_game_over_collider, GameEvent,
};
use iyes_loopless::prelude::*;

use animations::animate_sprite;
use app_states::{change_state_to_playing_on_input, AppState, InGameState, LaunchingState};
use assets::{change_state_to_ingame_when_assets_loaded, load_game_assets};
use camera::spawn_camera;
use consts::{BASE_GAME_SPEED, GAME_HEIGHT, GAME_WIDTH, GRAVITY};
use game::{ground_buffer_swap, move_game_elements_horizontal, update_game_speed, GameSpeed};
use obstacles::{
    despawn_passed_obstacles, reset_obstacles_state, spawn_obstacles, update_obstacles_data,
    ObstaclesData,
};
use player::{
    enable_player_gravity, handle_game_event_player_passed_opening, player_jump,
    reset_player_score, reset_player_state, spawn_player, PlayerScore,
};
use ui::{
    despawn_game_ready_label, despawn_game_score, spawn_game_ready_label, spawn_game_score,
    update_player_score_label,
};
use world::spawn_world_ground;

pub struct BuildGameAppData {
    pub canvas: Option<String>,
    pub window_title: Option<String>,
}

pub fn build_game_app(data: BuildGameAppData) -> App {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::hex("dff6f5").unwrap()))
        .insert_resource(GameSpeed {
            factor: BASE_GAME_SPEED,
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, GRAVITY),
            ..Default::default()
        })
        .init_resource::<ObstaclesData>()
        .init_resource::<PlayerScore>()
        .add_event::<GameEvent>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: data.window_title.unwrap_or("Flappy Bevy".to_string()),
                        height: GAME_HEIGHT,
                        width: GAME_WIDTH,
                        canvas: data.canvas,
                        present_mode: PresentMode::AutoVsync,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(36.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RngPlugin::default())
        .add_loopless_state(AppState::Launching(LaunchingState::Loading))
        // LAUNCHING - LOADING
        .add_startup_system(load_game_assets)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Launching(LaunchingState::Loading))
                .with_system(change_state_to_ingame_when_assets_loaded)
                .into(),
        )
        // LAUNCHING - READY
        //
        // IN GAME - INITIALIZATION
        .add_enter_system_set(
            AppState::InGame(InGameState::Initialization),
            ConditionSet::new()
                .with_system(spawn_world_ground)
                .with_system(spawn_player)
                .with_system(spawn_camera)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame(InGameState::Initialization))
                .with_system(change_state_to_ready_to_start)
                .into(),
        )
        // IN GAME - READY TO START
        .add_enter_system_set(
            AppState::InGame(InGameState::ReadyToStart),
            ConditionSet::new()
                .with_system(reset_player_state)
                .with_system(reset_player_score)
                .with_system(reset_obstacles_state)
                .with_system(spawn_game_ready_label)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame(InGameState::ReadyToStart))
                .with_system(change_state_to_playing_on_input)
                .with_system(animate_sprite)
                .into(),
        )
        .add_exit_system_set(
            AppState::InGame(InGameState::ReadyToStart),
            ConditionSet::new()
                .with_system(despawn_game_ready_label)
                .into(),
        )
        // IN GAME - PLAYING
        .add_enter_system_set(
            AppState::InGame(InGameState::Playing),
            ConditionSet::new()
                .with_system(enable_player_gravity)
                .with_system(spawn_game_score)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame(InGameState::Playing))
                .with_system(animate_sprite)
                .with_system(move_game_elements_horizontal)
                .with_system(update_game_speed)
                .with_system(ground_buffer_swap)
                .with_system(player_jump)
                .with_system(update_obstacles_data)
                .with_system(spawn_obstacles)
                .with_system(despawn_passed_obstacles)
                .with_system(update_player_score_label)
                .with_system(dispatch_collision_events)
                .with_system(handle_game_event_player_hit_game_over_collider)
                .with_system(handle_game_event_player_passed_opening)
                .into(),
        )
        .add_exit_system_set(
            AppState::InGame(InGameState::Playing),
            ConditionSet::new().with_system(despawn_game_score).into(),
        );
    app
}

fn change_state_to_ready_to_start(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::InGame(InGameState::ReadyToStart)));
}
