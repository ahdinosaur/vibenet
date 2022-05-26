use std::f32::consts::PI;
use std::rc::Rc;

use vibenet::{
    fixtures::{Fixture, MovingHead, RgbwLine, RgbwLineConfig},
    funs::{
        logged_fun, position_fun, rainbow_rgb_line_fun, ramp_u8_fun, sin_u8_line_fun, time_offset,
    },
    net::VibeNet,
};

fn main() {
    let rgbw_line_addresses = vec![1, 5, 9, 13, 17, 21, 25, 29, 33, 37, 41, 45, 57, 61];
    let hue_speed = 11_f32;
    let hue_range = 0.5_f32;
    let rgbw_line_rgb_fun = rainbow_rgb_line_fun(hue_speed, hue_range);
    let white_speed = 0.7_f32;
    let white_range = 0.2_f32;
    let white_max = 128_f32;
    let rgbw_line_white_fun = sin_u8_line_fun(white_speed, white_range, white_max);

    let color_wheel_mult = 4_f32;
    let color_wheel_max = 128_f32;
    let gobo_wheel_mult = 4_f32;
    let gobo_wheel_max = 128_f32;

    let fixtures = vec![
        Fixture::from(RgbwLine::new(
            rgbw_line_addresses,
            RgbwLineConfig::Functional {
                rgb_fun: Rc::new(rgbw_line_rgb_fun),
                white_fun: Rc::new(rgbw_line_white_fun),
            },
        )),
        Fixture::from(MovingHead {
            index: 100,
            position_fun: logged_fun(time_offset(position_fun(), 0_f32)),
            color_wheel_fun: ramp_u8_fun(color_wheel_mult, color_wheel_max),
            gobo_wheel_fun: ramp_u8_fun(gobo_wheel_mult, gobo_wheel_max),
        }),
        Fixture::from(MovingHead {
            index: 111,
            position_fun: time_offset(position_fun(), 1_f32 * PI),
            color_wheel_fun: ramp_u8_fun(color_wheel_mult, color_wheel_max),
            gobo_wheel_fun: ramp_u8_fun(gobo_wheel_mult, gobo_wheel_max),
        }),
        Fixture::from(MovingHead {
            index: 122,
            position_fun: time_offset(position_fun(), 2_f32 * PI),
            color_wheel_fun: ramp_u8_fun(color_wheel_mult, color_wheel_max),
            gobo_wheel_fun: ramp_u8_fun(gobo_wheel_mult, gobo_wheel_max),
        }),
        Fixture::from(MovingHead {
            index: 133,
            position_fun: time_offset(position_fun(), 3_f32 * PI),
            color_wheel_fun: ramp_u8_fun(color_wheel_mult, color_wheel_max),
            gobo_wheel_fun: ramp_u8_fun(gobo_wheel_mult, gobo_wheel_max),
        }),
        Fixture::from(MovingHead {
            index: 144,
            position_fun: time_offset(position_fun(), 4_f32 * PI),
            color_wheel_fun: ramp_u8_fun(color_wheel_mult, color_wheel_max),
            gobo_wheel_fun: ramp_u8_fun(gobo_wheel_mult, gobo_wheel_max),
        }),
    ];

    let mut net = VibeNet::new(fixtures);
    net.connect().unwrap();
    net.artnet_output().unwrap();
}
