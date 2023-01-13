use bevy::{
    asset::LoadState,
    prelude::{AssetServer, Commands, Handle, Image, Res, Resource},
};
use iyes_loopless::state::NextState;

use crate::app_states::{AppState, InGameState};

#[derive(Resource)]
pub struct GameAssets {
    pub background_image: Handle<Image>,
    pub obstacle_image: Handle<Image>,
    pub ground_image: Handle<Image>,
    pub characters_image: Handle<Image>,
}

pub fn load_game_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        background_image: asset_server.load("background.png"),
        obstacle_image: asset_server.load("obstacle.png"),
        ground_image: asset_server.load("ground.png"),
        characters_image: asset_server.load("characters.png"),
    })
}

pub fn start_game_when_assets_loaded(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    if asset_server.get_load_state(game_assets.background_image.clone()) == LoadState::Loaded
        && asset_server.get_load_state(game_assets.ground_image.clone()) == LoadState::Loaded
        && asset_server.get_load_state(game_assets.obstacle_image.clone()) == LoadState::Loaded
        && asset_server.get_load_state(game_assets.characters_image.clone()) == LoadState::Loaded
    {
        commands.insert_resource(NextState(AppState::InGame(InGameState::Playing)))
    }
}
