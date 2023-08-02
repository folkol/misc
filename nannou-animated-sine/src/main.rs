use nannou::prelude::*;

const NUM_POINTS: usize = 200;

#[derive(Debug)]
struct Model {}

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(BLACK);
    let points: Vec<_> = (0..NUM_POINTS)
        .into_iter()
        .enumerate()
        .map(|(i, e)| {
            (
                map_range(i, 0, NUM_POINTS - 1, win.left() / 2., win.right() / 2.),
                100. * (app.time * 5. + (5. / 60.) * e as f32).sin(),
            )
        })
        .collect();
    draw.polyline()
        .stroke_weight(20.)
        .start_cap_round()
        .end_cap_round()
        .hsla(
            app.time.sin().abs(),
            0.5,
            0.5,
            0.1 + app.time.sin().abs() * 0.3,
        )
        .points(points.clone());
    draw.polyline()
        .stroke_weight(15.)
        .start_cap_round()
        .end_cap_round()
        .hsla(
            (app.time).sin().abs(),
            0.5,
            0.5,
            0.2 + app.time.sin().abs() * 0.8,
        )
        .points(points);
    draw.to_frame(app, &frame).unwrap();
}
