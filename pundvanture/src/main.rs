mod plugins;
mod state;

use plugins::*;

use bevy::prelude::*;
use state::GameState;

fn main() {
    #[cfg(feature = "hot-reloading")]
    let watch_for_changes = true;
    #[cfg(not(feature = "hot-reloading"))]
    let watch_for_changes = false;

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pundvanture".into(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<GameState>()
        .add_plugin(CreditPlugin)
        .add_startup_system(startup)
        .add_system(debug_input_system)
        .run()
}

fn startup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn debug_input_system(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.pressed(KeyCode::C) {
        next_state.set(GameState::ShowCredits);
    }
    if input.pressed(KeyCode::G) {
        next_state.set(GameState::Game);
    }
    if input.pressed(KeyCode::M) {
        next_state.set(GameState::MainMenu);
    }
}
