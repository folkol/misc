use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::{close_on_esc, WindowResolution};

use components::*;

mod components;

const BALL_RADIUS: f32 = 5.;
const GRAVITY: f32 = 100.;
const HEIGHT: f32 = 500.;
const MAX_PADDLE_VELOCITY: f32 = 300.;
const WIDTH: f32 = 500.;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum AppState {
    #[default]
    Playing,
    GameWon,
    GameLost,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                title: "BREAKTHROUGH".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_systems(Startup, setup_world)
        .add_systems(OnEnter(AppState::GameWon), you_won)
        .add_systems(OnEnter(AppState::GameLost), game_over)
        .add_systems(Update, (close_on_esc, control_paddle))
        .add_systems(
            Update,
            (
                gravity,
                update_positions,
                bounce,
                brick_collision,
                check_game_over,
            )
                .run_if(in_state(AppState::Playing)),
        )
        .run();
}

const PADDLE_WIDTH: f32 = 65.;

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // bricks
    commands.spawn(Camera2dBundle::default());
    for n in 0..10 {
        for row in 0..4 {
            commands.spawn(BrickBundle::new(n, row));
        }
    }
    // ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::SILVER)),
            transform: Transform {
                translation: Vec3::new(-WIDTH / 2.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        Ball {},
        Velocity(Vec2::X * 150.),
    ));
    // paddle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::from((PADDLE_WIDTH, 5.))),
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: Vec3::from((0., -HEIGHT / 2.0 + 20., 0.)),
                ..default()
            },
            ..default()
        },
        Paddle {},
        Velocity(Vec2::ZERO),
    ));
}

fn gravity(mut ball_query: Query<&mut Velocity, With<Ball>>, time: Res<Time>) {
    let mut ball = ball_query.get_single_mut().unwrap();
    ball.0.y -= GRAVITY * time.delta().as_secs_f32();
}

fn update_positions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let time_delta = time.delta().as_secs_f32();
    for (mut pos, Velocity(velocity)) in query.iter_mut() {
        pos.translation.y += velocity.y * time_delta;
        pos.translation.x += velocity.x * time_delta;
    }
}

fn bounce(
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    time: Res<Time>,
) {
    let time_delta = time.delta().as_secs_f32();
    let (mut ball_transform, mut ball_velocity) = ball_query.get_single_mut().unwrap();
    let paddle_transform = paddle_query.get_single().unwrap();
    let Vec3 {
        y: ball_y,
        x: ball_x,
        ..
    } = ball_transform.translation;
    let paddle_surface = paddle_transform.translation.y + 2.0;
    if ball_y - BALL_RADIUS <= paddle_surface
        && ball_y + BALL_RADIUS >= paddle_surface
        && (ball_x - paddle_transform.translation.x).abs() < 30.
    {
        ball_velocity.0.y *= -1.2;
        ball_transform.translation.y += ball_velocity.0.y * time_delta;
        let off_center = paddle_transform.translation.x - ball_transform.translation.x;
        let off_center = off_center / (PADDLE_WIDTH / 2.0);
        let adjustment = off_center.signum() * off_center * off_center * 100.;
        ball_velocity.0.x -= adjustment;
    } else if ball_x <= -WIDTH / 2.0 || ball_x >= WIDTH / 2.0 {
        ball_velocity.0.x *= -0.9;
        ball_transform.translation.x += ball_velocity.0.x * time_delta;
    };
}

fn control_paddle(
    input: Res<Input<KeyCode>>,
    mut paddle_query: Query<&mut Velocity, With<Paddle>>,
) {
    let mut paddle = paddle_query.single_mut();
    if input.pressed(KeyCode::Left) {
        paddle.0.x = -MAX_PADDLE_VELOCITY;
    } else if input.pressed(KeyCode::Right) {
        paddle.0.x = MAX_PADDLE_VELOCITY;
    } else {
        paddle.0.x = 0.;
    }
}

fn check_game_over(
    ball_query: Query<&Transform, With<Ball>>,
    mut state: ResMut<NextState<AppState>>,
) {
    let Vec3 { y: ball_y, .. } = ball_query.single().translation;
    if ball_y > WIDTH / 2.0 {
        state.set(AppState::GameWon);
    } else if ball_y < -250. {
        state.set(AppState::GameLost);
    }
}

fn brick_collision(
    mut commands: Commands,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    brick_query: Query<(Entity, &mut Transform), (With<Brick>, Without<Ball>)>,
) {
    let (ball_transform, mut ball_velocity) = ball_query.single_mut();
    let Vec3 {
        x: ball_x,
        y: ball_y,
        ..
    } = ball_transform.translation;
    let mut did_hit = false;
    for (brick_entity, brick) in brick_query.iter() {
        let Vec3 { x, y, .. } = brick.translation;
        if (ball_y - y).abs() < 10. && (ball_x - x).abs() < 45. {
            did_hit = true;
            commands.entity(brick_entity).despawn();
        }
    }
    if did_hit {
        ball_velocity.0.y *= -0.7;
    }
}

fn you_won(commands: Commands) {
    full_screen_text(commands, "YOU WON! :D", Color::rgba(0., 0.7, 0.2, 0.8));
}

fn game_over(commands: Commands) {
    full_screen_text(commands, "GAME OVER :(", Color::rgba(1., 0., 0., 0.8));
}

fn full_screen_text(mut commands: Commands, text: &str, color: Color) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor::from(color),
            ..default()
        })
        .with_children(|breeder| {
            breeder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ));
        });
}
