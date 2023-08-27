use bevy::{prelude::*, window::WindowResolution};

const PADDLE_SIZE: Vec2 = Vec2::new(6., 46.);
const BALL_SIZE: Vec2 = Vec2::new(8., 8.);

const WINDOW_TITLE: &str = "Pong";
const WINDOW_WIDTH: f32 = 1138.;
const WINDOW_HEIGHT: f32 = 720.;

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
        .run();
}
