use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Component)]
pub struct Ship {}
