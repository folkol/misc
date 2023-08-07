use std::time::Duration;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1600;
const HEIGHT: usize = 1200;

type Pos = [f32; 2];
type Triangle = (Pos, f32);

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Poking Pixels",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let mut triangles: Vec<Triangle> = vec![(
        [WIDTH as f32 * 0.5, HEIGHT as f32 * 0.1],
        HEIGHT as f32 * 0.8,
    )];
    window.limit_update_rate(Some(Duration::from_millis(1000)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let heights = triangles[0].1;
        if heights > 2.0 {
            println!("Next gen!");
            triangles = draw_and_split(&mut buffer, &mut triangles);
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_and_split(buffer: &mut [u32], triangles: &mut Vec<Triangle>) -> Vec<Triangle> {
    let mut next_gen: Vec<Triangle> = Vec::new();
    for triangle in triangles {
        draw_triangle(buffer, triangle, ORANGE);
        draw_inverse_triangle(buffer, triangle);
        next_gen.extend(split(triangle))
    }
    next_gen
}

/*
               a
             .   .
            .     .
    ab ->  \       / <- ac
          .         .
         .           .
        b.............c
*/
fn split(triangle: &Triangle) -> Vec<Triangle> {
    let ([x, y], h) = *triangle;
    let w = h * 3.0 / 4.0;
    let a = [x, y];
    let b = [x - w, y + h];
    let c = [x + w, y + h];
    let ab = [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0];
    let ac = [(a[0] + c[0]) / 2.0, (a[1] + c[1]) / 2.0];
    vec![(a, h / 2.0), (ab, h / 2.0), (ac, h / 2.0)]
}

fn draw_triangle(buffer: &mut [u32], triangle: &Triangle, color: u32) {
    let ([col, row], h) = *triangle;
    for d in 0..h as usize {
        let d = d as f32;
        let dd = d * 3.0 / 4.0;
        draw_line(buffer, col, row + d, dd, color);
    }
}

const ORANGE: u32 = 255u32 << 16 | 128u32 << 8 | 64u32 << 0;
const RED: u32 = 255u32 << 16;

fn draw_line(buffer: &mut [u32], col: f32, row: f32, base: f32, color: u32) {
    let base = base as usize;
    for d in 0..=base {
        let d = d as f32;
        draw_pixel(buffer, row, col - base as f32 + d, color);
        draw_pixel(buffer, row, col + base as f32 - d, color);
    }
}

fn draw_pixel(buffer: &mut [u32], row: f32, col: f32, color: u32) {
    let row = row.round();
    let col = col.round();
    if row < 0.0 || row >= HEIGHT as f32 || col < 0.0 || col >= WIDTH as f32 {
        return;
    }
    let row = row as usize;
    let col = col as usize;
    let k = row * WIDTH + col;
    if k < WIDTH * HEIGHT {
        buffer[k] = color;
    }
}

fn draw_inverse_triangle(buffer: &mut [u32], triangle: &Triangle) {
    let ([x, y], h) = *triangle;
    let ([col, row], h1) = ([x, y + h], -h);
    for d in 0..(-h1 as usize) {
        let d = d as f32 * 0.5;
        let dd = d * 3.0 / 4.0;
        draw_line(buffer, col, row - d, dd, 0);
    }
}
