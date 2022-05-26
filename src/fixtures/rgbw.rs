use crate::fixture::FixtureControl;
use std::rc::Rc;

#[derive()]
pub struct Rgbw {
    pub address: usize,
    pub rgb_fun: Box<dyn Fn(f32) -> (u8, u8, u8)>,
    pub white_fun: Box<dyn Fn(f32) -> u8>,
}

impl FixtureControl for Rgbw {
    fn channels(&self) -> Box<dyn Iterator<Item = usize>> {
        let index = self.address - 1;
        Box::new(index..(index + 4))
    }

    fn output(&mut self, time: f32) -> Box<dyn Iterator<Item = u8>> {
        let (r, g, b) = (self.rgb_fun)(time);
        let w = (self.white_fun)(time);

        Box::new(vec![r, g, b, w].into_iter())
    }
}

#[derive()]
pub struct RgbwLine {
    pub addresses: Vec<usize>,
    pub rgb_fun: Rc<dyn Fn(f32, f32) -> (u8, u8, u8)>,
    pub white_fun: Rc<dyn Fn(f32, f32) -> u8>,
}

impl RgbwLine {
    pub fn new(
        addresses: Vec<usize>,
        rgb_fun: impl Fn(f32, f32) -> (u8, u8, u8) + 'static,
        white_fun: impl Fn(f32, f32) -> u8 + 'static,
    ) -> Self {
        Self {
            addresses,
            rgb_fun: Rc::new(rgb_fun),
            white_fun: Rc::new(white_fun),
        }
    }
}

impl FixtureControl for RgbwLine {
    fn channels(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(
            self.addresses
                .clone()
                .into_iter()
                .map(|address| {
                    let index = address - 1;

                    index..(index + 4)
                })
                .flatten(),
        )
    }

    fn output(&mut self, time: f32) -> Box<dyn Iterator<Item = u8>> {
        let addresses = self.addresses.clone();
        let rgb_fun = self.rgb_fun.clone();
        let white_fun = self.white_fun.clone();

        let length = addresses.len() as f32;

        Box::new(
            addresses
                .into_iter()
                .map(move |address| {
                    let index = (address - 1) as f32;
                    let position = index / length;

                    let (r, g, b) = rgb_fun(time, position);
                    let w = white_fun(time, position);

                    vec![r, g, b, w]
                })
                .flatten(),
        )
    }
}
