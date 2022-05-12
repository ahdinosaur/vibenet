use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait FixtureControl {
    fn index(&self) -> usize;
    fn length(&self) -> usize;
    fn output(&mut self, time: f32) -> Vec<u8>;

    fn write_output(&mut self, output: &mut Vec<u8>, time: f32) {
        let mut index = self.index();
        let length = self.length();
        for input in self.output(time).into_iter().take(length) {
            output[index] = input;
            index += 1;
        }
    }
}
