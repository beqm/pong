use crate::collider::components::Collider;
use crate::player::components::*;
use crate::WINDOW_HEIGHT;
use bevy::prelude::*;

const PADDLE_SIZE: Vec2 = Vec2::new(6., 46.);
const PADDLE_SPEED: f32 = 500.;

pub fn spawn_paddle(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(25., WINDOW_HEIGHT / 2., 0.),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));
}

pub fn paddle_controls(
    mut query: Query<&mut Transform, &Paddle>,
    input: Res<Input<KeyCode>>,
    time: Res<FixedTime>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut distance = 0.0;

        if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            distance += 1.0;
        }

        if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            distance -= 1.0;
        }

        let new_position =
            transform.translation.y + distance * PADDLE_SPEED * time.period.as_secs_f32();

        let upper_limit = WINDOW_HEIGHT - PADDLE_SIZE.y / 2.;
        let lower_limit = 0.0 + PADDLE_SIZE.y / 2.;

        transform.translation.y = new_position.clamp(lower_limit, upper_limit);
    }
}
