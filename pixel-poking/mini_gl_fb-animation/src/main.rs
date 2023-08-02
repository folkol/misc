use mini_gl_fb::glutin::event::VirtualKeyCode;
use rand::{Rng, thread_rng};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let (mut e, mut fb) = mini_gl_fb::gotta_go_fast("Redian motion", WIDTH as f64, HEIGHT as f64);

    let mut buf = [[0u8, 0, 0, 1]; WIDTH * HEIGHT];
    let mut x = WIDTH as i32 / 2;
    let mut y = HEIGHT as i32 / 2;
    let mut rng = thread_rng();

    fb.glutin_handle_basic_input(&mut e, |fb, input| {
        if input.keys.contains_key(&VirtualKeyCode::Q) || input.keys.contains_key(&VirtualKeyCode::Escape) {
            return false;
        }

        let dx = &rng.gen_range(-1..=1);
        let dy = &rng.gen_range(-1..=1);
        x += dx;
        y += dy;
        if x < 0 || x >= WIDTH as i32 || y < 0 || y >= HEIGHT as i32 {
            return false;
        }
        let i = y * WIDTH as i32 + x;
        let i = i as usize;
        buf[i][0] = (buf[i][0] / 2 + 255u8 / 2);
        fb.update_buffer(&buf);
        true
    });
}
