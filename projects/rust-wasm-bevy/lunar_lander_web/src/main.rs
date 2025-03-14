use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Load default Bevy plugins (window, rendering, input, etc.)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());
}