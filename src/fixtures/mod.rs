pub use crate::fixtures::moving_head::{MovingHead, MovingHeadValue};
pub use crate::fixtures::rgbw::{Rgbw, RgbwValue};

pub mod moving_head;
pub mod rgbw;

use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use crate::fixture::FixtureControl;

#[derive(Clone, Debug)]
pub enum Fixture {
    MovingHead(Rc<RefCell<MovingHead>>),
    Rgbw(Rc<RefCell<Rgbw>>),
}

impl Fixture {
    pub fn channels(&self) -> Range<usize> {
        match self {
            Fixture::MovingHead(fixture) => fixture.channels(),
            Fixture::Rgbw(fixture) => fixture.channels(),
        }
    }
}
