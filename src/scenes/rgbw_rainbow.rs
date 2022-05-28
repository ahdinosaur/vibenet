use crate::fixture::FixtureControl;
use crate::fixtures::rgbw::{Rgbw, RgbwValue};
use crate::funs::{f32_line_sin_fun, rgb_line_rainbow_fun};
use crate::scene::SceneControl;

pub struct RgbwRainbow {
    fixtures: Vec<Rgbw>,
    hue_speed: f32,
    hue_range: f32,
    white_speed: f32,
    white_range: f32,
    white_max: f32,
}

impl SceneControl for RgbwRainbow {
    type Fixture = Rgbw;

    fn fixtures(&mut self) -> Vec<Self::Fixture> {
        self.fixtures
    }

    fn set(&mut self, time: f32) {
        let length = self.fixtures.len();
        for (index, fixture) in self.fixtures.iter().enumerate() {
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
