use std::time::Duration;

use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

const COLOR: u32 = 255u32 << 8;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Circle",
        WIDTH,
        HEIGHT,
        WindowOptions {
            // borderless: false,
            // title: false,
            resize: true,
            // ..WindowOptions::default()
            scale: Scale::X8,
            // scale_mode: ScaleMode::Stretch,
            // topmost: false,
            // transparency: false,
            // none: false,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    window.limit_update_rate(Some(Duration::from_millis(1000 / 4)));

    let center_x: usize = 32;
    let center_y: usize = 32;
    let r: i32 = 20;

    {
        let r2 = r + r;

        let mut pen_x = r;
        let mut pen_y = 0;
        let mut dy: i32 = -2;
        let mut dx = r2 + r2 - 4;
        let mut d = r2 - 1;

        while pen_y <= pen_x {
            {
                let pen_x = pen_x as usize;
                let pen_y = pen_y as usize;
                buffer[(center_y - pen_y) * WIDTH + (center_x - pen_x)] = COLOR;
                if !window.is_open() {
                    break;
                }
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

                if !window.is_open() {
                    break;
                }
                buffer[(center_y - pen_y) * WIDTH + (center_x + pen_x)] = COLOR;
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

                if !window.is_open() {
                    break;
                }
                buffer[(center_y + pen_y) * WIDTH + (center_x - pen_x)] = COLOR;
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

                if !window.is_open() {
                    break;
                }
                buffer[(center_y + pen_y) * WIDTH + (center_x + pen_x)] = COLOR;
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

                if !window.is_open() {
                    break;
                }
                buffer[(center_y - pen_x) * WIDTH + (center_x - pen_y)] = COLOR;
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

                if !window.is_open() {
                    break;
                }
                buffer[(center_y - pen_x) * WIDTH + (center_x + pen_y)] = COLOR;
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

                if !window.is_open() {
                    break;
                }
                buffer[(center_y + pen_x) * WIDTH + (center_x - pen_y)] = COLOR;
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

                if !window.is_open() {
                    break;
                }
                buffer[(center_y + pen_x) * WIDTH + (center_x + pen_y)] = COLOR;
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
            }

            d += dy;
            dy -= 4;
            pen_y += 1;

            let mask: i32 = d >> 31;
            d += dx & mask;
            dx -= 4 & mask;
            pen_x += mask;
        }
    }

    window.limit_update_rate(Some(Duration::from_millis(1000 / 60)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
