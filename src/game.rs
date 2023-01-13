use bevy::{
    prelude::{
        info, Commands, Component, Entity, EventReader, Local, Query, Res, ResMut, Resource,
        Transform, With,
    },
    time::{Time, Timer, TimerMode},
};
use bevy_rapier2d::prelude::CollisionEvent;
use iyes_loopless::state::NextState;

use crate::{
    app_states::{AppState, InGameState},
    consts::{ACCELERATION_FACTOR, BASE_MOVE_SPEED, GAME_WIDTH, SECONDS_BETWEEN_ACCELERATION_TICK},
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
        info!("Lets go baby {}", game_speed.factor);
    }
}

pub fn ground_buffer_swap(mut q_ground_elements: Query<&mut Transform, With<Ground>>) {
    for mut t in q_ground_elements.iter_mut() {
        if t.translation.x < -GAME_WIDTH / 2. {
            info!("Swap ! {} was less than {}", t.translation.x, -GAME_WIDTH);
            t.translation.x += 2.0 * GAME_WIDTH;
        }
    }
}

pub fn change_state_to_gameover_on_death_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    q_player: Query<Entity, With<Player>>,
) {
    for col_ev in collision_events.iter() {
        match col_ev {
            CollisionEvent::Started(ent_a, ent_b, _) => {
                if q_player.contains(ent_a.clone()) || q_player.contains(ent_b.clone()) {
                    commands.insert_resource(NextState(AppState::InGame(InGameState::ReadyToStart)))
                }
            }
            _ => {}
        }
    }
}
