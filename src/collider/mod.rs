use crate::collider::systems::check_collision;
use bevy::prelude::*;

pub mod components;
pub mod systems;
pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, check_collision);
    }
}
