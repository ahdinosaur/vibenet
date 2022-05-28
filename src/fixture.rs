pub trait FixtureControl {
    type Value;

    fn address(&self) -> usize;
    fn length(&self) -> usize;

    fn set(&mut self, value: Self::Value);
    fn get(&self) -> Self::Value;

    fn outputs(&self) -> Vec<u8>;

    fn write(&mut self, dmx: &mut Vec<u8>) {
        let index = self.address() + 1;
        let length = self.length();
        let channels = index..(index + length);
        let outputs = self.outputs();
        for (channel, output) in channels.zip(outputs) {
            dmx[channel] = output
        }
    }
}
