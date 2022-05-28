use crate::fixture::FixtureControl;
use crate::util::{f32_to_double_u8, f32_to_u8};

#[derive(Clone, Copy, Debug)]
pub struct MovingHeadValue {
    pub pan: f32,
    pub tilt: f32,
    pub speed: f32,
    pub color_wheel: f32,
    pub gobo_wheel: f32,
    pub strobe: f32,
    pub dimmer: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct MovingHead {
    address: usize,
    value: MovingHeadValue,
}

impl MovingHead {
    pub fn new(address: usize, value: MovingHeadValue) -> Self {
        Self { address, value }
    }
}

impl FixtureControl for MovingHead {
    type Value = MovingHeadValue;

    fn address(&self) -> usize {
        self.address
    }

    fn length(&self) -> usize {
        11
    }

    fn set(&mut self, value: Self::Value) {
        self.value = value;
    }

    fn get(&self) -> Self::Value {
        self.value
    }

    fn outputs(&self) -> Vec<u8> {
        let MovingHeadValue {
            pan: pan_f32,
            tilt: tilt_f32,
            speed: speed_f32,
            color_wheel: color_wheel_f32,
            gobo_wheel: gobo_wheel_f32,
            strobe: strobe_f32,
            dimmer: dimmer_f32,
        } = self.value;

        let (pan_coarse, pan_fine) = f32_to_double_u8(pan_f32);
        let (tilt_coarse, tilt_fine) = f32_to_double_u8(tilt_f32);
        let color_wheel = f32_to_u8(color_wheel_f32);
        let gobo_wheel = f32_to_u8(gobo_wheel_f32);
        let strobe = f32_to_u8(strobe_f32);
        let dimmer = f32_to_u8(dimmer_f32);
        let speed = f32_to_u8(speed_f32);
        let auto = 0;
        let command = 0;

        vec![
            pan_coarse,
            pan_fine,
            tilt_coarse,
            tilt_fine,
            color_wheel,
            gobo_wheel,
            strobe,
            dimmer,
            speed,
            auto,
            command,
        ]
    }
}
