use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

use crate::fixture::FixtureControl;
use crate::fixtures::rgbw::{Rgbw, RgbwValue};
use crate::funs::{f32_line_sin_fun, rgb_line_rainbow_fun};
use crate::scene::SceneControl;

#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
pub struct RgbwRainbowConfig {
    pub hue_speed: f32,
    pub hue_range: f32,
    pub white_speed: f32,
    pub white_range: f32,
    pub white_max: f32,
}

#[derive(Debug, Clone)]
pub struct RgbwRainbow {
    pub fixtures: Vec<Rc<RefCell<Rgbw>>>,
    pub config: RgbwRainbowConfig,
}

impl SceneControl for RgbwRainbow {
    type Fixture = Rc<RefCell<Rgbw>>;
    type Config = RgbwRainbowConfig;

    fn fixtures(&mut self) -> Vec<Self::Fixture> {
        self.fixtures.clone()
    }

    fn play(&mut self, time: f32) {
        let config = self.config;

        let length = self.fixtures.len();
        for (index, mut fixture) in self.fixtures().into_iter().enumerate() {
            let position = (index as f32) / (length as f32);

            let rgb = rgb_line_rainbow_fun(time, position, config.hue_speed, config.hue_range);
            let white = f32_line_sin_fun(
                time,
                position,
                config.white_speed,
                config.white_range,
                config.white_max,
            );

            fixture.set(RgbwValue { rgb, white });
        }
    }
}
