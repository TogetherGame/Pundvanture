mod plugins;

use bevy::prelude::*;

fn main() {
    #[cfg(feature = "hot-reloading")]
    let watch_for_changes = true;
    #[cfg(not(feature = "hot-reloading"))]
    let watch_for_changes = false;

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin { watch_for_changes, ..default() })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pundvanture".into(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_startup_system(startup)
        .run()
}

fn startup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
