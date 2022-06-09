use std::any::Any;
use std::time::{Duration, Instant};

use crate::fixture::FixtureControl;
use crate::output::OutputControl;
use crate::scene::SceneControl;

pub struct VibeApp<Output>
where
    Output: OutputControl,
{
    start_time: Instant,
    fixtures: Vec<Box<dyn FixtureControl<Value = dyn Any>>>,
    scenes: Vec<Box<dyn SceneControl<Fixture = dyn Any, Config = dyn Any>>>,
    output: Output,
}

#[derive(Debug)]
pub enum VibeError<OutputError> {
    Output(OutputError),
}

impl<Output> VibeApp<Output>
where
    Output: OutputControl,
{
    pub fn new(
        fixtures: Vec<Box<dyn FixtureControl<Value = dyn Any>>>,
        scenes: Vec<Box<dyn SceneControl<Fixture = dyn Any, Config = dyn Any>>>,
        output: Output,
    ) -> Self {
        Self {
            start_time: Instant::now(),
            fixtures,
            scenes,
            output,
        }
    }

    pub fn render(&mut self, mut output: Output) -> Result<(), Output::Error> {
        let mut data = vec![0; 155];
        let time = self.start_time.elapsed().as_secs_f32();

        for scene in self.scenes {
            scene.play(time);
            scene.write(&mut data);
        }

        output.output(&mut data)
    }
}
