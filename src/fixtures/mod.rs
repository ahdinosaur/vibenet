use enum_dispatch::enum_dispatch;

use crate::fixture::FixtureControl;
pub use crate::fixtures::rgbw::RGBW;

pub mod rgbw;

#[enum_dispatch(FixtureControl)]
#[derive()]
pub enum Fixture {
    RGBW(RGBW),
}
