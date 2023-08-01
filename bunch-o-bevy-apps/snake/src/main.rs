use bevy::prelude::*;
use bevy::window::ExitCondition;

const NUM_SEGMENTS: usize = 100;

#[derive(Resource)]
struct GameState {
    direction: Direction,
    alive: bool,
}

#[derive(Component)]
struct Head;

#[derive(Component)]
struct Tail {
    segments: Vec<Vec3>,
}

#[derive(Component)]
struct Segment;

#[derive(Bundle)]
struct Snake {
    head: SpriteBundle,
    kind: Head,
    tail: Tail,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Snake {
    fn new() -> Self {
        Snake {
            kind: Head,
            tail: Tail {
                segments: vec![Vec3::default(); NUM_SEGMENTS],
            },
            head: SpriteBundle {
                transform: Transform::default(),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(9., 9.)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn main() {
    App::new()
        .insert_resource(FixedTime::new_from_secs(0.1))
        .insert_resource(GameState {
            direction: Direction::Right,
            alive: true,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (500., 500.).into(),
                title: "Snake".to_owned(),
                ..default()
            }),
            exit_condition: ExitCondition::OnAllClosed,
            close_when_requested: true,
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (input, bevy::window::close_on_esc))
        .add_systems(FixedUpdate, update)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // "Bevy also considers arbitrary tuples of components as bundles"
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Snake::new());
    for _ in 0..NUM_SEGMENTS {
        commands.spawn((
            Segment {},
            SpriteBundle {
                sprite: Sprite {
                    color: Color::ANTIQUE_WHITE,
                    custom_size: Some(Vec2::new(9., 9.)),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn input(keyboard_input: Res<Input<KeyCode>>, mut input_state: ResMut<GameState>) {
    if keyboard_input.pressed(KeyCode::Left) && !matches!(input_state.direction, Direction::Right) {
        input_state.direction = Direction::Left;
    }

    if keyboard_input.pressed(KeyCode::Right) && !matches!(input_state.direction, Direction::Left) {
        input_state.direction = Direction::Right;
    }

    if keyboard_input.pressed(KeyCode::Up) && !matches!(input_state.direction, Direction::Down) {
        input_state.direction = Direction::Up;
    }

    if keyboard_input.pressed(KeyCode::Down) && !matches!(input_state.direction, Direction::Up) {
        input_state.direction = Direction::Down;
    }
}

fn update(
    mut game_state: ResMut<GameState>,
    mut snake: Query<(&mut Transform, &mut Tail), With<Head>>,
    mut children: Query<&mut Transform, (Without<Head>, With<Segment>)>,
) {
    if !game_state.alive {
        return;
    }
    let (mut snake_pos, mut tail) = snake.single_mut();
    let prev_head_pos = snake_pos.translation.clone();
    match game_state.direction {
        Direction::Up => {
            snake_pos.translation.y += 10.;
        }
        Direction::Right => {
            snake_pos.translation.x += 10.;
        }
        Direction::Down => {
            snake_pos.translation.y -= 10.;
        }
        Direction::Left => {
            snake_pos.translation.x -= 10.;
        }
    }
    let Vec3 { x, y, z } = snake_pos.translation;
    if x < -250. || x > 250. || y < -250. || y > 250. || tail.segments.contains(&Vec3 { x, y, z }) {
        println!("DED AT {x} {y} {z}");
        game_state.alive = false;
    }
    tail.segments.insert(0, prev_head_pos);
    tail.segments.pop().unwrap();

    for (mut sprite, segment) in children.iter_mut().zip(tail.segments.iter()) {
        sprite.translation.x = segment.x;
        sprite.translation.y = segment.y;
        sprite.translation.z = segment.z;
    }
}
