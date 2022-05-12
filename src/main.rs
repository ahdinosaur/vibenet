use vibenet::{
    fixtures::{Fixture, RGBW},
    funs::{osc_white_fun, rainbow_rgb_fun, time_offset_rgb, time_offset_white},
    net::VibeNet,
};

fn main() {
    let hue_mult = 11_f32;
    let white_mult = 0.7_f32;
    let rgb_offset_mult = 20_f32 / hue_mult;
    let white_offset_mult = 20_f32 / hue_mult;

    let fixtures = vec![
        Fixture::from(RGBW {
            index: 0,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 0_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 0_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 4,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 1_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 1_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 8,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 2_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 2_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 12,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 3_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 3_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 16,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 4_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 4_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 20,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 5_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 5_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 24,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 6_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 6_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 28,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 7_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 7_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 32,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 8_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 8_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 36,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 9_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 9_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 40,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 10_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 10_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 44,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 11_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 11_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 57,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 12_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 12_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 61,
            rgb_fun: time_offset_rgb(rainbow_rgb_fun(hue_mult), 13_f32 * rgb_offset_mult),
            white_fun: time_offset_white(osc_white_fun(white_mult), 13_f32 * white_offset_mult),
        }),
    ];

    let mut net = VibeNet::new(fixtures);
    net.connect().unwrap();
    net.artnet_output().unwrap();
}
