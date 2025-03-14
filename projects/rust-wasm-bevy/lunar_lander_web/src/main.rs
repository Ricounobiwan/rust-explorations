use bevy::prelude::*;
use bevy::math::primitives::{RegularPolygon, Rectangle};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle, ColorMaterial};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0))) // Black background
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>, // ✅ ColorMaterial for 2D
) {
    commands.spawn(Camera2dBundle::default());

    // Spawn the moon surface as a gray rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(600.0, 50.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -250.0, 0.0),
        ..Default::default()
    });

    // 🛸 Lander Parts
    let lander_color = materials.add(ColorMaterial::from(Color::WHITE));

    // Octagonal main body
    let octagon = Mesh::from(RegularPolygon::new(15.0, 8));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(octagon)),
            material: lander_color.clone(),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..Default::default()
        },
    ));

    // Landing legs
    let leg = Mesh::from(Rectangle::new(10.0, 2.0)); // ✅ Fix: Correct API usage
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(leg.clone())),
            material: lander_color.clone(),
            transform: Transform::from_xyz(-10.0, 85.0, 0.0).with_rotation(Quat::from_rotation_z(0.4)),
            ..Default::default()
        },
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(leg.clone())),
            material: lander_color.clone(),
            transform: Transform::from_xyz(10.0, 85.0, 0.0).with_rotation(Quat::from_rotation_z(-0.4)),
            ..Default::default()
        },
    ));

    // Lower platform
    let platform = Mesh::from(Rectangle::new(20.0, 2.0)); // ✅ Fix: Correct API usage
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(platform.clone())),
            material: lander_color.clone(),
            transform: Transform::from_xyz(0.0, 80.0, 0.0),
            ..Default::default()
        },
    ));
}
