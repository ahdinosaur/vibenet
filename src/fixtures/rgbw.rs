use crate::fixture::FixtureControl;
use std::f32::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct RGBW {
    pub address: usize,
}

impl FixtureControl for RGBW {
    fn address(&self) -> usize {
        self.address
    }

    fn output(&mut self, time: f32) -> Vec<u8> {
        vec![
            ((time * 0.1_f32).sin() * 256_f32) as u8,
            ((time * 0.2_f32).sin() * 256_f32) as u8,
            ((time * 0.01_f32).sin() * 256_f32) as u8,
            0,
        ]
    }
}
