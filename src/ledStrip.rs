use nannou::prelude::*;

#[derive(Debug)]
pub struct LEDStrip {
    leds: Vec<(i32, i32, i32)>,
}

impl LEDStrip {
    pub fn new(numLeds: usize) -> Self {
        Self {
            leds: vec!((0,0,0); numLeds),
        }
    }
    pub fn draw(&self, draw: &Draw) {
        let r = 10;
        let h = r * 2 + 10;
        let w = h * self.leds.len() as i32;
        draw.rect()
            .w_h((w + h) as f32,h as f32)
            .color(GRAY);

        for i in (-w/2..=w/2).step_by(h as usize) {
            draw.ellipse()
                .color(RED)
                .radius(r as f32)
                .x_y(i as f32, 0.0);
        }
    }
}
