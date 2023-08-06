use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::{close_on_esc, Cursor, PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    cursor: Cursor {
                        visible: false,
                        ..default()
                    },
                    // mode: WindowMode::Fullscreen,
                    resolution: (800.0, 600.0).into(),
                    title: "Shoot em".to_string(),
                    ..default()
                }),
                ..default()
            }), // .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update,
                close_on_esc,
                animate_sprite,
                move_zombie,
                shoot,
                move_bullets,
            ),
        )
        .run();
}

#[derive(Component)]
struct Aim;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Zombie;

#[derive(Component, Debug)]
struct Bullet(Vec2);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("crosshairs.png".to_owned());
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 64, 64, None, None);
    let handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 3,
                ..default()
            },
            texture_atlas: handle,
            ..default()
        },
        Aim,
    ));

    let texture_handle = asset_server.load("zombie_tilesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(80.0, 120.0), 9, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 1 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            // transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Zombie,
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

const SPEED: f32 = 200.0;

fn move_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (entity, mut transform, bullet) in query.iter_mut() {
        transform.translation.x += bullet.0.x;
        transform.translation.y += bullet.0.y;
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds() * 10.0);
        if transform.translation.distance(Vec3::ZERO) > 500.0 {
            commands.entity(entity).despawn()
        }
    }
}

fn shoot(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    query: Query<&mut Transform, With<Zombie>>,
    q_cursor: Query<&Transform, (With<Aim>, Without<Zombie>)>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let cursor = q_cursor.single();
    let zombie = query.single();
    if input.pressed(KeyCode::Space) {
        let texture_handle = asset_server.load("zombie_tilesheet.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(80.0, 120.0), 9, 3, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 1 };
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform {
                    translation: zombie.translation,
                    // rotation: Quat::from_rotation_x(1.0),
                    scale: Vec3::new(0.3, 0.3, 1.0),
                    ..default()
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Bullet((cursor.translation - zombie.translation).normalize().xy() * 3.0),
        ));
    }
}

fn move_zombie(
    mut query: Query<&mut Transform, With<Zombie>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut zombie = query.single_mut();

    if input.pressed(KeyCode::W) {
        zombie.translation.y += SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::A) {
        zombie.translation.x -= SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::S) {
        zombie.translation.y -= SPEED * time.delta_seconds();
    }
    if input.pressed(KeyCode::D) {
        zombie.translation.x += SPEED * time.delta_seconds();
    }
}

fn update(
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_cursor: Query<&mut Transform, (With<Aim>, Without<PrimaryWindow>)>,
) {
    let mut cursor = q_cursor.single_mut();
    let window = q_window.single();
    if let Some(pos) = window.cursor_position() {
        cursor.translation = Vec3::new(
            pos.x - window.width() / 2.0,
            -pos.y + window.height() / 2.0,
            1.0,
        );
    }
}
