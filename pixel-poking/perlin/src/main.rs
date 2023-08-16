use mini_gl_fb::glutin::event::VirtualKeyCode;
use rand::{Rng, thread_rng};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
const GRID_WIDTH: usize = 4;
const GRID_HEIGHT: usize = 4;

fn main() {
    let (mut e, mut fb) = mini_gl_fb::gotta_go_fast("Perlin2D", WIDTH as f64, HEIGHT as f64);

    let mut buf = vec![[0u8, 0, 0, 1]; WIDTH * HEIGHT];
    let mut buff = vec![0f32; WIDTH * HEIGHT];
    let mut rng = thread_rng();
    let mut gradients: Vec<(f32, f32)> = Vec::new();
    for _ in 0..=GRID_WIDTH {
        for _ in 0..=GRID_HEIGHT {
            let theta = rng.gen_range(0.0..std::f32::consts::TAU);
            // let theta = TAU / 8.0;
            gradients.push((theta.cos(), theta.sin()));
        }
    }

    draw(&mut buf, &mut buff, &gradients);

    show_gradients(&mut buf, &mut gradients);

    fb.glutin_handle_basic_input(&mut e, |fb, input| {
        if input.keys.contains_key(&VirtualKeyCode::Q)
            || input.keys.contains_key(&VirtualKeyCode::Escape)
        {
            return false;
        }

        fb.update_buffer(&buf);
        true
    });
}

fn sample(gradients: &[(f32, f32)], x: f32, y: f32) -> f32 {
    let grid_x = x / (1.0 / GRID_WIDTH as f32);
    let grid_y = y / (1.0 / GRID_HEIGHT as f32);
    let grid_step_x = 1.0 / GRID_WIDTH as f32;
    let grid_step_y = 1.0 / GRID_HEIGHT as f32;

    interpolate(
        (x - grid_x.floor() * grid_step_x) / grid_step_x,
        (y - grid_y.floor() * grid_step_y) / grid_step_y,
        contribution(&gradients, x, y, grid_x.floor(), grid_y.floor()),
        contribution(&gradients, x, y, grid_x.ceil(), grid_y.floor()),
        contribution(&gradients, x, y, grid_x.floor(), grid_y.ceil()),
        contribution(&gradients, x, y, grid_x.ceil(), grid_y.ceil()),
    )
}

fn draw(buf: &mut Vec<[u8; 4]>, buff: &mut Vec<f32>, gradients: &[(f32, f32)]) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let offset = y * WIDTH + x;
            let c = sample(gradients, x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32);
            min = min.min(c);
            max = max.max(c);
            buff[offset] = c;
        }
    }
    normalize(buf, buff, min, max);
}

fn normalize(buf: &mut [[u8; 4]], buff: &Vec<f32>, min: f32, max: f32) {
    for (c, i) in buf.iter_mut().zip(buff) {
        let i = (*i - min) / (max - min);
        *c = [(i * 255.0) as u8, 0, 0, 255];
    }
}

fn interpolate(
    u: f32,
    v: f32,
    up_left: f32,
    up_right: f32,
    down_left: f32,
    down_right: f32,
) -> f32 {
    let u = u * u * (3.0 - 2.0 * u);
    let v = v * v * (3.0 - 2.0 * v);
    let avg1 = up_left * (1.0 - u) + up_right * u;
    let avg2 = down_left * (1.0 - u) + down_right * u;
    avg1 * (1.0 - v) + avg2 * v
}

fn contribution(gradients: &[(f32, f32)], x: f32, y: f32, gx: f32, gy: f32) -> f32 {
    let grid_step_x = 1.0 / GRID_WIDTH as f32;
    let grid_step_y = 1.0 / GRID_HEIGHT as f32;

    let i = gy as usize * GRID_WIDTH + gx as usize;
    let gradient = gradients[i];
    let dx = x - gx * grid_step_x;
    let dy = y - gy * grid_step_y;
    gradient.0 * dx + gradient.1 * dy
}

fn show_gradients(buf: &mut [[u8; 4]], gradients: &mut Vec<(f32, f32)>) {
    let grid_step_x = 1.0 / GRID_WIDTH as f32;
    let grid_step_y = 1.0 / GRID_HEIGHT as f32;
    let arrow_len = 10;
    for y in 0..=GRID_WIDTH {
        for x in 0..=GRID_WIDTH {
            let gradient = gradients[y * GRID_WIDTH + x];
            let (from_x, from_y) = (
                x as f32 * grid_step_x * WIDTH as f32,
                y as f32 * grid_step_y * HEIGHT as f32,
            );
            let (to_x, to_y) = (
                from_x + gradient.0 * arrow_len as f32,
                from_y + gradient.1 * arrow_len as f32,
            );
            let (dx, dy) = (to_x - from_x, to_y - from_y);
            if y == 0 && dy < 0.0 {
                continue;
            }
            if y == GRID_HEIGHT && dy > 0.0 {
                continue;
            }
            if x == 0 && dx < 0.0 {
                continue;
            }
            if x == GRID_WIDTH && dx > 0.0 {
                continue;
            }
            for n in 1..arrow_len {
                let pixel_y = from_y + n as f32 * dy / arrow_len as f32;
                let pixel_x = from_x + n as f32 * dx / arrow_len as f32;
                let offset = pixel_y * WIDTH as f32 + pixel_x;
                if offset >= 0.0 && (offset as usize) < WIDTH * HEIGHT {
                    let offset = pixel_y as usize * WIDTH + pixel_x as usize;
                    buf[offset] = [0, 255, 255, 255];
                }
            }
        }
    }
}
