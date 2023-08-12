use std::collections::{HashMap, HashSet};
use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use rand::Rng;

const W: usize = 10;
const COLS: usize = 50;
const ROWS: usize = 50;
const WIDTH: usize = COLS * W + 1;
const HEIGHT: usize = ROWS * W + 1;
const WHITE: u32 = 255u32 << 16 | 255u32 << 8 | 255u32;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Amazeing",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let mut walls: HashMap<(usize, usize, usize, usize), bool> = HashMap::new();
    for x in 0..COLS {
        for y in 0..ROWS {
            walls.entry((x, y, x + 1, y)).or_insert(true);
            walls.entry((x, y, x, y + 1)).or_insert(true);
        }
    }
    // window.limit_update_rate(Some(Duration::from_millis(1000 / 4)));
    init_grid(&mut buffer, &walls);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut pos = (0, 0);
    visited.insert(pos);
    // draw_cross(&mut buffer, pos, 255u32 << 16);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        match step(&visited, &mut stack, pos) {
            Some((x, y)) => {
                walls
                    .entry((pos.0, pos.1, x, y))
                    .and_modify(|wall| *wall = false);
                visited.insert((x, y));
                break_wall(&mut buffer, pos, (x, y));
                // draw_cross(&mut buffer, pos, 128u32 << 16);
                pos = (x, y);
                // draw_cross(&mut buffer, pos, 255u32 << 16);
            }
            None => {
                if let Some(p) = stack.pop() {
                    pos = p;
                    continue;
                }
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_cross(buffer: &mut [u32], pos: (usize, usize), i: u32) {
    for j in 1..(W - 1) {
        let pixel_x = (pos.0 * W) + j;
        let pixel_y = (pos.1 * W) + j;
        let offset = pixel_y * WIDTH + pixel_x;
        if offset < WIDTH * HEIGHT {
            buffer[offset] = i;
        }
        let pixel_x = (pos.0 * W + W) - j;
        let pixel_y = (pos.1 * W) + j;
        let offset = pixel_y * WIDTH + pixel_x;
        if offset < WIDTH * HEIGHT {
            buffer[offset] = i;
        }
    }
}

fn break_wall(buffer: &mut [u32], mut from: (usize, usize), mut to: (usize, usize)) {
    if from.0 > to.0 {
        (from.0, to.0) = (to.0, from.0)
    }
    if from.1 > to.1 {
        (from.1, to.1) = (to.1, from.1)
    }
    if from.1 == to.1 {
        // break right wall
        for k in 1..W {
            let pixel_x = to.0 * W;
            let pixel_y = to.1 * W;
            let offset = (pixel_y + k) * WIDTH + pixel_x;
            if offset < WIDTH * HEIGHT {
                buffer[offset] = 0;
            }
        }
    } else {
        // break bottom wall
        for k in 1..W {
            let pixel_x = from.0 * W;
            let pixel_y = (from.1 + 1) * W;
            let offset = pixel_y * WIDTH + pixel_x + k;
            if offset < WIDTH * HEIGHT {
                buffer[offset] = 0;
            }
        }
    }
}

fn step(
    visited: &HashSet<(usize, usize)>,
    stack: &mut Vec<(usize, usize)>,
    pos: (usize, usize),
) -> Option<(usize, usize)> {
    let neighbours = neighbours(pos);
    let candidates: HashSet<_> = neighbours.difference(visited).collect();
    if candidates.is_empty() {
        return None;
    }
    stack.push(pos);
    let m = candidates.len();
    let n: usize = thread_rng().gen();
    candidates.iter().skip(n % m).map(|x| **x).next()
}

fn neighbours((x, y): (usize, usize)) -> HashSet<(usize, usize)> {
    let mut result: HashSet<(usize, usize)> = HashSet::new();
    if x > 0 {
        result.insert((x - 1, y));
    }
    if y > 0 {
        result.insert((x, y - 1));
    }
    if x < (COLS - 1) {
        result.insert((x + 1, y));
    }
    if y < (ROWS - 1) {
        result.insert((x, y + 1));
    }
    result
}

fn init_grid(buffer: &mut [u32], walls: &HashMap<(usize, usize, usize, usize), bool>) {
    for cell_x in 0..COLS {
        for cell_y in 0..ROWS {
            if let Some(true) = walls.get(&(cell_x, cell_y, cell_x + 1, cell_y)) {
                for k in 0..W {
                    let pixel_x = cell_x * W;
                    let pixel_y = cell_y * W;
                    let offset = (pixel_y + k) * WIDTH + pixel_x;
                    if offset < WIDTH * HEIGHT {
                        buffer[offset] = WHITE;
                    }
                }
            }
            if let Some(true) = walls.get(&(cell_x, cell_y, cell_x, cell_y + 1)) {
                for k in 0..W {
                    let pixel_x = cell_x * W;
                    let pixel_y = (cell_y + 1) * W;
                    let offset = pixel_y * WIDTH + pixel_x + k;
                    if offset < WIDTH * HEIGHT {
                        buffer[offset] = WHITE;
                    }
                }
            }
        }
    }
    for y in 0..HEIGHT {
        let x = WIDTH - 1;
        buffer[y * WIDTH + x] = WHITE;
        let x = 0;
        buffer[y * WIDTH + x] = WHITE;
    }
    for x in 0..WIDTH {
        let y = HEIGHT - 1;
        buffer[y * WIDTH + x] = WHITE;
        let y = 0;
        buffer[y * WIDTH + x] = WHITE;
    }
}

#[test]
fn test_neighbours() {
    let n = neighbours((6, 10));
    for (x, y) in n {
        assert!(x < COLS && y < ROWS, "x: {x} y: {y}");
    }
}
