use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::*;
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};

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
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
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
        material: materials.add(MyMaterial {}),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, Clone, Reflect)]
#[uuid = "661C6A13-B89F-4699-AA11-AF15E20F725A"]
struct MyMaterial {}

impl Material2d for MyMaterial {
    fn fragment_shader() -> ShaderRef {
        "fragment_shader.wgsl".into()
    }
}
