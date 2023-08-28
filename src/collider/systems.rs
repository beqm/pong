use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::ball::components::*;
use crate::ball::systems::spawn_ball;
use crate::collider::components::*;

const BALL_SIZE: Vec2 = Vec2::new(8., 8.);
const PADDLE_SIZE: Vec2 = Vec2::new(6., 46.);

pub fn check_collision(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((ball_entity, mut ball_velocity, ball_transform)) = ball_query.get_single_mut() {
        for transform in &collider_query {
            let paddle_collision = collide(
                ball_transform.translation,
                BALL_SIZE,
                transform.translation,
                PADDLE_SIZE,
            );

            if paddle_collision.is_some() {
                // ball_velocity.y *= -1.0;
                ball_velocity.x *= -1.0;
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/impact.ogg"),
                    ..default()
                });
            }
        }

        let top_collision = collide(
            ball_transform.translation,
            BALL_SIZE,
            Vec3::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT, 0.),
            Vec2::new(WINDOW_WIDTH, BALL_SIZE.y),
        );

        let bottom_collision = collide(
            ball_transform.translation,
            BALL_SIZE,
            Vec3::new(WINDOW_WIDTH / 2., 0., 0.),
            Vec2::new(WINDOW_WIDTH, BALL_SIZE.y),
        );

        if top_collision.is_some() || bottom_collision.is_some() {
            ball_velocity.y = -ball_velocity.y;
        }

        let right_collision = collide(
            ball_transform.translation,
            BALL_SIZE,
            Vec3::new(WINDOW_WIDTH, WINDOW_HEIGHT / 2., 0.),
            Vec2::new(BALL_SIZE.x, WINDOW_HEIGHT),
        );

        let left_collision = collide(
            ball_transform.translation,
            BALL_SIZE,
            Vec3::new(0., WINDOW_HEIGHT / 2., 0.),
            Vec2::new(BALL_SIZE.x, WINDOW_HEIGHT),
        );

        if right_collision.is_some() || left_collision.is_some() {
            commands.entity(ball_entity).despawn();
            spawn_ball(commands);
        }
    }
}
