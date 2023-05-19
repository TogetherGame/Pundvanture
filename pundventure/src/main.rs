mod plugins;
mod state;
mod ecs_common;

use plugins::*;

use bevy::prelude::*;
use state::GameState;
use ecs_common::{systems, components::Fonts};

pub const GAME_TITLE: &str = "Pundventure";

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
                        title: GAME_TITLE.into(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<GameState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(CreditPlugin)
        .add_startup_system(startup)
        .add_system(debug_input_system)
        .add_systems((
            systems::load_fonts.in_schedule(OnEnter(GameState::Loading)),
            asset_loading_tracker.in_set(OnUpdate(GameState::Loading)),
        ))
        .run()
}

fn startup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

/// Change game state to show main menu after all ui assets are loaded.
fn asset_loading_tracker(_fonts: Res<Fonts>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::MainMenu);
}

fn debug_input_system(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.pressed(KeyCode::G) {
        next_state.set(GameState::Game);
    }
    if input.pressed(KeyCode::S) {
        next_state.set(GameState::Splash);
    }
    if input.pressed(KeyCode::L) {
        next_state.set(GameState::Loading);
    }
}
