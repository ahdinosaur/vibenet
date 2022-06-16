pub use crate::scenes::moving_head_flower::{MovingHeadFlower, MovingHeadFlowerConfig};
pub use crate::scenes::rgbw_rainbow::{RgbwRainbow, RgbwRainbowConfig};

pub mod moving_head_flower;
pub mod rgbw_rainbow;

use crate::scene::SceneControl;

#[derive(Clone, Debug)]
pub enum Scene {
    MovingHeadFlower(MovingHeadFlower),
    RgbwRainbow(RgbwRainbow),
}

impl Scene {
    pub fn play(&mut self, time: f32) {
        match self {
            Scene::MovingHeadFlower(scene) => scene.play(time),
            Scene::RgbwRainbow(scene) => scene.play(time),
        }
    }

    pub fn write(&mut self, dmx: &mut Vec<u8>) {
        match self {
            Scene::MovingHeadFlower(scene) => scene.write(dmx),
            Scene::RgbwRainbow(scene) => scene.write(dmx),
        }
    }
}
