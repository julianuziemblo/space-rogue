use bevy::{prelude::*, window::WindowResolution};

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d));
}

#[derive(Component)]
struct Player {}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("placeholder.png")),
        Player {},
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800., 600.).with_scale_factor_override(4.),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .run();
}
