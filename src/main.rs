use crate::systems::spawn_camera;
use ball::BallPlugin;
use bevy::{prelude::*, window::WindowResolution};
use collider::ColliderPlugin;
use player::PlayerPlugin;

mod ball;
mod collider;
mod player;
mod systems;

const WINDOW_TITLE: &str = "Pong";
const WINDOW_WIDTH: f32 = 1138.;
const WINDOW_HEIGHT: f32 = 720.;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: String::from(WINDOW_TITLE),
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            ..Default::default()
        }),
        ..Default::default()
    };

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_systems(Startup, spawn_camera)
        .add_plugins(ColliderPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
