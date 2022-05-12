use crate::fixture::FixtureControl;

#[derive()]
pub struct RGBW {
    pub index: usize,
    pub output_fn: Box<dyn Fn(f32) -> Vec<u8>>,
}

impl FixtureControl for RGBW {
    fn index(&self) -> usize {
        self.index
    }

    fn length(&self) -> usize {
        4
    }

    fn output(&mut self, time: f32) -> Vec<u8> {
        (self.output_fn)(time)
    }
}
