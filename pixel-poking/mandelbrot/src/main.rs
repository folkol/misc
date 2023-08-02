use std::time::Duration;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1600;
const HEIGHT: usize = 1200;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Mandlebrot",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    window.limit_update_rate(Some(Duration::from_millis(1000 / 60)));
    draw(&mut buffer);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw(buffer: &mut [u32]) {
    let mut min_a = f64::INFINITY;
    let mut max_a = f64::NEG_INFINITY;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let a = -2. + 3. * x as f64 / (WIDTH as f64);
            let b = -1. + 2. * y as f64 / (HEIGHT as f64);
            let a = a;
            let b = b; // * (HEIGHT as f64 / WIDTH as f64);
            // let a = a * 2.;
            // let b = b * 2.;
            min_a = min_a.min(a);
            max_a = max_a.max(a);

            let mandel = f(a, b) as f64 * 255. / 1000.;
            let i = y * WIDTH + x;
            let k = mandel as u8;
            let r = (k / 3) as u32;
            let g = (k / 2) as u32;
            let b = k as u32;
            let i1 = (r as u32) << 16 | g << 8 | b;
            buffer[i] = i1;
        }
    }
    println!("{min_a} {max_a}");
    for i in buffer.iter_mut() {
        *i += 1
    }
}

fn f(a: f64, b: f64) -> i32 {
    let mut z = (0f64, 0f64);
    for i in 100..=1000 {
        let zprim = (z.0 * z.0 - z.1 * z.1, 2. * z.0 * z.1);
        let zprim = (zprim.0 + a, zprim.1 + b);
        let abs_z = zprim.0 * zprim.0 + zprim.1 * zprim.1;
        if abs_z > 4. {
            // diverged in i steps
            return i;
        }
        z = zprim;
    }
    0
}
