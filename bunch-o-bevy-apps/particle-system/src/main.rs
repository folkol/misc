use bevy::prelude::*;
use rand::Rng;

const GRAVITY: f32 = 9.82;

#[derive(Resource)]
struct State {
    num_particles: usize,
    max_particles: usize,
}

#[derive(Component)]
struct ParticleState {
    velocity: Vec2,
    ttl: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(State {
            num_particles: 0,
            max_particles: 1_000_000,
        })
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (
                spawn_particles,
                update_particle_positions,
                kill_particles,
                bevy::window::close_on_esc,
            ),
        )
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update_particle_positions(
    time: Res<Time>,
    mut particles: Query<(&mut Transform, &mut ParticleState)>,
) {
    let delta = time.delta().as_secs_f32();
    for (mut transform, mut particle_state) in &mut particles {
        transform.translation.x += particle_state.velocity.x * delta;
        transform.translation.y += particle_state.velocity.y * delta;
        particle_state.velocity.y -= GRAVITY * delta;
        particle_state.ttl.tick(time.delta());
    }
}
fn kill_particles(
    mut commands: Commands,
    mut state: ResMut<State>,
    mut particles: Query<(Entity, &ParticleState)>,
) {
    for (entity, particle_state) in &mut particles {
        if particle_state.ttl.finished() {
            commands.entity(entity).despawn();
            state.num_particles -= 1;
        }
    }
}

fn spawn_particles(mut commands: Commands, mut state: ResMut<State>) {
    let mut rng = rand::thread_rng();
    if state.num_particles < state.max_particles {
        state.num_particles += 1;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::ANTIQUE_WHITE,
                    custom_size: Some(Vec2::new(3., 3.)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: 0.,
                        y: -250.,
                        z: 0.,
                    },
                    ..default()
                },
                // texture: Default::default(),
                ..default()
            },
            ParticleState {
                velocity: Vec2 {
                    x: rng.gen_range(-10..10) as f32,
                    y: rng.gen_range(50..100) as f32,
                },
                ttl: Timer::from_seconds(20., TimerMode::Once),
            },
        ));
    }
}
