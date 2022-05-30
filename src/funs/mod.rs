use std::f32::consts::PI;

use palette::{convert::IntoColor, Hsl, Srgb};

/* rgb */

pub fn rgb_rainbow_fun(time: f32, hue_mult: f32) -> Srgb<f32> {
    let hue = time * hue_mult;
    let color = Hsl::new(hue, 1_f32, 0.5_f32);
    color.into_color()
}

pub fn rgb_line_rainbow_fun(time: f32, position: f32, hue_speed: f32, hue_range: f32) -> Srgb<f32> {
    let hue = (time * hue_speed) + (position * hue_range * 360_f32);
    let color = Hsl::new(hue, 1_f32, 0.5_f32);
    color.into_color()
}

/* moving head */

pub fn moving_head_flower_fun(time: f32) -> (f32, f32, f32) {
    let pan = ((time / PI / 4_f32).sin() + 1_f32) / 2_f32;
    let tilt = ((PI - time / PI / 1_f32).sin() + 1_f32) / 2_f32;
    let speed = 0.5_f32;

    (pan, tilt, speed)
}

pub fn moving_head_line_flower_fun(
    time: f32,
    position: f32,
    pan_speed: f32,
    pan_range: f32,
    tilt_speed: f32,
    tilt_range: f32,
) -> (f32, f32, f32) {
    let pan = f32_line_sin_fun(time, position, pan_speed, pan_range, 1_f32);
    let tilt = f32_line_sin_fun(time, position, tilt_speed, tilt_range, 1_f32);
    let speed = 0.5_f32;

    (pan, tilt, speed)
}

/* float */

pub fn f32_sin_fun(time: f32, mult: f32, max: f32) -> f32 {
    (((time / mult).sin() + 1_f32) / 2_f32) * max
}

pub fn f32_line_sin_fun(time: f32, position: f32, speed: f32, range: f32, max: f32) -> f32 {
    let time_offset = 2_f32 * PI * position * range;
    let shifted_time = (time * speed) + time_offset;
    ((shifted_time.sin() + 1_f32) / 2_f32) * max
}

pub fn f32_ramp_fun(time: f32, mult: f32, max: f32) -> f32 {
    (time * mult) % max
}
