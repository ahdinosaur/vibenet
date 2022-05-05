use crate::fixture::FixtureControl;

#[derive()]
pub struct RGBW {
    pub address: usize,
    pub output_fn: Box<dyn Fn(f32) -> Vec<u8>>,
}

impl FixtureControl for RGBW {
    fn address(&self) -> usize {
        self.address
    }

    fn output(&mut self, time: f32) -> Vec<u8> {
        (self.output_fn)(time)
    }
}
