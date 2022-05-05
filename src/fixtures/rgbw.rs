use crate::fixture::FixtureControl;

#[derive(Debug, Copy, Clone)]
pub struct RGBW<OutputFn>
where
    OutputFn: Fn(f32) -> Vec<u8>,
{
    pub address: usize,
    pub output_fn: OutputFn,
}

impl<OutputFn> FixtureControl for RGBW<OutputFn> {
    fn address(&self) -> usize {
        self.address
    }

    fn output(&mut self, time: f32) -> Vec<u8> {
        self.output_fn(time)
    }
}
