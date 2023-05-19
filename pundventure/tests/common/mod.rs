use bevy::prelude::App;

/// Setup an event loop for bevy app.
/// 
/// First function in the arguments is for setting up resource and/or systems,
/// Second function will consume application, extracting result and do assertions.
pub(crate) fn setup<F, N>(f: F, a: N)
where
    F: FnOnce(&mut App),
    N: FnOnce(&App),
{
    let mut app = App::new();
    f(&mut app);
    app.update();
    a(&app);
}
