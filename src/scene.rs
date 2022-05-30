use crate::fixture::FixtureControl;

pub trait SceneControl {
    type Fixture: FixtureControl;

    fn fixtures(&mut self) -> Vec<Self::Fixture>;
    fn set(&mut self, time: f32);

    fn write(&mut self, dmx: &mut Vec<u8>) {
        let fixtures = self.fixtures();
        for mut fixture in fixtures {
            fixture.write(dmx);
        }
    }
}
