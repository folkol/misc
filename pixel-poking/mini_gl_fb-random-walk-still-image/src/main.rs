use rand::{Rng, thread_rng};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let (mut event_loop, mut fb) = mini_gl_fb::gotta_go_fast("Hello world!", WIDTH as f64, HEIGHT as f64);
    let buffer = paint();
    fb.update_buffer(&buffer);
    fb.persist(&mut event_loop);
}

fn paint() -> Vec<[u8; 4]> {
    let mut rng = thread_rng();
    let mut buffer = vec![[0, 0, 28u8, 255]; WIDTH * HEIGHT];
    let (mut x, mut y) = (WIDTH / 2, HEIGHT / 2);
    let mut num_steps = 0;
    while x > 300 && x < 500 && y < 400 && y > 200 && num_steps < 1_000_000 {
        num_steps += 1;
        buffer[y * WIDTH + x][1] = 100;
        let step = 1;
        match rng.gen_range(0..=3) {
            0 => { x += step }
            1 => { y += step }
            2 => { x -= step }
            3 => { y -= step; }
            _ => panic!("Unexpected direction")
        }
    }
    buffer
}