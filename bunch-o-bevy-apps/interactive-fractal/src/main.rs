use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::*;
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Resource)]
struct Foo(Vec4);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (500., 500.).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<MyMaterial>::default(),
        ))
        .insert_resource(Foo(Vec4::new(-1.0, 0.0, 2.0, 2.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, bevy::window::close_on_esc))
        .run();
}

fn update(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut foo: ResMut<Foo>,
    mut materials: ResMut<Assets<MyMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<Entity, With<Mesh2dHandle>>,
) {
    if input.pressed(KeyCode::Up) {
        foo.0.z *= 1.0 - time.delta().as_secs_f32();
        foo.0.w *= 1.0 - time.delta().as_secs_f32();
    }
    if input.pressed(KeyCode::Down) {
        foo.0.z *= 1.0 + time.delta().as_secs_f32();
        foo.0.w *= 1.0 + time.delta().as_secs_f32();
    }
    if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
        foo.0.x += foo.0.z * time.delta().as_secs_f32();
    }
    if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
        foo.0.x -= foo.0.z * time.delta().as_secs_f32();
    }
    if input.pressed(KeyCode::W) {
        foo.0.y -= foo.0.w * time.delta().as_secs_f32();
    }
    if input.pressed(KeyCode::S) {
        foo.0.y += foo.0.w * time.delta().as_secs_f32();
    }

    let quad = query.single_mut();
    commands.entity(quad).despawn();
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(-500., 500.)).into())
            .into(),
        material: materials.add(MyMaterial { foo: foo.0 }),
        ..default()
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(-500., 500.)).into())
            .into(),
        material: materials.add(MyMaterial {
            foo: Vec4::new(-2.0, -1.0, 3.0, 2.0),
        }),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, Clone, Reflect)]
#[uuid = "B419E240-5411-4EA2-A0B0-08D7A4046D74"]
struct MyMaterial {
    #[uniform(0)]
    foo: Vec4,
}

impl Material2d for MyMaterial {
    fn fragment_shader() -> ShaderRef {
        "fragment_shader.wgsl".into()
    }
}
