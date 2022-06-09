use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

use crate::fixture::FixtureControl;

pub trait SceneControl {
    type Fixture: FixtureControl;
    type Config: Serialize + DeserializeOwned + Debug;

    fn fixtures(&mut self) -> Vec<Self::Fixture>;
    fn play(&mut self, time: f32);

    fn write(&mut self, dmx: &mut Vec<u8>) {
        let fixtures = self.fixtures();
        for mut fixture in fixtures {
            fixture.write(dmx);
        }
    }
}
