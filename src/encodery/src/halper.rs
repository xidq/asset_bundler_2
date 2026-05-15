use image::{ColorType, DynamicImage, ImageBuffer};
use rand::RngExt;
use std::path::{Path, PathBuf};
use lcms2::PixelFormat;
use enumy::rozszerzenia::kolor::ColorProfilePhoto;
use crate::transform::{konwertuj_przestrzen, profil_z_nclx};

pub fn zaszumianie(noising: u8, bufor: DynamicImage) -> DynamicImage {
    // let mut xoxo = bufor.clone();
    let mut rng = rand::rng();
    // let (w, h) = bufor.dimensions();
    let n_factor = noising as f64 / 100.0;
    // let bit_depth = bufor.color().bits_per_pixel() / bufor.color().channel_count() as u16;

    // let max_val: f64 = match bit_depth {
    //     8 => 255.0,
    //     16 => 65535.0,
    //     32 => 1.0, // Dla obrazów HDR (f32)
    //     _ => 255.0,
    // };


    // let max_delta = (max_val * (noising as f64 / 100.0));

    match bufor {
        // --- OBSŁUGA 8-BIT ---
        DynamicImage::ImageRgba8(mut img) => {
            let max_val = 255.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                for i in 0..3 {
                    // Tylko R, G, B
                    let v = pixel.0[i] as f64;
                    let delta = rng.random_range(-max_delta..=max_delta);
                    pixel.0[i] = (v + delta).clamp(0.0, max_val) as u8;
                }
            }
            DynamicImage::ImageRgba8(img)
        }

        // --- OBSŁUGA 16-BIT ---
        DynamicImage::ImageRgba16(mut img) => {
            let max_val = 65535.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                for i in 0..3 {
                    let v = pixel.0[i] as f64;
                    let delta = rng.random_range(-max_delta..=max_delta);
                    pixel.0[i] = (v + delta).clamp(0.0, max_val) as u16;
                }
            }
            DynamicImage::ImageRgba16(img)
        }

        // --- OBSŁUGA 32-BIT (F32) ---
        DynamicImage::ImageRgba32F(mut img) => {
            let max_val = 1.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                for i in 0..3 {
                    let v = pixel.0[i] as f64;
                    let delta = rng.random_range(-max_delta..=max_delta);
                    pixel.0[i] = (v + delta).clamp(0.0, max_val) as f32;
                }
            }
            DynamicImage::ImageRgba32F(img)
        }

        // Jeśli wpadnie format bez Alfy (RGB), traktujemy go tak samo
        DynamicImage::ImageRgb8(mut img) => {
            let max_val = 255.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                for i in 0..3 {
                    let v = pixel.0[i] as f64;
                    let delta = rng.random_range(-max_delta..=max_delta);
                    pixel.0[i] = (v + delta).clamp(0.0, max_val) as u8;
                }
            }
            DynamicImage::ImageRgb8(img)
        }
        DynamicImage::ImageLuma8(mut img) => {
            let max_val = 255.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                let v = pixel.0[0] as f64;
                let delta = rng.random_range(-max_delta..=max_delta);
                pixel.0[0] = (v + delta).clamp(0.0, max_val) as u8;
            }
            DynamicImage::ImageLuma8(img)
        }
        DynamicImage::ImageLumaA8(mut img) => {
            let max_val = 255.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                let v = pixel.0[0] as f64;
                let delta = rng.random_range(-max_delta..=max_delta);
                pixel.0[0] = (v + delta).clamp(0.0, max_val) as u8;
            }
            DynamicImage::ImageLumaA8(img)
        }
        DynamicImage::ImageLuma16(mut img) => {
            let max_val = 65535.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                let v = pixel.0[0] as f64;
                let delta = rng.random_range(-max_delta..=max_delta);
                pixel.0[0] = (v + delta).clamp(0.0, max_val) as u16;
            }
            DynamicImage::ImageLuma16(img)
        }
        DynamicImage::ImageLumaA16(mut img) => {
            let max_val = 65535.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                let v = pixel.0[0] as f64;
                let delta = rng.random_range(-max_delta..=max_delta);
                pixel.0[0] = (v + delta).clamp(0.0, max_val) as u16;
            }
            DynamicImage::ImageLumaA16(img)
        }

        _ => bufor, // Reszta formatów bez zmian
    }
}

pub fn usun_kanal_alpha(bufor: DynamicImage, alfa_rgb: (u16, u16, u16)) -> DynamicImage {
    let mapuj_u16_na_u8 = |v: u16| -> u8 { ((v as f32 / 65535.0) * 255.0).round() as u8 };


    // let (w, h) = (bufor.width(), bufor.height());
    let obraz_koncowy = bufor.clone();

    let usuniete_alpha: DynamicImage = if bufor.has_alpha() {
        match bufor.color() {
            ColorType::La8 => {

                //
                // for y in 0..obraz_koncowy.height() {
                //     for x in 0..obraz_koncowy.width() {
                //         let pikselek = obraz_koncowy.as_luma_alpha8().unwrap().get_pixel(x, y);
                //
                //         let l = if pikselek[1] < 255 {
                //             (tlo_szare as f32 * (1. - (pikselek[1] as f32 / 255.))
                //                 + (pikselek[0] as f32 * (pikselek[1] as f32 / 255.)))
                //                 as u8
                //         } else {
                //             pikselek[0]
                //         };
                //         nowy_bufor.put_pixel(x, y, image::Luma([l]));
                //     }
                // }
                // obraz_koncowy = image::DynamicImage::ImageLuma8(nowy_bufor);
                // obraz_koncowy
                let luma = bufor.as_luma_alpha8().unwrap();
                let (w, h) = luma.dimensions();

                let mut dane_luma = Vec::with_capacity((w * h) as usize);


                // let mut nowy_bufor = image::ImageBuffer::new(w, h);
                let tlo_r: u8 = mapuj_u16_na_u8(alfa_rgb.0);
                let tlo_g: u8 = mapuj_u16_na_u8(alfa_rgb.1);
                let tlo_b: u8 = mapuj_u16_na_u8(alfa_rgb.2);
                let tlo_szare = (((tlo_b + tlo_g + tlo_r) as f32) / 3.).round() as u32;
                for p in luma.pixels() {
                    let alpha = p[1] as u32;

                    if alpha == 255 {
                        // Szybka ścieżka dla pełnego koloru
                        dane_luma.push(p[0]);
                    } else if alpha == 0 {
                        // Szybka ścieżka dla pełnej przezroczystości (tylko kolor tła)
                        dane_luma.push(tlo_szare as u8);
                    } else {
                        // Alpha blending na liczbach całkowitych (szybsze niż floaty)
                        // Formuła: (kolor * alpha + tlo * (255 - alpha)) / 255

                        let inv_alpha = 255 - alpha;

                        dane_luma.push(((p[0] as u32 * alpha + tlo_szare * inv_alpha) / 255) as u8);
                    }
                }

                let nowy_bufor = ImageBuffer::from_raw(w, h, dane_luma).unwrap();
                DynamicImage::ImageLuma8(nowy_bufor)
            }
            ColorType::Rgba8 => {
                // let mut nowy_bufor = image::ImageBuffer::new(w, h);
                // let tlo_r: u8 = mapuj_u16_na_u8(alfa_rgb.0);
                // let tlo_g: u8 = mapuj_u16_na_u8(alfa_rgb.1);
                // let tlo_b: u8 = mapuj_u16_na_u8(alfa_rgb.2);
                //
                // for y in 0..obraz_koncowy.height() {
                //     for x in 0..obraz_koncowy.width() {
                //         let pikselek = obraz_koncowy.get_pixel(x, y);
                //
                //         let (r, g, b) = if pikselek[3] < 255 {
                //             let rgb_r = (tlo_r as f32 * (1. - (pikselek[3] as f32 / 255.))
                //                 + (pikselek[0] as f32 * (pikselek[3] as f32 / 255.)))
                //                 as u8;
                //             let rgb_g = (tlo_g as f32 * (1. - (pikselek[3] as f32 / 255.))
                //                 + (pikselek[1] as f32 * (pikselek[3] as f32 / 255.)))
                //                 as u8;
                //             let rgb_b = (tlo_b as f32 * (1. - (pikselek[3] as f32 / 255.))
                //                 + (pikselek[2] as f32 * (pikselek[3] as f32 / 255.)))
                //                 as u8;
                //             (rgb_r, rgb_g, rgb_b)
                //         } else {
                //             (pikselek[0], pikselek[1], pikselek[2])
                //         };
                //         nowy_bufor.put_pixel(x, y, image::Rgb([r, g, b]));
                //     }
                // }
                // obraz_koncowy = image::DynamicImage::ImageRgb8(nowy_bufor);
                // obraz_koncowy
                let rgba = bufor.as_rgba8().unwrap();
                let (w, h) = rgba.dimensions();

                // Przygotowujemy kolory tła raz, zamiast w pętli
                let tlo_r = mapuj_u16_na_u8(alfa_rgb.0) as u32;
                let tlo_g = mapuj_u16_na_u8(alfa_rgb.1) as u32;
                let tlo_b = mapuj_u16_na_u8(alfa_rgb.2) as u32;

                // Tworzymy wektor na dane wyjściowe (3 kanały RGB)
                let mut dane_rgb = Vec::with_capacity((w * h * 3) as usize);

                // Iterujemy bezpośrednio po surowych bajtach [R, G, B, A]
                for p in rgba.pixels() {
                    let alpha = p[3] as u32;

                    if alpha == 255 {
                        // Szybka ścieżka dla pełnego koloru
                        dane_rgb.push(p[0]);
                        dane_rgb.push(p[1]);
                        dane_rgb.push(p[2]);
                    } else if alpha == 0 {
                        // Szybka ścieżka dla pełnej przezroczystości (tylko kolor tła)
                        dane_rgb.push(tlo_r as u8);
                        dane_rgb.push(tlo_g as u8);
                        dane_rgb.push(tlo_b as u8);
                    } else {
                        // Alpha blending na liczbach całkowitych (szybsze niż floaty)
                        // Formuła: (kolor * alpha + tlo * (255 - alpha)) / 255
                        let inv_alpha = 255 - alpha;

                        dane_rgb.push(((p[0] as u32 * alpha + tlo_r * inv_alpha) / 255) as u8);
                        dane_rgb.push(((p[1] as u32 * alpha + tlo_g * inv_alpha) / 255) as u8);
                        dane_rgb.push(((p[2] as u32 * alpha + tlo_b * inv_alpha) / 255) as u8);
                    }
                }

                let nowy_bufor = ImageBuffer::from_raw(w, h, dane_rgb).unwrap();
                DynamicImage::ImageRgb8(nowy_bufor)
            }
            ColorType::La16 => {
                //ImageBuffer<Rgb<u16>, Vec<u16>>
                // let mut nowy_bufor: ImageBuffer<Luma<u16>, Vec<u16>> =
                //     image::ImageBuffer::new(w, h);
                // let tlo_szare =
                //     (((alfa_rgb.0 * alfa_rgb.1 * alfa_rgb.2) as f32) / 3.).round() as u8;
                //
                // for y in 0..obraz_koncowy.height() {
                //     for x in 0..obraz_koncowy.width() {
                //         let pikselek: &LumaA<u16> =
                //             obraz_koncowy.as_luma_alpha16().unwrap().get_pixel(x, y);
                //
                //         let l: u16 = if pikselek[1] < 65535 {
                //             let l: u16 = (tlo_szare as f32 * (1. - (pikselek[1] as f32 / 65535.))
                //                 + (pikselek[0] as f32 * (pikselek[1] as f32 / 65535.)))
                //                 as u16;
                //             l
                //         } else {
                //             pikselek[0]
                //         };
                //         nowy_bufor.put_pixel(x, y, image::Luma([l]));
                //     }
                // }
                // obraz_koncowy = image::DynamicImage::ImageLuma16(nowy_bufor);
                // obraz_koncowy
                let rgba = bufor.as_rgba16().unwrap();
                let (w, h) = rgba.dimensions();

                // Przygotowujemy kolory tła raz, zamiast w pętli
                let tlo_r = alfa_rgb.0 as u32;
                let tlo_g = alfa_rgb.1 as u32;
                let tlo_b = alfa_rgb.2 as u32;
                let tlo_szare = (((tlo_b + tlo_g + tlo_r) as f32) / 3.).round() as u32;


                // Tworzymy wektor na dane wyjściowe (3 kanały RGB)
                let mut dane_rgb = Vec::with_capacity((w * h * 3) as usize);

                // Iterujemy bezpośrednio po surowych bajtach [R, G, B, A]
                for p in rgba.pixels() {
                    let alpha = p[1] as u32;

                    if alpha == u16::MAX as u32 {
                        // Szybka ścieżka dla pełnego koloru
                        dane_rgb.push(p[0]);
                    } else if alpha == 0 {
                        // Szybka ścieżka dla pełnej przezroczystości (tylko kolor tła)
                        dane_rgb.push(tlo_szare as u16);
                    } else {
                        // Alpha blending na liczbach całkowitych (szybsze niż floaty)
                        // Formuła: (kolor * alpha + tlo * (255 - alpha)) / 255
                        let inv_alpha = u16::MAX as *const () as u32 - alpha;

                        dane_rgb.push(((p[0] as u32 * alpha + tlo_szare * inv_alpha) / 255) as u16);
                    }
                }

                let nowy_bufor = ImageBuffer::from_raw(w, h, dane_rgb).unwrap();
                DynamicImage::ImageLuma16(nowy_bufor)
            }
            ColorType::Rgba16 => {
                // let mut nowy_bufor: ImageBuffer<Rgb<u16>, Vec<u16>> = image::ImageBuffer::new(w, h);
                // for y in 0..obraz_koncowy.height() {
                //     for x in 0..obraz_koncowy.width() {
                //         let pikselek: &Rgba<u16> =
                //             obraz_koncowy.as_rgba16().unwrap().get_pixel(x, y);
                //         // u16 ma max 65535
                //         let (r, g, b) = if pikselek[3] < 65535 {
                //             let rgb_r = (alfa_rgb.0 as f32 * (1. - (pikselek[3] as f32 / 65535.))
                //                 + (pikselek[0] as f32 * (pikselek[3] as f32 / 65535.)))
                //                 as u16;
                //             let rgb_g = (alfa_rgb.1 as f32 * (1. - (pikselek[3] as f32 / 65535.))
                //                 + (pikselek[1] as f32 * (pikselek[3] as f32 / 65535.)))
                //                 as u16;
                //             let rgb_b = (alfa_rgb.2 as f32 * (1. - (pikselek[3] as f32 / 65535.))
                //                 + (pikselek[2] as f32 * (pikselek[3] as f32 / 65535.)))
                //                 as u16;
                //             (rgb_r, rgb_g, rgb_b)
                //         } else {
                //             (pikselek[0], pikselek[1], pikselek[2])
                //         };
                //         nowy_bufor.put_pixel(x, y, image::Rgb([r, g, b]));
                //     }
                // }
                //
                // obraz_koncowy = image::DynamicImage::ImageRgb16(nowy_bufor);
                // obraz_koncowy
                let rgba = bufor.as_rgba16().unwrap();
                let (w, h) = rgba.dimensions();

                // Przygotowujemy kolory tła raz, zamiast w pętli
                let tlo_r = alfa_rgb.0 as u32;
                let tlo_g = alfa_rgb.1 as u32;
                let tlo_b = alfa_rgb.2 as u32;

                // Tworzymy wektor na dane wyjściowe (3 kanały RGB)
                let mut dane_rgb = Vec::with_capacity((w * h * 3) as usize);

                // Iterujemy bezpośrednio po surowych bajtach [R, G, B, A]
                for p in rgba.pixels() {
                    let alpha = p[3] as u32;

                    if alpha == u16::MAX as u32 {
                        // Szybka ścieżka dla pełnego koloru
                        dane_rgb.push(p[0]);
                        dane_rgb.push(p[1]);
                        dane_rgb.push(p[2]);
                    } else if alpha == 0 {
                        // Szybka ścieżka dla pełnej przezroczystości (tylko kolor tła)
                        dane_rgb.push(tlo_r as u16);
                        dane_rgb.push(tlo_g as u16);
                        dane_rgb.push(tlo_b as u16);
                    } else {
                        // Alpha blending na liczbach całkowitych (szybsze niż floaty)
                        // Formuła: (kolor * alpha + tlo * (255 - alpha)) / 255
                        let inv_alpha = u16::MAX as *const () as u32 - alpha;

                        dane_rgb.push(((p[0] as u32 * alpha + tlo_r * inv_alpha) / 255) as u16);
                        dane_rgb.push(((p[1] as u32 * alpha + tlo_g * inv_alpha) / 255) as u16);
                        dane_rgb.push(((p[2] as u32 * alpha + tlo_b * inv_alpha) / 255) as u16);
                    }
                }

                let nowy_bufor = ImageBuffer::from_raw(w, h, dane_rgb).unwrap();
                DynamicImage::ImageRgb16(nowy_bufor)
            }
            ColorType::Rgba32F => obraz_koncowy,
            _ => obraz_koncowy,
        }
    } else {
        obraz_koncowy
    };
    usuniete_alpha
}

pub fn merge_sciezki(ścieżka_wyjściowa:&Path, ścieżka_dopełniająca: &String) -> PathBuf {
    let mut huehuehue = ścieżka_wyjściowa.to_path_buf();
    huehuehue.push(ścieżka_dopełniająca);
    huehuehue
}

pub fn ogarnij_icc(kolor: ColorProfilePhoto, img: &DynamicImage, profil: PixelFormat) -> Vec<u8> {
    if let ColorProfilePhoto::ICC(ref xoxo) = kolor {
        konwertuj_przestrzen(img, Some(xoxo), profil)
            .expect("Błąd konwersji ICC")

    } else if let ColorProfilePhoto::NCLX(ref nclx_data) = kolor {
        // profil z NCLX, do bajtów ICC
        let wygenerowany_profil = profil_z_nclx(nclx_data).expect("Błąd generowania profilu z NCLX");
        let icc_bajty = wygenerowany_profil.icc().expect("Błąd serializacji profilu do ICC");

        konwertuj_przestrzen(img, Some(&icc_bajty), profil)
            .expect("Błąd konwersji z profilu NCLX")

    } else {
        // Brak profilu (None)
        konwertuj_przestrzen(img, None, profil)
            .expect("Błąd konwersji bez profilu")
    }
}