use std::f32::consts::PI;
use std::time::Duration;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::{Rng, thread_rng};
use rand::distributions::uniform::Uniform;
use rand::prelude::*;

const WIDTH: usize = 1600;
const HEIGHT: usize = 1200;
const PITCH: usize = WIDTH / 10;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Noise",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let range = Uniform::from(0..HEIGHT / 2);
    let lattice: Vec<_> = thread_rng()
        .sample_iter(&range)
        .take(1 + WIDTH / PITCH)
        .collect();

    let yx: Vec<_> = (0..WIDTH)
        .map(|x| {
            let a = lattice[x / PITCH];
            let b = lattice[1 + x / PITCH];
            let t = (x % PITCH) as f32 / PITCH as f32;
            let y = cosine_interpolation(a, b, t);
            y
        })
        .collect();

    window.limit_update_rate(Some(Duration::from_millis(1000 / 60)));
    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        for (x, y) in yx.iter().enumerate() {
            plot(&mut buffer, x, y + HEIGHT / 4);
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn lerp(lo: usize, hi: usize, t: f32) -> usize {
    (lo as f32 * t + hi as f32 * (1.0 - t)) as usize
}
fn cosine_interpolation(lo: usize, hi: usize, t1: f32) -> usize {
    let t = ((t1 * PI).cos() + 1.0) / 2.0;
    let i = (lo as f32 * t + hi as f32 * (1.0 - t)) as usize;
    println!("{lo} {hi} {t1} {t} {i}");
    i
}

fn plot(buffer: &mut Vec<u32>, x: usize, y: usize) {
    let pos = y * WIDTH + x;
    if pos < WIDTH * HEIGHT {
        buffer[pos] = to_0rgb(255, 255, 255);
    }
}

fn paint(buffer: &mut Vec<u32>) {
    buffer[2700] = to_0rgb(255, 0, 0);
}

fn to_0rgb(r: u32, g: u32, b: u32) -> u32 {
    r << 16 | g << 8 | b
}
