use crate::fixture::FixtureControl;

#[derive(Debug, Copy, Clone)]
pub struct RGBW {
    pub address: usize,
}

impl FixtureControl for RGBW {
    fn address(&self) -> usize {
        self.address
    }

    fn output(&mut self, _time: f32) -> Vec<u8> {
        vec![255, 0, 0, 0]
    }
}
