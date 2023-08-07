use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 2048;
const HEIGHT: usize = 1024;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "0rgb colors (row by row, stride 8)",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    for i in 0..WIDTH * HEIGHT {
        buffer[i] = i as u32 * 8;
    }
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
