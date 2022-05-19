use crate::fixture::FixtureControl;

#[derive()]
pub struct MovingHead {
    pub index: usize,
    // (pan, tilt, speed)
    pub position_fun: Box<dyn Fn(f32) -> (f32, f32, f32)>,
    pub color_wheel_fun: Box<dyn Fn(f32) -> u8>,
    pub gobo_wheel_fun: Box<dyn Fn(f32) -> u8>,
    // pub strobe_fun: Box<dyn Fn(f32) -> u8>
    // pub dimmer_fun: Box<dyn Fn(f32) -> u8>
}

impl FixtureControl for MovingHead {
    fn channels(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(self.index..(self.index + 11))
    }

    fn output(&mut self, time: f32) -> Box<dyn Iterator<Item = u8>> {
        let (pan_f, tilt_f, speed_f) = (self.position_fun)(time);
        let (pan_coarse, pan_fine) = float_to_double_int(pan_f);
        let (tilt_coarse, tilt_fine) = float_to_double_int(tilt_f);
        let color_wheel = (self.color_wheel_fun)(time);
        let gobo_wheel = (self.gobo_wheel_fun)(time);
        let strobe: u8 = 0;
        let dimmer: u8 = 255;
        let speed: u8 = float_to_int(speed_f);
        let auto: u8 = 0;
        let command: u8 = 0;

        Box::new(
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
            .into_iter(),
        )
    }
}

fn float_to_int(value: f32) -> u8 {
    (value * 256_f32) as u8
}

fn float_to_double_int(value: f32) -> (u8, u8) {
    let coarse = (value * 256_f32) as u8;
    let fine = (((value * 256_f32) % 1_f32) * 256_f32) as u8;
    (coarse, fine)
}
