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
    let size = LogicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_title("Bounce that pixel!")
        .with_inner_size(size)
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dx: i32 = 1;
    let mut dy: i32 = 1;
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) && input.close_requested() {
            println!("ESC");
            *control_flow = ControlFlow::Exit
        }

        let buf = pixels.frame_mut();
        let offset = (y * WIDTH + x) as usize * 4;
        buf[offset + 0] = 255u8;
        buf[offset + 1] = 255u8;
        buf[offset + 2] = 255u8;
        buf[offset + 3] = 128u8;
        x += dx;
        y += dy;
        if x >= WIDTH || x <= 0 {
            dx *= -1;
            x = x + dx;
        }
        if y >= HEIGHT || y <= 0 {
            dy *= -1;
            y += dy;
        }

        pixels.render().expect("Render failed");
    });
}
