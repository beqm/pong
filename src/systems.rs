use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;
use bevy::prelude::*;

#[derive(Component)]
pub struct Collider;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2., 0.),
        ..default()
    });
}
