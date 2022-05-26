use enum_dispatch::enum_dispatch;

use crate::fixture::FixtureControl;
pub use crate::fixtures::moving_head::MovingHead;
pub use crate::fixtures::rgbw::{Rgbw, RgbwConfig, RgbwLine, RgbwLineConfig};

pub mod moving_head;
pub mod rgbw;

#[enum_dispatch(FixtureControl)]
#[derive()]
pub enum Fixture {
    Rgbw(Rgbw),
    RgbwLine(RgbwLine),
    MovingHead(MovingHead),
}
