use std::cell::RefCell;
use std::rc::Rc;

pub trait FixtureControl {
    type Value;

    fn address(&self) -> usize;
    fn length(&self) -> usize;

    fn set(&mut self, value: Self::Value);
    fn get(&self) -> Self::Value;

    fn outputs(&self) -> Vec<u8>;

    fn write(&mut self, dmx: &mut Vec<u8>) {
        let index = self.address() - 1;
        let length = self.length();
        let channels = index..(index + length);
        let outputs = self.outputs();
        for (channel, output) in channels.zip(outputs) {
            dmx[channel] = output
        }
    }
}

impl<Fixture> FixtureControl for Rc<RefCell<Fixture>>
where
    Fixture: FixtureControl,
{
    type Value = <Fixture as FixtureControl>::Value;

    fn address(&self) -> usize {
        self.borrow().address()
    }

    fn length(&self) -> usize {
        self.borrow().length()
    }

    fn set(&mut self, value: Self::Value) {
        self.borrow_mut().set(value)
    }

    fn get(&self) -> Self::Value {
        self.borrow().get()
    }

    fn outputs(&self) -> Vec<u8> {
        self.borrow().outputs()
    }
}
