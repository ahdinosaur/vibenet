use std::cell::RefCell;
use std::sync::Arc;

use crate::fixture::FixtureControl;
use crate::fixtures::rgbw::{Rgbw, RgbwValue};
use crate::funs::{f32_line_sin_fun, rgb_line_rainbow_fun};
use crate::scene::SceneControl;

pub struct RgbwRainbow {
    pub fixtures: Vec<Arc<RefCell<Rgbw>>>,
    pub hue_speed: f32,
    pub hue_range: f32,
    pub white_speed: f32,
    pub white_range: f32,
    pub white_max: f32,
}

impl SceneControl for RgbwRainbow {
    type Fixture = Arc<RefCell<Rgbw>>;

    fn fixtures(&mut self) -> Vec<Self::Fixture> {
        self.fixtures.clone()
    }

    fn set(&mut self, time: f32) {
        let length = self.fixtures.len();
        for (index, mut fixture) in self.fixtures().into_iter().enumerate() {
            let position = (index as f32) / (length as f32);

            let rgb = rgb_line_rainbow_fun(time, position, self.hue_speed, self.hue_range);
            let white = f32_line_sin_fun(
                time,
                position,
                self.white_speed,
                self.white_range,
                self.white_max,
            );

            fixture.set(RgbwValue { rgb, white });
        }
    }
}
