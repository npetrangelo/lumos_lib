mod ledStrip;

use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    leds: Vec<(i32, i32, i32)>,
}

fn model(_app: &App) -> Model {
    Model { leds: Vec::new() }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    // frame.clear(PURPLE);
    let draw = app.draw();
    draw.background().color(PLUM);
    // Draw a blue ellipse with default size and position.
    draw.ellipse()
        .color(STEELBLUE)
        .w(300.0)
        .h(200.0)
        .x_y(200.0, -100.0);

    let w = app.window_rect().w() as i32;
    for i in (-w/2..w/2).step_by(100) {
        draw.ellipse()
            .color(RED)
            .x_y(i as f32, -100.0);
    }

    draw.to_frame(app, &frame).unwrap();
}