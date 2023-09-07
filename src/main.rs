mod ledStrip;
use ledStrip::LEDStrip;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    leds: LEDStrip,
}

fn model(_app: &App) -> Model {
    Model { leds: LEDStrip::new(20) }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    model.leds.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}