use enum_dispatch::enum_dispatch;

use crate::fixture::FixtureControl;
pub use crate::fixtures::rgbw::RGBW;

pub mod rgbw;

#[enum_dispatch(FixtureControl)]
#[derive(Debug, Copy, Clone)]
pub enum Fixture<RGBWOutputFn>
where
    RGBWOutputFn: Fn(f32) -> Vec<u8> + Copy,
{
    RGBW(RGBW<RGBWOutputFn>),
}
