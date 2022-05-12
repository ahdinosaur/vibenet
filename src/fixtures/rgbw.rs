use crate::fixture::FixtureControl;

#[derive()]
pub struct RGBW {
    pub index: usize,
    pub rgb_fun: Box<dyn Fn(f32) -> (u8, u8, u8)>,
    pub white_fun: Box<dyn Fn(f32) -> u8>,
}

impl FixtureControl for RGBW {
    fn index(&self) -> usize {
        self.index
    }

    fn length(&self) -> usize {
        4
    }

    fn output(&mut self, time: f32) -> Vec<u8> {
        let (r, g, b) = (self.rgb_fun)(time);
        let w = (self.white_fun)(time);

        vec![r, g, b, w]
    }
}
