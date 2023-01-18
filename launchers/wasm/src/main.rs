use bevy::prelude::info;
use stylist::{css, yew::styled_component};
use yew::prelude::*;

fn set_window_title(title: &str) {
    web_sys::window()
        .map(|w| w.document())
        .flatten()
        .expect("Unable to get DOM")
        .set_title(title);
}

#[styled_component(Root)]
fn view() -> Html {
    set_window_title("Flappy Bevy - 🐦");

    let css = css!(
        r#"
        position: absolute;
        overflow: hidden;
        width: 100%;
        height: 100%;
        "#
    );

    html! {
        <div class={ css }>
            <canvas id="bevy"></canvas>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    yew::start_app::<Root>();
    // Start the Bevy App
    let mut app = flappy_bevy::build_game_app(flappy_bevy::BuildGameAppData {
        window_title: None,
        canvas: Some("#bevy".to_string()),
    });
    info!("Starting launcher: WASM");
    app.run();
}
