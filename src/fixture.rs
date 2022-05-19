use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait FixtureControl {
    fn channels(&self) -> Box<dyn Iterator<Item = usize>>;
    fn output(&mut self, time: f32) -> Box<dyn Iterator<Item = u8>>;

    fn write_output(&mut self, output: &mut Vec<u8>, time: f32) {
        let channels = self.channels();
        let outputs = self.output(time);
        for (channel, input) in channels.zip(outputs) {
            output[channel] = input
        }
    }
}
