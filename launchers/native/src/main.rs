use bevy::prelude::info;

fn main() {
    let mut app = flappy_bevy::build_game_app(flappy_bevy::BuildGameAppData {
        canvas: None,
        window_title: Some("Flappy Bevy - 🐦".to_string()),
    });

    info!("Starting launcher: Native");
    app.run();
}
