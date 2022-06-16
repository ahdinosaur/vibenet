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
    data: Vec<u8>,
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
        let max_fixture_channel = fixtures
            .clone()
            .into_iter()
            .flat_map(|fixture| fixture.channels())
            .reduce(|sofar, next| if sofar >= next { sofar } else { next })
            .expect("Unexpected no max fixture channel");
        let data = vec![0; max_fixture_channel + 1];

        Self {
            start_time: Instant::now(),
            fixtures,
            scenes,
            output,
            data,
        }
    }

    pub fn render(&mut self) -> Result<(), Output::Error> {
        let data = &mut self.data;
        let time = self.start_time.elapsed().as_secs_f32();

        for mut scene in self.scenes.clone() {
            scene.play(time);
            scene.write(data);
        }

        self.output.output(data)
    }
}
