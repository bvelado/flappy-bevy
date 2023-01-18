use bevy::{
    prelude::{
        info, Commands, Component, Entity, EventReader, Local, Name, Query, Res, ResMut, Resource,
        Transform, With,
    },
    time::{Time, Timer, TimerMode},
};
use bevy_rapier2d::prelude::CollisionEvent;
use iyes_loopless::state::NextState;

use crate::{
    app_states::{AppState, InGameState},
    consts::{ACCELERATION_FACTOR, BASE_MOVE_SPEED, GAME_WIDTH, SECONDS_BETWEEN_ACCELERATION_TICK},
    obstacles::Obstacle,
    player::Player,
    world::Ground,
};

#[derive(Resource)]
pub struct GameSpeed {
    pub factor: f32,
}

#[derive(Component)]
pub struct HorizontalMove {
    pub factor: f32,
}

pub struct UpdateGameSpeedData {
    pub acceleration_tick_timer: Timer,
}

impl Default for UpdateGameSpeedData {
    fn default() -> Self {
        Self {
            acceleration_tick_timer: Timer::from_seconds(
                SECONDS_BETWEEN_ACCELERATION_TICK,
                TimerMode::Repeating,
            ),
        }
    }
}

pub fn move_game_elements_horizontal(
    mut q_elements: Query<(&HorizontalMove, &mut Transform)>,
    game_speed: Res<GameSpeed>,
    time: Res<Time>,
) {
    for (m, mut t) in q_elements.iter_mut() {
        t.translation.x -= BASE_MOVE_SPEED * m.factor * game_speed.factor * time.delta_seconds();
    }
}

pub fn update_game_speed(
    mut data: Local<UpdateGameSpeedData>,
    mut game_speed: ResMut<GameSpeed>,
    time: Res<Time>,
) {
    data.acceleration_tick_timer.tick(time.delta());
    if data.acceleration_tick_timer.just_finished() {
        game_speed.factor *= ACCELERATION_FACTOR;
    }
}

pub fn ground_buffer_swap(mut q_ground_elements: Query<&mut Transform, With<Ground>>) {
    for mut t in q_ground_elements.iter_mut() {
        if t.translation.x < -GAME_WIDTH / 2. {
            t.translation.x += 2.0 * GAME_WIDTH;
        }
    }
}