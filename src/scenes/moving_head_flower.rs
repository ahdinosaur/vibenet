use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::sync::Arc;

use crate::fixture::FixtureControl;
use crate::fixtures::moving_head::{MovingHead, MovingHeadValue};
use crate::funs::{f32_ramp_fun, moving_head_line_flower_fun};
use crate::scene::SceneControl;

#[derive(Deserialize, Serialize)]
pub struct MovingHeadFlowerConfig {
    pub pan_speed: f32,
    pub pan_range: f32,
    pub tilt_speed: f32,
    pub tilt_range: f32,
    pub color_wheel_mult: f32,
    pub color_wheel_max: f32,
    pub gobo_wheel_mult: f32,
    pub gobo_wheel_max: f32,
}

pub struct MovingHeadFlower {
    pub fixtures: Vec<Arc<RefCell<MovingHead>>>,
    pub config: MovingHeadFlowerConfig,
}

impl SceneControl for MovingHeadFlower {
    type Fixture = Arc<RefCell<MovingHead>>;
    type Config = MovingHeadFlowerConfig;

    fn fixtures(&mut self) -> Vec<Self::Fixture> {
        self.fixtures.clone()
    }

    fn play(&mut self, time: f32) {
        let config = self.config;

        let length = self.fixtures.len();
        for (index, mut fixture) in self.fixtures().into_iter().enumerate() {
            let position = (index as f32) / (length as f32);

            let (pan, tilt, speed) = moving_head_line_flower_fun(
                time,
                position,
                config.pan_speed,
                config.pan_range,
                config.tilt_speed,
                config.tilt_range,
            );
            let color_wheel = f32_ramp_fun(time, config.color_wheel_mult, config.color_wheel_max);
            let gobo_wheel = f32_ramp_fun(time, config.gobo_wheel_mult, config.gobo_wheel_max);
            let strobe = 0_f32;
            let dimmer = 1_f32;

            fixture.set(MovingHeadValue {
                pan,
                tilt,
                speed,
                color_wheel,
                gobo_wheel,
                strobe,
                dimmer,
            });
        }
    }
}
