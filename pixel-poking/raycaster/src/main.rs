use std::f32::consts::PI;
use std::time::{Duration, SystemTime};

use minifb::{Key, KeyRepeat, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const NUM_SLICES: usize = 10;
const WORLD: [[u8; 10]; 10] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

fn main() {
    let mut window = Window::new(
        "Kittenstein",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    window.limit_update_rate(Some(Duration::from_millis(1000 / 60)));
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let (mut x, mut y) = (4., 4.);
    let mut angle: f32 = 0.;

    let begin = SystemTime::now();
    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        let mut is_running = false;
        if window.is_key_down(Key::Right) {
            angle += 0.05;
        }
        if window.is_key_down(Key::Left) {
            angle -= 0.05;
        }
        if window.is_key_down(Key::Up) {
            let dx = angle.cos();
            let dy = angle.sin();
            let next_dx_rounded = (x + dx).round() as usize;
            let next_dy_rounded = (y + dy).round() as usize;
            if WORLD[next_dy_rounded][next_dx_rounded] == 0 {
                x += dx / 20.;
                y += dy / 20.;
            }
            is_running = true;
        }
        if window.is_key_down(Key::Down) {
            let dx = angle.cos();
            let dy = angle.sin();
            x -= dx / 20.;
            y -= dy / 20.;
        }

        let elapsed = begin.elapsed().unwrap().as_secs_f32();
        let bob = if is_running {
            (elapsed * 20.).sin() * HEIGHT as f32 / 100.
        } else {
            0.
        };
        let scene = cast_rays(x, y, angle);
        let step_size = WIDTH / NUM_SLICES;
        buffer.iter_mut().for_each(|x| *x = 0);
        for row in 0..NUM_SLICES - 1 {
            let left = row * step_size;
            let right = (row + 1) * step_size;
            draw_quad(&mut buffer, bob, scene[row], scene[row + 1], left, right);
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn cast_rays(x: f32, y: f32, angle: f32) -> [f32; 10] {
    let mut scene = [0.; NUM_SLICES];
    let start = angle - PI / 8.;
    let step = (PI / 4.) / NUM_SLICES as f32;
    for slice in 0..NUM_SLICES {
        let cur_angle = start + slice as f32 * step;
        let dx = cur_angle.cos() / 100.;
        let dy = cur_angle.sin() / 100.;
        let mut ray_x = x;
        let mut ray_y = y;
        let distance = loop {
            let sample_x = ray_x.round() as isize;
            let sample_y = ray_y.round() as isize;

            if sample_x < 0
                || sample_x >= 10
                || sample_y < 0
                || sample_y >= 10
                || WORLD[sample_y as usize][sample_x as usize] == 1
            {
                break (ray_x - x).powf(2.) + (ray_y - y).powf(2.);
            }
            ray_x += dx;
            ray_y += dy;
        };
        scene[slice] = distance;
    }
    scene
}

fn draw_quad(
    buffer: &mut Vec<u32>,
    bob: f32,
    distance_left: f32,
    distance_right: f32,
    left: usize,
    right: usize,
) {
    let height_left = (HEIGHT / 2) as f32 / distance_left;
    let height_right = (HEIGHT / 2) as f32 / distance_right;
    let step_size = right - left;
    for col in left..right {
        let height =
            height_left + ((col - left) as f32 / step_size as f32) * (height_right - height_left);
        let top = (HEIGHT as f32 / 2. - height + bob).clamp(0., HEIGHT as f32) as usize;
        let bottom = (HEIGHT as f32 / 2. + height + bob).clamp(0., HEIGHT as f32) as usize;
        let distance = distance_left + (distance_right - distance_left) / step_size as f32;

        for line in top..bottom {
            let i = line * WIDTH + col;
            let c = 20 + (235. / distance) as u32;
            buffer[i] = c << 16 | c << 8 | c;
        }
    }
}
