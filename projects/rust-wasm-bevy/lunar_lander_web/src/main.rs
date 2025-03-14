use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0))) // Black background
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera (needed to see objects)
    commands.spawn(Camera2dBundle::default());

    // Spawn the moon surface as a gray rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.3, 0.3, 0.3), // Gray color for the moon
            custom_size: Some(Vec2::new(600.0, 50.0)), // Wide rectangle
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -250.0, 0.0), // Position at the bottom
        ..Default::default()
    });
}
