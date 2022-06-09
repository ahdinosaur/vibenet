pub trait OutputControl {
    type Error;

    fn setup(&mut self) -> Result<(), Self::Error>;
    fn output(&mut self, dmx: &mut Vec<u8>) -> Result<(), Self::Error>;
}
