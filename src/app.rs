use std::time::Instant;

use crate::fixtures::Fixture;
use crate::output::OutputControl;
use crate::scenes::Scene;

pub struct VibeApp<Output>
where
    Output: OutputControl,
{
    start_time: Instant,
    fixtures: Vec<Fixture>,
    scenes: Vec<Scene>,
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
    pub fn new(fixtures: Vec<Fixture>, scenes: Vec<Scene>, output: Output) -> Self {
        Self {
            start_time: Instant::now(),
            fixtures,
            scenes,
            output,
        }
    }

    pub fn render(&mut self) -> Result<(), Output::Error> {
        let mut data = vec![0; 155];
        let time = self.start_time.elapsed().as_secs_f32();

        for mut scene in self.scenes.clone() {
            scene.play(time);
            scene.write(&mut data);
        }

        self.output.output(&mut data)
    }
}
