use crate::fixture::FixtureControl;
use crate::fixtures::moving_head::{MovingHead, MovingHeadValue};
use crate::funs::{f32_ramp_fun, moving_head_line_flower_fun};
use crate::scene::SceneControl;

pub struct MovingHeadFlower {
    fixtures: Vec<MovingHead>,
    hue_speed: f32,
    hue_range: f32,
    white_max: f32,
}

impl SceneControl for MovingHeadFlower {
    type Fixture = MovingHead;

    fn fixtures(&mut self) -> Vec<Self::Fixture> {
        self.fixtures
    }

    fn set(&mut self, time: f32) {
        let length = self.fixtures.len();
        for (index, fixture) in self.fixtures.iter().enumerate() {
            let position = (index as f32) / (length as f32);

            let pan_speed = 0.4_f32;
            let pan_range = 1_f32;
            let tilt_speed = 0.1_f32;
            let tilt_range = 1_f32;
            let (pan, tilt, speed) = moving_head_line_flower_fun(
                time, position, pan_speed, pan_range, tilt_speed, tilt_range,
            );

            let color_wheel_mult = 4_f32;
            let color_wheel_max = 0.5_f32;
            let color_wheel = f32_ramp_fun(time, color_wheel_mult, color_wheel_max);

            let gobo_wheel_mult = 4_f32;
            let gobo_wheel_max = 0.5_f32;
            let gobo_wheel = f32_ramp_fun(time, gobo_wheel_mult, gobo_wheel_max);

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
