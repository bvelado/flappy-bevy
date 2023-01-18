use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, With};
use bevy_rapier2d::prelude::CollisionEvent;
use iyes_loopless::state::NextState;

use crate::{
    app_states::{AppState, InGameState},
    obstacles::Obstacle,
    player::Player,
};

#[derive(Debug, Eq, PartialEq)]
pub enum GameEvent {
    PlayerHitGameOverCollider,
    PlayerPassedAnOpening,
}

pub fn dispatch_collision_events(
    mut ev_collisions: EventReader<CollisionEvent>,
    mut ev_game: EventWriter<GameEvent>,
    q_player: Query<Entity, With<Player>>,
    q_obstacles: Query<(Entity, &Obstacle)>,
) {
    for col_ev in ev_collisions.iter() {
        match col_ev {
            CollisionEvent::Started(ent_a, ent_b, _) => {
                if q_player.contains(ent_a.clone()) && q_obstacles.contains(ent_b.clone()) {
                    handle_player_obstacle_collision(
                        q_obstacles.get(ent_b.clone()).unwrap(),
                        &mut ev_game,
                    );
                } else if q_player.contains(ent_b.clone()) && q_obstacles.contains(ent_a.clone()) {
                    handle_player_obstacle_collision(
                        q_obstacles.get(ent_a.clone()).unwrap(),
                        &mut ev_game,
                    );
                }
            }
            _ => {}
        }
    }
}

fn handle_player_obstacle_collision(
    (_, obstacle): (Entity, &Obstacle),
    ev_game: &mut EventWriter<GameEvent>,
) {
    match obstacle {
        Obstacle::GameOverStatic | Obstacle::GameOver => {
            ev_game.send(GameEvent::PlayerHitGameOverCollider)
        }
        Obstacle::Opening => ev_game.send(GameEvent::PlayerPassedAnOpening),
    }
}

pub fn handle_game_event_player_hit_game_over_collider(
    mut commands: Commands,
    mut ev_game: EventReader<GameEvent>,
) {
    for _ in ev_game
        .iter()
        .filter(|&ev| ev == &GameEvent::PlayerHitGameOverCollider)
    {
        commands.insert_resource(NextState(AppState::InGame(InGameState::ReadyToStart)));
    }
}
