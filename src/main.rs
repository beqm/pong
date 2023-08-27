use bevy::{prelude::*, window::WindowResolution};

const WINDOW_TITLE: &str = "Pong";
const WINDOW_WIDTH: f32 = 1138.;
const WINDOW_HEIGHT: f32 = 720.;

const PADDLE_SIZE: Vec2 = Vec2::new(6., 46.);
const PADDLE_SPEED: f32 = 500.;

const BALL_SIZE: Vec2 = Vec2::new(8., 8.);

#[derive(Component)]
pub struct Paddle {}

#[derive(Component)]
pub struct Ball {}

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
        Paddle {},
    ));

    // Ball
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
        Ball {},
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

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from(WINDOW_TITLE),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, paddle_controls)
        .run();
}
