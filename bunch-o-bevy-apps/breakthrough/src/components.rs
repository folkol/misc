use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Bundle)]
pub struct BrickBundle {
    pub sprite_bundle: SpriteBundle,
    pub brick: Brick,
}

impl BrickBundle {
    pub fn new(n: i32, row: i32) -> Self {
        BrickBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::from((45., 10.))),
                    color: Color::WHITE,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::from((-225. + n as f32 * 50., 242. - row as f32 * 15., 0.)),
                    ..default()
                },
                ..default()
            },
            brick: Brick,
        }
    }
}
