pub use crate::fixtures::moving_head::{MovingHead, MovingHeadValue};
pub use crate::fixtures::rgbw::{Rgbw, RgbwValue};

pub mod moving_head;
pub mod rgbw;

use std::cell::RefCell;
use std::rc::Rc;

pub enum Fixture {
    MovingHead(Rc<RefCell<MovingHead>>),
    Rgbw(Rc<RefCell<Rgbw>>),
}
