//! This plugin will load assets' credit files and
//! display some of the useful information on a UI panel.

use assets_loader::{Credits, LoaderPlugin};
use bevy::prelude::*;

use crate::state::MainMenuState;

pub(crate) struct CreditPlugin;

#[derive(Resource)]
struct CreditRes(Handle<Credits>);

impl Plugin for CreditPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoaderPlugin::<Credits>::json())
            .add_system(init_credit.in_schedule(OnEnter(MainMenuState::Credits)))
            .add_system(show_credits.in_set(OnUpdate(MainMenuState::Credits)));
    }
}

fn init_credit(
    mut cmd: Commands,
    assets: Res<AssetServer>,
) {
    let credit_handle: Handle<Credits> = assets.load("credits.json");
    cmd.insert_resource(CreditRes(credit_handle));
}

fn show_credits(mut _cmd: Commands, credits: Res<CreditRes>, credit_assets: Res<Assets<Credits>>) {
    if let Some(cre) = credit_assets.get(&credits.0) {
        println!("{:?}", cre);
    }
}
