use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Cube",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let mut corners = [
        [-1.0, 1.0, -1.0],
        [1.0, 1.0, -1.0],
        [1.0, -1.0, -1.0],
        [-1.0, -1.0, -1.0],
        [-1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, -1.0, 1.0],
        [-1.0, -1.0, 1.0],
    ];
    let _rotation = 0;
    // window.limit_update_rate(Some(Duration::from_millis(1000 / 60)));
    // window.limit_update_rate(Some(Duration::from_secs(1)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        rotate(&mut corners);
        draw(&mut buffer, &mut corners);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn rotate(corners: &mut [[f64; 3]; 8]) {
    // xyz, z is facing us, y is up
    for [x, y, z] in corners.iter_mut() {
        // multiplying with complex number
        let [a, b] = [1.0f64, 0.01f64];
        let norm = (a * a + b * b).sqrt();
        let [a_norm, b_norm] = [a / norm, b / norm];
        [*z, *x] = [*z * a_norm - *x * b_norm, *x + b_norm * *z * a_norm]
    }
}

fn draw(buffer: &mut [u32], corners: &mut [[f64; 3]; 8]) {
    buffer.iter_mut().for_each(|pixel| *pixel /= 2);
    draw_line(buffer, corners[0], corners[1]);
    draw_line(buffer, corners[1], corners[2]);
    draw_line(buffer, corners[2], corners[3]);
    draw_line(buffer, corners[3], corners[0]);
    draw_line(buffer, corners[4], corners[5]);
    draw_line(buffer, corners[5], corners[6]);
    draw_line(buffer, corners[6], corners[7]);
    draw_line(buffer, corners[7], corners[4]);
    draw_line(buffer, corners[0], corners[4]);
    draw_line(buffer, corners[1], corners[5]);
    draw_line(buffer, corners[2], corners[6]);
    draw_line(buffer, corners[3], corners[7]);
}

fn draw_line(buffer: &mut [u32], from: [f64; 3], to: [f64; 3]) {
    let from = [
        from[0] / (2.1 - from[2]).abs(),
        from[1] / (2.1 - from[2]).abs(),
    ];
    let to = [to[0] / (2.1 - to[2]).abs(), to[1] / (2.1 - to[2]).abs()];
    let num_steps = 100;
    let dx = (to[0] - from[0]) / num_steps as f64;
    let dy = (to[1] - from[1]) / num_steps as f64;
    for n in 0..num_steps {
        let x = map_range(from[0] + n as f64 * dx, -1.0, 1.0, 0, WIDTH);
        let y = map_range(from[1] + n as f64 * dy, -1.0, 1.0, 0, HEIGHT);
        let pixel = y * WIDTH + x;
        let pixel = pixel.clamp(0, WIDTH * HEIGHT - 1);
        // todo: dim in distance?
        buffer[pixel] = 255u32 << 16 | 128u32 << 7;
    }
}

fn map_range(x: f64, a: f64, b: f64, c: usize, d: usize) -> usize {
    let in_range = b - a;
    let out_range = (d - c + 1) as f64;
    let i = (x - a) / in_range;
    c + (i * out_range) as usize
}

#[test]
fn test_map() {
    assert_eq!(map_range(-2.0, -1.0, 1.0, 0, 100), 0);
    assert_eq!(map_range(2.0, -1.0, 1.0, 0, 100), 100);
    assert_eq!(map_range(-1.0, -1.0, 1.0, 0, 100), 0);
    assert_eq!(map_range(1.0, -1.0, 1.0, 0, 100), 100);
    assert_eq!(map_range(-0.3, -1.0, 1.0, 0, 100), 35);
    assert_eq!(map_range(0.3, -1.0, 1.0, 0, 100), 65);
}
