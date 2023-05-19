use bevy::prelude::*;

use crate::state::{GameState, MainMenuState};
use crate::ecs_common::components::Fonts;
use crate::GAME_TITLE;

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_system(init_main_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(show_main_menu.in_schedule(OnEnter(MainMenuState::Show)));
    }
}

fn init_main_menu(mut mms: ResMut<NextState<MainMenuState>>) {
    mms.set(MainMenuState::Show);
}

fn show_main_menu(mut cmd: Commands, fonts: Res<Fonts>) {
    info!("show main menu called!");
    cmd.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        // Show Game Title
        // TODO: use the `big text` sprites from ui asset
        parent.spawn(
            TextBundle::from_section(
                GAME_TITLE,
                TextStyle {
                    font: fonts.ui.clone(),
                    font_size: 120.0,
                    color: Color::ORANGE,
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(50.0)),
                ..default()
            }),
        );

        // A panel with back ground image contains buttons
        // TODO: use parent.spawn(NodeBundle)
    });
}
