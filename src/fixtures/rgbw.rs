use palette::Srgb;

use crate::fixture::FixtureControl;
use crate::util::f32_to_u8;

#[derive(Clone, Copy, Debug)]
pub struct RgbwValue {
    pub rgb: Srgb<f32>,
    pub white: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Rgbw {
    address: usize,
    value: RgbwValue,
}

impl Rgbw {
    pub fn new(address: usize, value: RgbwValue) -> Self {
        Self { address, value }
    }
}

impl FixtureControl for Rgbw {
    type Value = RgbwValue;

    fn address(&self) -> usize {
        self.address
    }

    fn length(&self) -> usize {
        4
    }

    fn set(&mut self, value: Self::Value) {
        self.value = value;
    }

    fn get(&self) -> Self::Value {
        self.value
    }

    fn outputs(&self) -> Vec<u8> {
        let RgbwValue {
            rgb: rgb_f32,
            white: white_f32,
        } = self.value;
        let rgb_u8: Srgb<u8> = rgb_f32.into_format();
        let (red_u8, green_u8, blue_u8) = rgb_u8.into_components();
        let white_u8 = f32_to_u8(white_f32);

        vec![red_u8, green_u8, blue_u8, white_u8]
    }
}
