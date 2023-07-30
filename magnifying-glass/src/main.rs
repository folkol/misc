use image::{DynamicImage, GenericImageView};
use pixel_canvas::{input::MouseState, Canvas, Color};

const W: usize = 1000;
const H: usize = 572;
const RADIUS: f64 = 150.;

// Test image from https://stock.adobe.com/
fn main() {
    let data = include_bytes!("../assets/test.jpg");
    let bg = image::load_from_memory(data).unwrap();
    let canvas = Canvas::new(W, H)
        .title("Magnifying Glass")
        .state(MouseState::new())
        .input(MouseState::handle_input);
    canvas.render(move |mouse, image| {
        let width = image.width() as u32;
        let height = image.height() as u32;
        for (y, row) in image.chunks_mut(width as usize).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let distance = distance(mouse, x, y);
                let [x, y] = if distance > RADIUS {
                    [x as i32, y as i32]
                } else {
                    let (dx, dy) = project(
                        x as f64 - mouse.x as f64,
                        y as f64 - mouse.y as f64,
                        distance,
                    );
                    [x as i32 + dx, y as i32 + dy]
                };
                *pixel = sample(&bg, width, height, x as u32, y as u32);
            }
        }
    });
}

fn distance(mouse: &MouseState, x: usize, y: usize) -> f64 {
    let a = x.abs_diff(mouse.x as usize).pow(2);
    let b = y.abs_diff(mouse.y as usize).pow(2);
    let distance_squared = (a + b) as f64;
    distance_squared.sqrt()
}

fn sample(bg: &DynamicImage, width: u32, height: u32, x: u32, y: u32) -> Color {
    let sample_x = x * bg.width() / width;
    let sample_y = y * bg.height() / height;
    let rgba = bg.get_pixel(
        sample_x.clamp(0, bg.width() - 1),
        sample_y.clamp(0, bg.height() - 1),
    );
    Color {
        r: rgba.0[0],
        g: rgba.0[1],
        b: rgba.0[2],
    }
}

fn project(dx: f64, dy: f64, distance: f64) -> (i32, i32) {
    let d_norm = distance / RADIUS;
    let g = (-0.5 + d_norm).tan();
    let dx = dx * g;
    let dy = dy * g;
    (dx as i32, dy as i32)
}
