use image::DynamicImage;
use enumy::rozszerzenia::kolor::ColorProfilePhoto;

pub struct DaneDoWczytywania{
    pub dane: DynamicImage,
    pub exif: Option<Vec<u8>>,
    pub kolor: ColorProfilePhoto,
}