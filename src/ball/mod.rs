use crate::collider::systems::check_collision;
use bevy::prelude::*;
use systems::*;

pub mod components;
pub mod systems;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(FixedUpdate, apply_velocity.before(check_collision));
    }
}
