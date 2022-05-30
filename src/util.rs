pub fn f32_to_u8(value: f32) -> u8 {
    (value * (u8::MAX as f32)) as u8
}

pub fn f32_to_double_u8(value: f32) -> (u8, u8) {
    let coarse = f32_to_u8(value);
    let fine = (((value * 256_f32) % 1_f32) * 256_f32) as u8;
    (coarse, fine)
}
