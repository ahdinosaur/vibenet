use std::f32::consts::PI;

use vibenet::{
    fixtures::{Fixture, MovingHead, RGBW},
    funs::{logged_fun, position_fun, rainbow_rgb_fun, ramp_u8_fun, sin_u8_fun, time_offset},
    net::VibeNet,
};

fn main() {
    let hue_mult = 11_f32;
    let white_mult = 0.7_f32;
    let rgb_offset_mult = 20_f32 / hue_mult;
    let white_offset_mult = 20_f32 / hue_mult;
    let white_max = 128_f32;

    let color_wheel_mult = 4_f32;
    let color_wheel_max = 128_f32;
    let gobo_wheel_mult = 4_f32;
    let gobo_wheel_max = 128_f32;

    let fixtures = vec![
        Fixture::from(RGBW {
            index: 0,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 0_f32 * rgb_offset_mult),
            white_fun: time_offset(sin_u8_fun(white_mult, white_max), 0_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 4,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 1_f32 * rgb_offset_mult),
            white_fun: time_offset(sin_u8_fun(white_mult, white_max), 1_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 8,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 2_f32 * rgb_offset_mult),
            white_fun: time_offset(sin_u8_fun(white_mult, white_max), 2_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 12,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 3_f32 * rgb_offset_mult),
            white_fun: time_offset(sin_u8_fun(white_mult, white_max), 3_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 16,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 4_f32 * rgb_offset_mult),
            white_fun: time_offset(sin_u8_fun(white_mult, white_max), 4_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 20,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 5_f32 * rgb_offset_mult),
            white_fun: time_offset(sin_u8_fun(white_mult, white_max), 5_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 24,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 6_f32 * rgb_offset_mult),
            white_fun: time_offset(sin_u8_fun(white_mult, white_max), 6_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 28,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 7_f32 * rgb_offset_mult),
            white_fun: time_offset(sin_u8_fun(white_mult, white_max), 7_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 32,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 8_f32 * rgb_offset_mult),
            white_fun: time_offset(sin_u8_fun(white_mult, white_max), 8_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 36,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 9_f32 * rgb_offset_mult),
            white_fun: time_offset(sin_u8_fun(white_mult, white_max), 9_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 40,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 10_f32 * rgb_offset_mult),
            white_fun: time_offset(
                sin_u8_fun(white_mult, white_max),
                10_f32 * white_offset_mult,
            ),
        }),
        Fixture::from(RGBW {
            index: 44,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 11_f32 * rgb_offset_mult),
            white_fun: time_offset(
                sin_u8_fun(white_mult, white_max),
                11_f32 * white_offset_mult,
            ),
        }),
        Fixture::from(RGBW {
            index: 56,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 12_f32 * rgb_offset_mult),
            white_fun: time_offset(
                sin_u8_fun(white_mult, white_max),
                12_f32 * white_offset_mult,
            ),
        }),
        Fixture::from(RGBW {
            index: 60,
            rgb_fun: time_offset(rainbow_rgb_fun(hue_mult), 13_f32 * rgb_offset_mult),
            white_fun: time_offset(
                sin_u8_fun(white_mult, white_max),
                13_f32 * white_offset_mult,
            ),
        }),
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
