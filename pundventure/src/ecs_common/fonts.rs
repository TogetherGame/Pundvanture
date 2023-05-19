use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct Fonts {
    pub ui: Handle<Font>
}

pub(crate) fn load_fonts(mut cmd: Commands, assets: Res<AssetServer>) {
    let ui_font_handle: Handle<Font> = assets.load("font/GnuUnifontFull.ttf");
    let fonts_res = Fonts {
        ui: ui_font_handle,
    };
    cmd.insert_resource(fonts_res);
}
