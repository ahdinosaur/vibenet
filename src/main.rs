use std::cell::RefCell;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use vibenet::{
    app::VibeApp,
    fixtures::{Fixture, MovingHead, Rgbw},
    output::OutputControl,
    outputs::Artnet,
    scenes::{MovingHeadFlower, MovingHeadFlowerConfig, RgbwRainbow, RgbwRainbowConfig, Scene},
};

fn main() {
    let rgbw_fixtures: Vec<Rc<RefCell<Rgbw>>> = vec![
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
    .map(|fixture| Rc::new(RefCell::new(fixture)))
    .collect();

    let moving_head_fixtures: Vec<Rc<RefCell<MovingHead>>> = vec![
        MovingHead::new(101),
        MovingHead::new(112),
        MovingHead::new(123),
        MovingHead::new(134),
        MovingHead::new(145),
    ]
    .into_iter()
    .map(|fixture| Rc::new(RefCell::new(fixture)))
    .collect();

    let hue_speed = 11_f32;
    let hue_range = 0.5_f32;
    let white_speed = 0.7_f32;
    let white_range = 0.2_f32;
    let white_max = 0.5_f32;

    let rgb_rainbow_scene = RgbwRainbow {
        fixtures: rgbw_fixtures.clone(),
        config: RgbwRainbowConfig {
            hue_speed,
            hue_range,
            white_speed,
            white_range,
            white_max,
        },
    };

    let pan_speed = 0.1_f32;
    let pan_range = 1_f32;
    let tilt_speed = 0.05_f32;
    let tilt_range = 1_f32;
    let color_wheel_mult = 0.05_f32;
    let color_wheel_max = 0.5_f32;
    let gobo_wheel_mult = 0.005_f32;
    let gobo_wheel_max = 0.5_f32;

    let moving_head_flower = MovingHeadFlower {
        fixtures: moving_head_fixtures.clone(),
        config: MovingHeadFlowerConfig {
            pan_speed,
            pan_range,
            tilt_speed,
            tilt_range,
            color_wheel_mult,
            color_wheel_max,
            gobo_wheel_mult,
            gobo_wheel_max,
        },
    };

    let fixtures = rgbw_fixtures
        .clone()
        .into_iter()
        .map(|rgbw| Fixture::Rgbw(rgbw))
        .chain(
            moving_head_fixtures
                .clone()
                .into_iter()
                .map(|moving_head| Fixture::MovingHead(moving_head)),
        )
        .collect();

    let scenes = vec![
        Scene::RgbwRainbow(rgb_rainbow_scene),
        Scene::MovingHeadFlower(moving_head_flower),
    ];

    let mut output = Artnet::new();

    output.setup().unwrap();

    let mut app = VibeApp::new(fixtures, scenes, output);

    loop {
        app.render().unwrap();

        sleep(Duration::from_millis(20));
    }
}
