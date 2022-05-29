use std::cell::RefCell;
use std::sync::Arc;

use vibenet::{
    fixtures::{MovingHead, Rgbw},
    net::VibeNet,
    scene::SceneControl,
    scenes::{MovingHeadFlower, RgbwRainbow},
};

fn main() {
    let rgbw_fixtures: Vec<Arc<RefCell<Rgbw>>> = vec![
        Rgbw::new(1),
        Rgbw::new(5),
        Rgbw::new(9),
        Rgbw::new(13),
        Rgbw::new(17),
        Rgbw::new(21),
        Rgbw::new(25),
        Rgbw::new(29),
        Rgbw::new(33),
        Rgbw::new(37),
        Rgbw::new(41),
        Rgbw::new(45),
        Rgbw::new(57),
        Rgbw::new(61),
    ]
    .into_iter()
    .map(|fixture| Arc::new(RefCell::new(fixture)))
    .collect();

    let moving_head_fixtures: Vec<Arc<RefCell<MovingHead>>> = vec![
        MovingHead::new(101),
        MovingHead::new(112),
        MovingHead::new(123),
        MovingHead::new(134),
        MovingHead::new(145),
    ]
    .into_iter()
    .map(|fixture| Arc::new(RefCell::new(fixture)))
    .collect();

    let hue_speed = 11_f32;
    let hue_range = 0.5_f32;
    let white_speed = 0.7_f32;
    let white_range = 0.2_f32;
    let white_max = 0.5_f32;

    let mut rgb_rainbow_scene = RgbwRainbow {
        fixtures: rgbw_fixtures,
        hue_speed,
        hue_range,
        white_speed,
        white_range,
        white_max,
    };

    let pan_speed = 0.4_f32;
    let pan_range = 1_f32;
    let tilt_speed = 0.1_f32;
    let tilt_range = 1_f32;
    let color_wheel_mult = 4_f32;
    let color_wheel_max = 0.5_f32;
    let gobo_wheel_mult = 4_f32;
    let gobo_wheel_max = 0.5_f32;

    let mut moving_head_flower = MovingHeadFlower {
        fixtures: moving_head_fixtures,
        pan_speed,
        pan_range,
        tilt_speed,
        tilt_range,
        color_wheel_mult,
        color_wheel_max,
        gobo_wheel_mult,
        gobo_wheel_max,
    };

    let scene_controller = move |time: f32, dmx: &mut Vec<u8>| {
        rgb_rainbow_scene.set(time);
        rgb_rainbow_scene.write(dmx);

        moving_head_flower.set(time);
        moving_head_flower.write(dmx);
    };

    let mut net = VibeNet::new(scene_controller);
    net.connect().unwrap();
    net.artnet_output().unwrap();
}
