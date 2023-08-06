use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use bevy::window::PrimaryWindow;
use rand::{thread_rng, Rng};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // DefaultPlugins.set(LogPlugin {
            // filter: "LOGGER".to_string(),
            // level: Level::DEBUG,
            // }),
            Material2dPlugin::<MyMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Voronoi(Vec<Vec2>);

// check different metrics, manhattan etc
// https://www.willusher.io/graphics/2023/04/11/0-to-gltf-bind-groups
// https://bevyengine.org/examples-webgpu/Shaders/shader-material/
// "A shader and a material that uses it."
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<MyMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_window.single();
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: materials.add(MyMaterial::default()),
        transform: Transform::from_scale(Vec3::ONE * window.width()),
        ..default()
    });
}

#[derive(TypePath, TypeUuid, AsBindGroup, Clone)]
#[uuid = "B33E2FA6-9C82-4B0E-B012-FB458953B03F"]
struct MyMaterial {
    #[uniform(0)]
    points: [Vec4; 100],
    #[uniform(1)]
    colors: [Vec4; 100],
}

impl MyMaterial {
    fn default() -> MyMaterial {
        let mut rng = thread_rng();
        let mut points = [Vec4::ONE; 100];
        for point in &mut points {
            point.x = rng.gen();
            point.y = rng.gen();
            point.z = rng.gen();
            point.w = rng.gen();
        }
        let mut colors = [Vec4::ONE; 100];
        for color in &mut colors {
            color.x = rng.gen();
            color.y = rng.gen();
            color.z = rng.gen();
            color.w = rng.gen();
        }

        MyMaterial { points, colors }
    }
}

impl Material2d for MyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shader.wgsl".into()
    }
}
