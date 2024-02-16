use bevy::prelude::*;

#[derive(Component)]
struct Ship;

fn spawn_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/boat.png"),
            transform: Transform::from_translation(Vec3::new(20., 20., 2.)),
            ..default()
        },
        Ship,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_ship)
        .run();
}
