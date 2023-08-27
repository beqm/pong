use bevy::{prelude::*, sprite::collide_aabb::collide, window::WindowResolution};
use rand::Rng;

const WINDOW_TITLE: &str = "Pong";
const WINDOW_WIDTH: f32 = 1138.;
const WINDOW_HEIGHT: f32 = 720.;

const PADDLE_SIZE: Vec2 = Vec2::new(6., 46.);
const PADDLE_SPEED: f32 = 500.;

const BALL_SIZE: Vec2 = Vec2::new(8., 8.);
const BALL_SPEED: f32 = 600.;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

impl Ball {
    pub fn spawn(mut commands: Commands) {
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
}

#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

pub fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2., 0.),
        ..default()
    });

    // Paddle
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

    Ball::spawn(commands);
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

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn check_collision(
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
                ball_velocity.0.x = -ball_velocity.0.x;
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
            ball_velocity.0.y = -ball_velocity.0.y;
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
            Ball::spawn(commands);
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from(WINDOW_TITLE),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                check_collision,
                apply_velocity.before(check_collision),
                paddle_controls
                    .before(check_collision)
                    .after(apply_velocity),
            ),
        )
        .run();
}
