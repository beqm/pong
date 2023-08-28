use crate::ball::components::*;
use bevy::prelude::*;
use rand::Rng;

pub const BALL_SIZE: Vec2 = Vec2::new(8., 8.);
pub const BALL_SPEED: f32 = 600.;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

pub fn spawn_ball(mut commands: Commands) {
    let rng = rand::thread_rng().gen_bool(1. / 2.);
    let mut direction: f32 = -1.0;
    if rng {
        direction = 1.0;
    }
    let angle = rand::thread_rng().gen_range(-2.0..=2.0);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2., 0.),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(BALL_SIZE),
                ..default()
            },
            ..default()
        },
        Ball,
        Velocity(Vec2::new(direction, angle).normalize() * BALL_SPEED),
    ));
}
