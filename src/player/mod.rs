use self::systems::*;
use bevy::prelude::*;

pub mod components;
pub mod systems;

use crate::ball::systems::apply_velocity;
use crate::collider::systems::check_collision;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_paddle).add_systems(
            FixedUpdate,
            paddle_controls
                .before(check_collision)
                .after(apply_velocity),
        );
    }
}
