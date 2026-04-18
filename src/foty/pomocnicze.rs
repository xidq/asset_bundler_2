use image::{ColorType, DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, LumaA, Rgb, Rgba};

pub fn usun_kanal_alpha(bufor:DynamicImage, alfa_rgb:(u16,u16,u16)) -> DynamicImage {


    let mapuj_u16_na_u8 = |v: u16| -> u8 {
        ((v as f32 / 65535.0) * 255.0).round() as u8
    };


    let (w, h) = (bufor.width(), bufor.height());
    let mut obraz_koncowy = bufor.clone();

    let usuniete_alpha:DynamicImage =
        if bufor.has_alpha(){
            match bufor.color(){
                ColorType::La8 => {
                    let mut nowy_bufor = image::ImageBuffer::new(w, h);
                    let tlo_r:u8 = mapuj_u16_na_u8(alfa_rgb.0);
                    let tlo_g:u8 = mapuj_u16_na_u8(alfa_rgb.1);
                    let tlo_b:u8 = mapuj_u16_na_u8(alfa_rgb.2);
                    let tlo_szare = (((tlo_b * tlo_g * tlo_r)as f32) / 3.).round() as u8 ;

                    for y in 0..obraz_koncowy.height(){
                        for x in 0..obraz_koncowy.width() {
                            let pikselek = obraz_koncowy.as_luma_alpha8().unwrap().get_pixel(x, y);

                            let l = if pikselek[1] < 255 {
                                 (tlo_szare as f32 * (1. - (pikselek[1] as f32 / 255.)) + (pikselek[0] as f32 * (pikselek[1] as f32 / 255.))) as u8

                            } else {
                                pikselek[0]
                            };
                            nowy_bufor.put_pixel(x, y, image::Luma([l]));
                        }
                    }
                    obraz_koncowy = image::DynamicImage::ImageLuma8(nowy_bufor);
                    obraz_koncowy
                },
                ColorType::Rgba8 => {
                    let mut nowy_bufor = image::ImageBuffer::new(w, h);
                    let tlo_r:u8 = mapuj_u16_na_u8(alfa_rgb.0);
                    let tlo_g:u8 = mapuj_u16_na_u8(alfa_rgb.1);
                    let tlo_b:u8 = mapuj_u16_na_u8(alfa_rgb.2);

                    for y in 0..obraz_koncowy.height(){
                        for x in 0..obraz_koncowy.width() {
                            let pikselek = obraz_koncowy.get_pixel(x, y);

                            let (r, g, b) = if pikselek[3] < 255{
                                let rgb_r = (tlo_r as f32 * (1. - (pikselek[3] as f32 / 255.)) + (pikselek[0] as f32 * (pikselek[3] as f32 / 255.))) as u8 ;
                                let rgb_g = (tlo_g as f32 * (1. - (pikselek[3] as f32 / 255.)) + (pikselek[1] as f32 * (pikselek[3] as f32 / 255.))) as u8 ;
                                let rgb_b = (tlo_b as f32 * (1. - (pikselek[3] as f32 / 255.)) + (pikselek[2] as f32 * (pikselek[3] as f32 / 255.))) as u8 ;
                                (rgb_r,rgb_g,rgb_b)
                            } else {
                                (pikselek[0], pikselek[1], pikselek[2])
                            };
                            nowy_bufor.put_pixel(x, y, image::Rgb([r, g, b]));


                        }
                    }
                    obraz_koncowy = image::DynamicImage::ImageRgb8(nowy_bufor);
                    obraz_koncowy
                },
                ColorType::La16 => {//ImageBuffer<Rgb<u16>, Vec<u16>>
                    let mut nowy_bufor:ImageBuffer<Luma<u16>, Vec<u16>> = image::ImageBuffer::new(w, h);
                    let tlo_szare = (((alfa_rgb.0 * alfa_rgb.1 * alfa_rgb.2)as f32) / 3.).round() as u8 ;

                    for y in 0..obraz_koncowy.height(){
                        for x in 0..obraz_koncowy.width() {
                            let pikselek: &LumaA<u16> = obraz_koncowy.as_luma_alpha16().unwrap().get_pixel(x, y);

                            let l:u16 = if pikselek[1] < 65535 {
                                let l:u16 = (tlo_szare as f32 * (1. - (pikselek[1] as f32 / 65535.)) + (pikselek[0] as f32 * (pikselek[1] as f32 / 65535.))) as u16;
                                l
                            } else {
                                pikselek[0]
                            };
                            nowy_bufor.put_pixel(x, y, image::Luma([l]));
                        }
                    }
                    obraz_koncowy = image::DynamicImage::ImageLuma16(nowy_bufor);
                    obraz_koncowy

                },
                ColorType::Rgba16 => {
                    let mut nowy_bufor:ImageBuffer<Rgb<u16>, Vec<u16>> = image::ImageBuffer::new(w, h);
                    for y in 0..obraz_koncowy.height(){
                        for x in 0..obraz_koncowy.width() {
                            let pikselek: &Rgba<u16> = obraz_koncowy.as_rgba16().unwrap().get_pixel(x, y);
                            // u16 ma max 65535
                            let (r, g, b) = if pikselek[3] < 65535{
                                let rgb_r = (alfa_rgb.0 as f32 * (1. - (pikselek[3] as f32 / 65535.)) + (pikselek[0] as f32 * (pikselek[3] as f32 / 65535.))) as u16 ;
                                let rgb_g = (alfa_rgb.1 as f32 * (1. - (pikselek[3] as f32 / 65535.)) + (pikselek[1] as f32 * (pikselek[3] as f32 / 65535.))) as u16 ;
                                let rgb_b = (alfa_rgb.2 as f32 * (1. - (pikselek[3] as f32 / 65535.)) + (pikselek[2] as f32 * (pikselek[3] as f32 / 65535.))) as u16 ;
                                (rgb_r,rgb_g,rgb_b)
                            } else {
                                (pikselek[0], pikselek[1], pikselek[2])
                            };
                            nowy_bufor.put_pixel(x, y, image::Rgb([r, g, b]));

                        }
                    }

                    obraz_koncowy = image::DynamicImage::ImageRgb16(nowy_bufor);
                    obraz_koncowy
                },
                ColorType::Rgba32F => {
                    obraz_koncowy
                },
                _ => obraz_koncowy
            }
    } else {obraz_koncowy};
    usuniete_alpha
}