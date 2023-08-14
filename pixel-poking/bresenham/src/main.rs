use std::time::SystemTime;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 800;

const HEIGHT: u32 = 600;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_title("Bresenham")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };
    let mut num_lines = 0;
    let begin = SystemTime::now();
    let mut num_frames = 0;
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) && input.close_requested() {
            *control_flow = ControlFlow::Exit;
        };
        draw_lines(pixels.frame_mut(), num_lines);
        num_lines += 1;
        pixels.render().expect("render failed");
        let fps = num_frames as f32 / begin.elapsed().unwrap().as_secs_f32();
        num_frames += 1;
        println!("fps: {fps}");
    });
}

fn draw_lines(buffer: &mut [u8], num_lines: i32) {
    let num_lines = num_lines as f32 * 10.0;
    draw_line(
        buffer,
        (num_lines) as i32,
        (HEIGHT * 9 / 10) as i32,
        (WIDTH as f32 - num_lines) as i32,
        (HEIGHT / 10) as i32,
    );
}

fn draw_line(buffer: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32) {
    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            draw_line_low(buffer, x1, y1, x0, y0);
        } else {
            draw_line_low(buffer, x0, y0, x1, y1);
        }
    } else {
        if y0 > y1 {
            draw_line_high(buffer, x1, y1, x0, y0);
        } else {
            draw_line_high(buffer, x0, y0, x1, y1);
        }
    }
}

fn draw_line_low(buffer: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32) {
    let dx = x1 - x0;
    let mut dy: i32 = y1 - y0;
    let mut yi: i32 = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = (2 * dy) - dx;
    let mut y = y0;

    let color = unsafe { get_color() };
    for x in x0..=x1 {
        plot(buffer, x, y, color);
        if d > 0 {
            y += yi;
            d += 2 * (dy - dx);
        } else {
            d += 2 * dy;
        }
    }
}
fn draw_line_high(buffer: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32) {
    let mut dx = x1 - x0;
    let dy: i32 = y1 - y0;
    let mut xi: i32 = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut d = (2 * dx) - dy;
    let mut x = x0;

    let color = unsafe { get_color() };
    for y in y0..=y1 {
        plot(buffer, x, y, color);
        if d > 0 {
            x += xi;
            d += 2 * (dx - dy);
        } else {
            d += 2 * dx;
        }
    }
}

static mut K: u32 = 0;
unsafe fn get_color() -> (u8, u8, u8) {
    let k = unsafe { K };
    let l = k % (255 * 3);
    let mut b = 0;
    let mut g = 0;
    let mut r = 0;
    if l < 256 {
        b = l % 255;
    } else if l < 255 * 2 {
        b = 255;
        g = l % 255;
    } else {
        b = 255;
        g = 255;
        r = l % 255;
    }
    unsafe { K = (k + 1) % (255 * 255 * 255) };
    (r as u8, g as u8, b as u8)
}
fn plot(buffer: &mut [u8], x: i32, y: i32, (r, g, b): (u8, u8, u8)) {
    let i = (y * WIDTH as i32 + x) * 4;
    if i > 0 && i < (WIDTH * HEIGHT * 4) as i32 {
        buffer[0 + i as usize] = r;
        buffer[1 + i as usize] = g;
        buffer[2 + i as usize] = b;
        buffer[3 + i as usize] = 255;
    }
}
