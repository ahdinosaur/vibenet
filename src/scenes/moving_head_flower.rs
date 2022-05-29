use std::cell::RefCell;
use std::sync::Arc;

use crate::fixture::FixtureControl;
use crate::fixtures::moving_head::{MovingHead, MovingHeadValue};
use crate::funs::{f32_ramp_fun, moving_head_line_flower_fun};
use crate::scene::SceneControl;

pub struct MovingHeadFlower {
    pub fixtures: Vec<Arc<RefCell<MovingHead>>>,
    pub pan_speed: f32,
    pub pan_range: f32,
    pub tilt_speed: f32,
    pub tilt_range: f32,
    pub color_wheel_mult: f32,
    pub color_wheel_max: f32,
    pub gobo_wheel_mult: f32,
    pub gobo_wheel_max: f32,
}

impl SceneControl for MovingHeadFlower {
    type Fixture = Arc<RefCell<MovingHead>>;

    fn fixtures(&mut self) -> Vec<Self::Fixture> {
        self.fixtures.clone()
    }

    fn set(&mut self, time: f32) {
        let length = self.fixtures.len();
        for (index, mut fixture) in self.fixtures().into_iter().enumerate() {
            let position = (index as f32) / (length as f32);

            let (pan, tilt, speed) = moving_head_line_flower_fun(
                time,
                position,
                self.pan_speed,
                self.pan_range,
                self.tilt_speed,
                self.tilt_range,
            );
            let color_wheel = f32_ramp_fun(time, self.color_wheel_mult, self.color_wheel_max);
            let gobo_wheel = f32_ramp_fun(time, self.gobo_wheel_mult, self.gobo_wheel_max);
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
