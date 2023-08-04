use std::time::SystemTime;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: i32 = 320;
const HEIGHT: i32 = 200;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_title("Ripple it!")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let epicenter = (WIDTH / 2, HEIGHT / 2);
    let begin = SystemTime::now();
    event_loop.run(move |event, _, control_flow| {
        let elapsed = begin.elapsed().unwrap().as_secs_f32();
        if input.update(&event) && input.close_requested() {
            *control_flow = ControlFlow::Exit
        }

        let buf = pixels.frame_mut();
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let offset = (row * WIDTH + col) as usize * 4;
                let base = 128;
                let dx = epicenter.0 - col;
                let dy = epicenter.1 - row;
                let d = dx * dx + dy * dy;
                let d = d as f32;
                let k = d - elapsed * 4.;

                let color = base + (75f32 * k.sin()) as u8;
                let r = color;
                let g = color;
                let b = color;
                let a = 255u8;

                buf[offset + 0] = r;
                buf[offset + 1] = g;
                buf[offset + 2] = b;
                buf[offset + 3] = a;
            }
        }

        pixels.render().expect("Render failed");
    });
}
