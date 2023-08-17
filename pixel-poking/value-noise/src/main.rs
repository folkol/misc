use std::f32::consts::PI;
use std::time::Duration;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::{Rng, thread_rng};
use rand::distributions::uniform::Uniform;

const WIDTH: usize = 1600;
const HEIGHT: usize = 1200;
const NUM_OCTAVES: usize = 10;

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
    let mut ys: Vec<_> = vec![0; WIDTH];
    for octave in 1..NUM_OCTAVES {
        let pitch = WIDTH / 2usize.pow(octave as u32);
        let range = Uniform::from(0..HEIGHT / 2usize.pow(octave as u32));
        let lattice: Vec<_> = thread_rng()
            .sample_iter(&range)
            .take(2 * (WIDTH / pitch))
            .collect();
        println!("{pitch:?}");
        for (x, point) in ys.iter_mut().enumerate() {
            let a = lattice[x / pitch];
            let b = lattice[1 + x / pitch];
            let t = (x % pitch) as f32 / pitch as f32;
            let y = cosine_interpolation(a, b, t);
            if x == 10 {
                *point += y;
            } else {
                *point += y;
            }
        }
    }
    let min = *ys.iter().min().unwrap();
    let max = *ys.iter().max().unwrap();

    window.limit_update_rate(Some(Duration::from_millis(1000 / 60)));
    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        for (x, y) in ys.iter().enumerate() {
            plot(
                &mut buffer,
                x,
                map(min, max, HEIGHT / 4, 3 * HEIGHT / 4, *y),
            );
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn map(from_lo: usize, from_hi: usize, to_lo: usize, to_hi: usize, x: usize) -> usize {
    let t = (x - from_lo) as f32 / (from_hi - from_lo) as f32;
    (to_lo as f32 * (1.0 - t) + to_hi as f32 * t) as usize
}
fn cosine_interpolation(lo: usize, hi: usize, t1: f32) -> usize {
    let t = ((t1 * PI).cos() + 1.0) / 2.0;
    (lo as f32 * t + hi as f32 * (1.0 - t)) as usize
}

fn plot(buffer: &mut Vec<u32>, x: usize, y: usize) {
    let pos = y * WIDTH + x;
    if pos < WIDTH * HEIGHT {
        buffer[pos] = to_0rgb(255, 255, 255);
    }
}

fn to_0rgb(r: u32, g: u32, b: u32) -> u32 {
    r << 16 | g << 8 | b
}
