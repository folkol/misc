use std::time::Duration;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

const BALL_SIZE: f32 = 50.;
const BALL_SPEED: f32 = 0.8;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Metaballs",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    window.limit_update_rate(Some(Duration::from_millis(1000 / 60)));
    let mut balls = vec![
        Ball {
            x: WIDTH as f32 / 2.0,
            y: HEIGHT as f32 / 2.0,
            dx: BALL_SPEED,
            dy: BALL_SPEED,
        },
        Ball {
            x: WIDTH as f32 / 2.0,
            y: HEIGHT as f32 / 2.0,
            dx: BALL_SPEED,
            dy: -BALL_SPEED / 2.0,
        },
        Ball {
            x: WIDTH as f32 / 2.0,
            y: HEIGHT as f32 / 2.0,
            dx: -BALL_SPEED / 3.0,
            dy: BALL_SPEED,
        },
        Ball {
            x: WIDTH as f32 / 2.0,
            y: HEIGHT as f32 / 2.0,
            dx: -BALL_SPEED,
            dy: -BALL_SPEED / 4.0,
        },
    ];
    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        for Ball {
            ref mut x,
            ref mut y,
            ref mut dx,
            ref mut dy,
        } in balls.iter_mut()
        {
            if *x < BALL_SIZE || *x > WIDTH as f32 - BALL_SIZE {
                *dx *= -1.;
            }
            if *y < BALL_SIZE || *y > HEIGHT as f32 - BALL_SIZE {
                *dy *= -1.;
            }
            *x += *dx;
            *y += *dy;
        }
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let is_inside = metaball(x, y, &balls);
                let color = if is_inside {
                    to_0rgb(0, 180, 0)
                } else {
                    to_0rgb(0, 0, 20)
                };
                buffer[y * WIDTH + x] = color;
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn to_0rgb(r: u32, g: u32, b: u32) -> u32 {
    r << 16 | g << 8 | b
}

fn metaball(x: usize, y: usize, balls: &Vec<Ball>) -> bool {
    const THRESHOLD: f32 = 0.1;
    let mut sum = 0.0;
    for Ball { x: bx, y: by, .. } in balls {
        let j = (bx - x as f32) * (bx - x as f32) + (by - y as f32) * (by - y as f32);
        sum += 1. / j.sqrt()
    }
    sum > THRESHOLD
}
