use std::time::Duration;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1600;
const HEIGHT: usize = 1200;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Rainbow",
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

// https://www.fourmilab.ch/documents/specrend/
fn draw(buffer: &mut [u32]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let i = y as i32 * WIDTH as i32 + x as i32;
            let dx = WIDTH as i32 / 2 - x as i32;
            let dy = (HEIGHT - y) as i32;
            let d = dx * dx + dy * dy;
            let d = (d as f32).sqrt();
            buffer[i as usize] = color(d)
        }
    }
}

fn color(d: f32) -> u32 {
    let wavelength = d as f64 / 1.5;

    let xyz = cie1931wavelength_to_xyzfit(wavelength);
    let rgb = srgb_xyz2rgb(xyz);

    let r = ((rgb[0] * 0xFF as f64) as u32) & 0xFF;
    let g = ((rgb[1] * 0xFF as f64) as u32) & 0xFF;
    let b = ((rgb[2] * 0xFF as f64) as u32) & 0xFF;

    r << 16 | g << 8 | b
}

fn cie1931wavelength_to_xyzfit(wavelength: f64) -> [f64; 3] {
    let wave: f64 = wavelength;

    let x: f64 = {
        let t1: f64 = (wave - 442.0) * (if wave < 442.0 { 0.0624 } else { 0.0374 });
        let t2: f64 = (wave - 599.8) * (if wave < 599.8 { 0.0264 } else { 0.0323 });
        let t3: f64 = (wave - 501.1) * (if wave < 501.1 { 0.0490 } else { 0.0382 });

        0.362 * (-0.5 * t1 * t1).exp() + 1.056 * (-0.5 * t2 * t2).exp()
            - 0.065 * (-0.5 * t3 * t3).exp()
    };

    let y: f64 = {
        let t1: f64 = (wave - 568.8) * (if wave < 568.8 { 0.0213 } else { 0.0247 });
        let t2: f64 = (wave - 530.9) * (if wave < 530.9 { 0.0613 } else { 0.0322 });

        0.821 * (-0.5 * t1 * t1).exp() + 0.286 * (-0.5 * t2 * t2).exp()
    };

    let z: f64 = {
        let t1: f64 = (wave - 437.0) * (if wave < 437.0 { 0.0845 } else { 0.0278 });
        let t2: f64 = (wave - 459.0) * (if wave < 459.0 { 0.0385 } else { 0.0725 });

        1.217 * (-0.5 * t1 * t1).exp() + 0.681 * (-0.5 * t2 * t2).exp()
    };

    [x, y, z]
}

fn srgb_xyz2rgb(xyz: [f64; 3]) -> [f64; 3] {
    let [x, y, z] = xyz;
    let rl: f64 = 3.2406255 * x + -1.537208 * y + -0.4986286 * z;
    let gl: f64 = -0.9689307 * x + 1.8757561 * y + 0.0415175 * z;
    let bl: f64 = 0.0557101 * x + -0.2040211 * y + 1.0569959 * z;

    [
        srgb_xyz2rgbpostprocess(rl),
        srgb_xyz2rgbpostprocess(gl),
        srgb_xyz2rgbpostprocess(bl),
    ]
}

fn srgb_xyz2rgbpostprocess(c: f64) -> f64 {
    let c = if c < 0. {
        0.
    } else if c > 1. {
        1.
    } else {
        c
    };

    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1. / 2.4) - 0.055
    }
}
