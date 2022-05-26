use crate::fixture::FixtureControl;
use std::rc::Rc;

pub enum RgbwConfig {
    Manual {
        red: u8,
        green: u8,
        blue: u8,
        white: u8,
    },
    Functional {
        rgb_fun: Box<dyn Fn(f32) -> (u8, u8, u8)>,
        white_fun: Box<dyn Fn(f32) -> u8>,
    },
}

pub struct Rgbw {
    address: usize,
    config: RgbwConfig,
}

impl Rgbw {
    pub fn new(address: usize, config: RgbwConfig) -> Self {
        Self { address, config }
    }
}

impl FixtureControl for Rgbw {
    fn channels(&self) -> Box<dyn Iterator<Item = usize>> {
        let index = self.address - 1;
        Box::new(index..(index + 4))
    }

    fn output(&mut self, time: f32) -> Box<dyn Iterator<Item = u8>> {
        match &self.config {
            RgbwConfig::Manual {
                red,
                green,
                blue,
                white,
            } => Box::new(vec![*red, *green, *blue, *white].into_iter()),

            RgbwConfig::Functional { rgb_fun, white_fun } => {
                let (red, green, blue) = rgb_fun(time);
                let white = white_fun(time);

                Box::new(vec![red, green, blue, white].into_iter())
            }
        }
    }
}

#[derive(Clone)]
pub enum RgbwLineConfig {
    Manual {
        red: u8,
        green: u8,
        blue: u8,
        white: u8,
    },
    Functional {
        rgb_fun: Rc<dyn Fn(f32, f32) -> (u8, u8, u8)>,
        white_fun: Rc<dyn Fn(f32, f32) -> u8>,
    },
}

pub struct RgbwLine {
    addresses: Vec<usize>,
    config: RgbwLineConfig,
}

impl RgbwLine {
    pub fn new(addresses: Vec<usize>, config: RgbwLineConfig) -> Self {
        Self { addresses, config }
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

        match self.config.clone() {
            RgbwLineConfig::Manual {
                red,
                green,
                blue,
                white,
            } => Box::new(
                addresses
                    .into_iter()
                    .map(move |_| vec![red, green, blue, white])
                    .flatten(),
            ),

            RgbwLineConfig::Functional { rgb_fun, white_fun } => {
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
    }
}
