use std::f32::consts::PI;

use vibenet::{
    fixtures::{Fixture, MovingHead, RGBW},
    funs::{
        osc_white_fun, rainbow_rgb_fun, time_offset_f32, time_offset_f32_triplet, time_offset_u8,
        time_offset_u8_triplet,
    },
    net::VibeNet,
};

fn main() {
    let hue_mult = 11_f32;
    let white_mult = 0.7_f32;
    let rgb_offset_mult = 20_f32 / hue_mult;
    let white_offset_mult = 20_f32 / hue_mult;

    let position_fun = |time: f32| {
        let pan = ((time / PI / 4_f32).sin() + 1_f32) / 2_f32;
        let tilt = ((PI - time / PI / 1_f32).sin() + 1_f32) / 2_f32;
        let speed = 0.5_f32;

        (pan, tilt, speed)
    };
    let logged_position_fun = |fun: Box<dyn Fn(f32) -> (f32, f32, f32)>| {
        Box::new(move |time: f32| {
            let (pan, tilt, speed) = fun(time);

            println!("position: {}, {}", pan, tilt);

            (pan, tilt, speed)
        })
    };
    let color_wheel_fun = |time: f32| ((time * 4_f32) % 128_f32) as u8;
    let gobo_wheel_fun = |time: f32| ((time * 4_f32) % 128_f32) as u8;

    let fixtures = vec![
        Fixture::from(RGBW {
            index: 0,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 0_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 0_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 4,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 1_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 1_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 8,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 2_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 2_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 12,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 3_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 3_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 16,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 4_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 4_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 20,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 5_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 5_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 24,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 6_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 6_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 28,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 7_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 7_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 32,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 8_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 8_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 36,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 9_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 9_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 40,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 10_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 10_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 44,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 11_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 11_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 56,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 12_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 12_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 60,
            rgb_fun: time_offset_u8_triplet(rainbow_rgb_fun(hue_mult), 13_f32 * rgb_offset_mult),
            white_fun: time_offset_u8(osc_white_fun(white_mult), 13_f32 * white_offset_mult),
        }),
        Fixture::from(MovingHead {
            index: 100,
            position_fun: logged_position_fun(time_offset_f32_triplet(
                Box::new(position_fun),
                0_f32,
            )),
            color_wheel_fun: Box::new(color_wheel_fun),
            gobo_wheel_fun: Box::new(gobo_wheel_fun),
        }),
        Fixture::from(MovingHead {
            index: 111,
            position_fun: time_offset_f32_triplet(Box::new(position_fun), 1_f32 * PI),
            color_wheel_fun: Box::new(color_wheel_fun),
            gobo_wheel_fun: Box::new(gobo_wheel_fun),
        }),
        Fixture::from(MovingHead {
            index: 122,
            position_fun: time_offset_f32_triplet(Box::new(position_fun), 2_f32 * PI),
            color_wheel_fun: Box::new(color_wheel_fun),
            gobo_wheel_fun: Box::new(gobo_wheel_fun),
        }),
        Fixture::from(MovingHead {
            index: 133,
            position_fun: time_offset_f32_triplet(Box::new(position_fun), 3_f32 * PI),
            color_wheel_fun: Box::new(color_wheel_fun),
            gobo_wheel_fun: Box::new(gobo_wheel_fun),
        }),
        Fixture::from(MovingHead {
            index: 144,
            position_fun: time_offset_f32_triplet(Box::new(position_fun), 4_f32 * PI),
            color_wheel_fun: Box::new(color_wheel_fun),
            gobo_wheel_fun: Box::new(gobo_wheel_fun),
        }),
    ];

    let mut net = VibeNet::new(fixtures);
    net.connect().unwrap();
    net.artnet_output().unwrap();
}
