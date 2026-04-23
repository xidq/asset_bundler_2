use std::num::NonZeroU32;

pub fn oblicz_ilosc_mipmap(width: u32, height: u32) -> NonZeroU32 {
    let poziomy = (width.max(height) as f32).log2().floor() as u32 + 1;
    NonZeroU32::new(poziomy).unwrap()
}
