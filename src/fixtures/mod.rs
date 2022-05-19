use enum_dispatch::enum_dispatch;

use crate::fixture::FixtureControl;
pub use crate::fixtures::moving_head::MovingHead;
pub use crate::fixtures::rgbw::RGBW;

pub mod moving_head;
pub mod rgbw;

#[enum_dispatch(FixtureControl)]
#[derive()]
pub enum Fixture {
    RGBW(RGBW),
    MovingHead(MovingHead),
}
