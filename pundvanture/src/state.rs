use bevy::prelude::States;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub(crate) enum GameState {
    #[default]
    Splash,
    Loading,
    MainMenu,
    ShowCredits,
    Game,
    Paused,
}
