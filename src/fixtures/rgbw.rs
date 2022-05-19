use crate::fixture::FixtureControl;

#[derive()]
pub struct RGBW {
    pub index: usize,
    pub rgb_fun: Box<dyn Fn(f32) -> (u8, u8, u8)>,
    pub white_fun: Box<dyn Fn(f32) -> u8>,
}

impl FixtureControl for RGBW {
    fn channels(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(self.index..(self.index + 4))
    }

    fn output(&mut self, time: f32) -> Box<dyn Iterator<Item = u8>> {
        let (r, g, b) = (self.rgb_fun)(time);
        let w = (self.white_fun)(time);

        Box::new(vec![r, g, b, w].into_iter())
    }
}
