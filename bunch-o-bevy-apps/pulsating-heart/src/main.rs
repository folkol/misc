use std::f32::consts::PI;

use bevy::window::WindowMode::Fullscreen;
use bevy::{prelude::*, window::WindowTheme};

const HEART: Color = Color::rgb(176. / 255., 30. / 255., 40. / 255.);
const SOUL: Color = Color::rgb(15. / 255., 12. / 255., 23. / 255.);

fn main() {
    App::new()
        .insert_resource(ClearColor(SOUL))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: Fullscreen,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, bevy::window::close_on_esc))
        .run();
}

const R: f32 = 100.;

fn update(mut gizmos: Gizmos, time: Res<Time>, mut q: Query<&mut OrthographicProjection>) {
    let scale = (time.elapsed_seconds() * 2.).sin().powf(100.) / 10.;
    q.single_mut().scale = 1. + scale;

    // background gradient? (glow in the middle?)
    gizmos
        .arc_2d(Vec2 { x: -R, y: R / 2. }, 0., PI, R, HEART)
        .segments(200);
    let segment = Bezier::new([[
        Vec2 {
            x: 2. * R,
            y: R / 2.,
        },
        Vec2 {
            x: 2. * R,
            y: -R / 2.,
        },
        Vec2 { x: R, y: -R },
        Vec2 { x: 0., y: -2. * R },
    ]])
    .to_curve();
    gizmos.linestrip_2d(segment.iter_positions(50), HEART);
    gizmos.linestrip_2d(
        segment
            .iter_positions(50)
            .map(|Vec2 { x, y }| Vec2 { x: -x, y }),
        HEART,
    );
    gizmos
        .arc_2d(Vec2 { x: R, y: R / 2. }, 0., PI, R, HEART)
        .segments(200);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
