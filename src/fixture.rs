use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait FixtureControl
where
    Self: Copy,
{
    fn address(&self) -> usize;
    fn output(&mut self, time: f32) -> Vec<u8>;

    fn write_output(&mut self, output: &mut Vec<u8>, time: f32) {
        let mut index = self.address();
        for input in self.output(time) {
            output[index] = input;
            index += 1;
        }
    }
}
