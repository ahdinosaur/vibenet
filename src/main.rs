use palette::{convert::IntoColor, Hsl, Srgb};

use vibenet::{
    fixtures::{Fixture, RGBW},
    net::VibeNet,
};

fn main() {
    let hue_mult = 11_f32;
    let white_mult = 0.7_f32;
    let rainbow_rgb_fun = move |time: f32| {
        let hue = time * hue_mult;
        let color = Hsl::new(hue, 1_f32, 0.5_f32);
        let rgb_f: Srgb = color.into_color();
        let rgb_u: Srgb<u8> = rgb_f.into_format();
        let (r, g, b) = rgb_u.into_components();

        (r, g, b)
    };
    let osc_white_fun =
        move |time: f32| ((((time / white_mult).sin() + 1_f32) / 2_f32) * 64_f32) as u8;

    let time_offset_triplet = |fun: Box<dyn Fn(f32) -> (u8, u8, u8)>, offset: f32| {
        Box::new(move |time: f32| fun(time + offset))
    };
    let time_offset_byte =
        |fun: Box<dyn Fn(f32) -> u8>, offset: f32| Box::new(move |time: f32| fun(time + offset));
    let rgb_offset_mult = 20_f32 / hue_mult;
    let white_offset_mult = 20_f32 / hue_mult;

    let fixtures = vec![
        Fixture::from(RGBW {
            index: 0,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 0_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 0_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 4,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 1_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 1_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 8,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 2_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 2_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 12,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 3_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 3_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 16,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 4_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 4_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 20,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 5_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 5_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 24,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 6_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 6_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 28,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 7_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 7_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 32,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 8_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 8_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 36,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 9_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 9_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 40,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 10_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 10_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 44,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 11_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 11_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 57,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 12_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 12_f32 * white_offset_mult),
        }),
        Fixture::from(RGBW {
            index: 61,
            rgb_fun: time_offset_triplet(Box::new(rainbow_rgb_fun), 13_f32 * rgb_offset_mult),
            white_fun: time_offset_byte(Box::new(osc_white_fun), 13_f32 * white_offset_mult),
        }),
    ];

    let mut net = VibeNet::new(fixtures);
    net.connect().unwrap();
    net.artnet_output().unwrap();
}
