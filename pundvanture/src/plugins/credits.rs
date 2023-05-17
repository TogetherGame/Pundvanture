//! This plugin will load assets' credit files and
//! display some of the useful information on a UI panel.

use bevy::prelude::*;
use assets_loader::{LoaderPlugin, Credits};

use crate::state::GameState;

pub(crate) struct CreditPlugin;

#[derive(Resource)]
struct CreditRes(Handle<Credits>);

impl Plugin for CreditPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoaderPlugin::json())
            .add_system(init_credit.in_schedule(OnEnter(GameState::ShowCredits)));
            //.add_system(show_credits.in_set(OnUpdate(GameState::ShowCredits)));
    }
}

fn init_credit(mut cmd: Commands, assets: Res<AssetServer>, mut state: ResMut<NextState<GameState>>) {
    let credit_handle: Handle<Credits> = assets.load("credits.json");
    cmd.insert_resource(CreditRes(credit_handle));
    state.set(GameState::Loading);
}

fn show_credits(mut _cmd: Commands, credits: Res<CreditRes>, credit_assets: Res<Assets<Credits>>, mut _state: ResMut<State<GameState>>) {
    // if let Some(cre) = credit_assets.get(&credits.0) {
    //     println!("{:?}", cre);
    // }
}
