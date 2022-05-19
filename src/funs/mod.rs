use palette::{convert::IntoColor, Hsl, Srgb};

type Rgb = (u8, u8, u8);
type RgbFun = dyn Fn(f32) -> Rgb;
type White = u8;
type WhiteFun = dyn Fn(f32) -> White;

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

pub fn osc_white_fun(white_mult: f32) -> Box<WhiteFun> {
    Box::new(move |time: f32| ((((time / white_mult).sin() + 1_f32) / 2_f32) * 64_f32) as u8)
}

pub fn time_offset_u8(fun: Box<dyn Fn(f32) -> White>, offset: f32) -> Box<dyn Fn(f32) -> u8> {
    Box::new(move |time: f32| fun(time + offset))
}

pub fn time_offset_u8_triplet(
    fun: Box<dyn Fn(f32) -> (u8, u8, u8)>,
    offset: f32,
) -> Box<dyn Fn(f32) -> (u8, u8, u8)> {
    Box::new(move |time: f32| fun(time + offset))
}

pub fn time_offset_f32(fun: Box<dyn Fn(f32) -> f32>, offset: f32) -> Box<dyn Fn(f32) -> f32> {
    Box::new(move |time: f32| fun(time + offset))
}

pub fn time_offset_f32_triplet(
    fun: Box<dyn Fn(f32) -> (f32, f32, f32)>,
    offset: f32,
) -> Box<dyn Fn(f32) -> (f32, f32, f32)> {
    Box::new(move |time: f32| fun(time + offset))
}
