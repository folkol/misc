use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rolling Game".to_string(),
                    ..default()
                }),
                ..default()
            }),
            InputManagerPlugin::<Action>::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0),
            RapierDebugRenderPlugin::default(), // debug wireframes
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
enum Action {
    Move,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-150.0, 0.0, 1.0)),
            texture: asset_server.load("ball_blue_large.png"),
            ..default()
        })
        .insert(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            // input_map: InputMap::new([(KeyCode::Left, Action::Move)]),
            input_map: InputMap::default()
                .insert(VirtualDPad::arrow_keys(), Action::Move)
                .build(),
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(32.0))
        .insert(ExternalForce {
            force: Vec2::ZERO, // starting force
            torque: 0.0,
        })
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 5.0,
        })
        .insert(Restitution::coefficient(1.0));
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(150.0, 200.0, 0.0),
            texture: asset_server.load("block_corner.png"),
            ..default()
        })
        .insert(Collider::triangle(
            Vec2::new(-32.0, 32.0),
            Vec2::new(32.0, -32.0),
            Vec2::new(-32.0, -32.0),
        ))
        .insert(RigidBody::Fixed)
        .insert(Restitution::coefficient(3.0));
}

const MOVE_FORCE: f32 = 1500.0;

fn movement(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>,
) {
    for (action_state, mut external_force) in &mut query {
        let axis_vector = action_state.clamped_axis_pair(Action::Move).unwrap().xy();
        external_force.force = axis_vector * MOVE_FORCE * time.delta_seconds();
    }
}
