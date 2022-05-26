use std::f32::consts::PI;
use std::fmt::Debug;

use palette::{convert::IntoColor, Hsl, Srgb};

/* rgb */

type Rgb = (u8, u8, u8);
type RgbFun = dyn Fn(f32) -> Rgb;

pub fn rainbow_rgb_fun(hue_mult: f32) -> Box<RgbFun> {
    Box::new(move |time: f32| {
        let hue = time * hue_mult;
        let color = Hsl::new(hue, 1_f32, 0.5_f32);
        let rgb_f: Srgb = color.into_color();
        let rgb_u: Srgb<u8> = rgb_f.into_format();
        let (r, g, b) = rgb_u.into_components();

        (r, g, b)
    })
}

/* moving head */

type Position = (f32, f32, f32);
type PositionFun = dyn Fn(f32) -> Position;

pub fn position_fun() -> Box<PositionFun> {
    Box::new(move |time: f32| {
        let pan = ((time / PI / 4_f32).sin() + 1_f32) / 2_f32;
        let tilt = ((PI - time / PI / 1_f32).sin() + 1_f32) / 2_f32;
        let speed = 0.5_f32;

        (pan, tilt, speed)
    })
}

/* u8 */

type U8Fun = dyn Fn(f32) -> u8;

pub fn sin_u8_fun(mult: f32, max: f32) -> Box<U8Fun> {
    Box::new(move |time: f32| ((((time / mult).sin() + 1_f32) / 2_f32) * max) as u8)
}

pub fn ramp_u8_fun(mult: f32, max: f32) -> Box<U8Fun> {
    Box::new(move |time: f32| ((time * mult) % max) as u8)
}

/* utils */

pub fn time_offset<T: 'static>(fun: Box<dyn Fn(f32) -> T>, offset: f32) -> Box<dyn Fn(f32) -> T> {
    Box::new(move |time: f32| fun(time + offset))
}

pub fn logged_fun<T: Debug + 'static>(fun: Box<dyn Fn(f32) -> T>) -> Box<dyn Fn(f32) -> T> {
    Box::new(move |time: f32| {
        let value = fun(time);

        println!("value: {:?}", value);

        value
    })
}
