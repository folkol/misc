use image::GenericImageView;
use pixel_canvas::{input::MouseState, Canvas, Color};

fn main() {
    let data = include_bytes!("../assets/test.jpeg");
    let bg = image::load_from_memory(data).unwrap();
    let canvas = Canvas::new(500, 500)
        .title("Tile")
        .state(MouseState::new())
        .input(MouseState::handle_input);
    canvas.render(move |mouse, image| {
        let width = image.width();
        let height = image.height();
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let x = x as i32;
                let y = y as i32;
                let dx = x - mouse.x;
                let dy = y - mouse.y;
                let a = x.abs_diff(mouse.x).pow(2);
                let b = y.abs_diff(mouse.y).pow(2);
                let distance = ((a + b) as f64).sqrt();
                if distance > 75. {
                    let sample_x = x * bg.width() as i32 / width as i32;
                    let sample_y = y * bg.height() as i32 / height as i32;
                    let rgba = bg.get_pixel(
                        (sample_x as u32).clamp(0, bg.width() - 1),
                        (sample_y as u32).clamp(0, bg.height() - 1),
                    );
                    *pixel = Color {
                        r: rgba.0[0],
                        g: rgba.0[1],
                        b: rgba.0[2],
                    };
                } else {
                    let d_norm = distance / 75.0;
                    let g = (-0.5 + d_norm).tan();
                    let x = x as f64 + dx as f64 * g;
                    let y = y as f64 + dy as f64 * g;
                    let x = x.clamp(0., width as f64);
                    let y = y.clamp(0., height as f64);
                    let sample_x = x as i32 * bg.width() as i32 / width as i32;
                    let sample_y = y as i32 * bg.height() as i32 / height as i32;
                    let sample_x = sample_x.clamp(0, bg.width() as i32 - 1);
                    let sample_y = sample_y.clamp(0, bg.height() as i32 - 1);
                    let rgba = bg.get_pixel(sample_x as u32, sample_y as u32);
                    *pixel = Color {
                        r: rgba.0[0],
                        g: rgba.0[1],
                        b: rgba.0[2],
                    };
                }
            }
        }
    });
}

#[test]
fn plot_arc_cos() {
    for x in 1..75 {
        let x = x as f64;
        let d_inv = 75. - x;
        let d_inv_norm = 1. + -1. * (d_inv / 75.0);
        let g = d_inv_norm;
        // let g = g * 75.;
        for _ in 1..(g as i32) {
            print!(" ")
        }
        println!(". ({g})");
    }
}
