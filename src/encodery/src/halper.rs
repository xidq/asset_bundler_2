use image::{ColorType, DynamicImage, GenericImageView, ImageBuffer, Rgba32FImage, RgbaImage};
use rand::RngExt;
use std::path::{Path, PathBuf};
use image::ExtendedColorType::Rgba16;
use lcms2::PixelFormat;
use libheif_rs::{ColorPrimaries, TransferCharacteristics};
use enumy::rozszerzenia::kolor::{ColorNclx, ColorProfilePhoto, PrzestrzeńExr};
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

pub fn konwersja_float_na_mniejsze(foto: DynamicImage,  chroma: PrzestrzeńExr) -> (DynamicImage, ColorProfilePhoto){

    //tutaj jest wczytywany plik f32, który może być f32a, f32, f16, f16a itd... ogólnie to jest exr...
    // w chroma jest [f32; 8] wyciągany przy wczytywaniu.
    // w tej fn przetwarzane ma być to exr zapakowane w dynamicimage na 16bit obraz
    // jak ma alpha to 16bAlpha, a jak nie ma to tylko 16b.
    // też przestrzeń kolorów aby była brana pod uwagę, czyli przekształcenie np do rec2020 czy srgb no i
    // aby było to poprawnie zrobione dla późniejszego ewentualnego przetwarzania ICC
    // pub enum ColorProfilePhoto {
    //     ICC(Vec<u8>),
    //     NCLX(enumy::rozszerzenia::kolor::ColorNclx),
    //     Exr(enumy::rozszerzenia::kolor::PrzestrzeńExr),
    //     None,
    // }
    // #[derive(Clone, Debug)]
    // pub enum PrzestrzeńExr {
    //     LinearSRGB,
    //     LinearCustom([f32; 8]),
    // }
    //
    // #[derive(Debug)]
    // #[derive(Clone)]
    // pub struct ColorNclx {
    //     pub primaries: libheif_rs::ColorPrimaries,
    //     pub transfer: libheif_rs::TransferCharacteristics,
    //     pub matrix: libheif_rs::MatrixCoefficients,
    //     pub full_range: bool,
    // }
    let src = match foto {
        DynamicImage::ImageRgba32F(buf) => buf,
        _ => panic!("Oczekiwano ImageRgba32F"),
    };

    let (width, height) = src.dimensions();
    let pixels: Vec<f32> = src.into_raw(); // układ R,G,B,A,R,G,B,A...

    // 2. Oddzielamy alfa i RGB
    let num_pixels = (width * height) as usize;
    let mut rgb_linear = vec![0.0f32; num_pixels * 3];
    let mut alpha = vec![1.0f32; num_pixels];

    for i in 0..num_pixels {
        let base = i * 4;
        rgb_linear[i * 3]     = pixels[base];
        rgb_linear[i * 3 + 1] = pixels[base + 1];
        rgb_linear[i * 3 + 2] = pixels[base + 2];
        alpha[i]              = pixels[base + 3];
    }

    // 3. Macierz konwersji z przestrzeni źródłowej do liniowego sRGB
    let to_srgb_matrix = match chroma {
        PrzestrzeńExr::LinearSRGB => {
            // Już jesteśmy w liniowym sRGB – macierz jednostkowa
            [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ]
        }
        PrzestrzeńExr::LinearCustom(primaries) => {
            // primaries = [rx, ry, gx, gy, bx, by, wx, wy]
            compute_rgb_to_rgb_matrix(primaries, SRGB_PRIMARIES)
        }
    };

    // 4. Aplikacja macierzy na każdym pikselu
    for rgb in rgb_linear.chunks_mut(3) {
        let r = rgb[0];
        let g = rgb[1];
        let b = rgb[2];

        let new_r = to_srgb_matrix[0] * r + to_srgb_matrix[1] * g + to_srgb_matrix[2] * b;
        let new_g = to_srgb_matrix[3] * r + to_srgb_matrix[4] * g + to_srgb_matrix[5] * b;
        let new_b = to_srgb_matrix[6] * r + to_srgb_matrix[7] * g + to_srgb_matrix[8] * b;

        // Przycięcie do [0, 1] (w liniowym sRGB dopuszczamy wartości >1, ale dla 16-bit int lepiej przyciąć)
        rgb[0] = new_r.clamp(0.0, 1.0);
        rgb[1] = new_g.clamp(0.0, 1.0);
        rgb[2] = new_b.clamp(0.0, 1.0);
    }

    // 5. Korekcja gamma: liniowy sRGB → sRGB (gamma ~2.2)
    for c in rgb_linear.iter_mut() {
        *c = linear_to_srgb(*c);
    }

    // 6. Konwersja do 16-bit integer (0..65535)
    let mut u16_data = vec![0u16; num_pixels * 3];
    for i in 0..num_pixels * 3 {
        u16_data[i] = (rgb_linear[i] * 65535.0).round() as u16;
    }

    // 7. Tworzenie DynamicImage: Rgb16 lub Rgba16 w zależności od obecności alfa
    let has_alpha = alpha.iter().any(|&a| (a - 1.0).abs() > 0.001); // uproszczona heurystyka
    let dynamic_output = if has_alpha {
        let mut rgba_u16 = vec![0u16; num_pixels * 4];
        for i in 0..num_pixels {
            let si = i * 3;
            let di = i * 4;
            rgba_u16[di]     = u16_data[si];
            rgba_u16[di + 1] = u16_data[si + 1];
            rgba_u16[di + 2] = u16_data[si + 2];
            rgba_u16[di + 3] = (alpha[i] * 65535.0).round() as u16;
        }
        DynamicImage::ImageRgba16(
            ImageBuffer::<image::Rgba<u16>, _>::from_raw(width, height, rgba_u16)
                .expect("Błąd tworzenia Rgba16Image")
        )
    } else {
        DynamicImage::ImageRgb16(
            ImageBuffer::<image::Rgb<u16>, _>::from_raw(width, height, u16_data)
                .expect("Błąd tworzenia Rgb16Image")
        )
    };

    // 8. Tworzymy profil dla obrazu wynikowego: sRGB z korekcją gamma
    let profile = ColorProfilePhoto::NCLX(ColorNclx {
        primaries: libheif_rs::ColorPrimaries::ITU_R_BT_709_5,
        transfer: libheif_rs::TransferCharacteristics::IEC_61966_2_1,
        matrix: libheif_rs::MatrixCoefficients::RGB_GBR,
        full_range: true,
    });

    (dynamic_output, profile)
}

// ---------- pomocnicze ----------
// const SRGB_PRIMARIES: [f32; 8] = [0.64, 0.33, 0.30, 0.60, 0.15, 0.06, 0.3127, 0.3290];

/// Wyznacza macierz 3x3 przekształcającą liniowe RGB z przestrzeni o chromatycznościach `src_prim`
/// do przestrzeni `dst_prim`. Oba zestawy podane jako [rx, ry, gx, gy, bx, by, wx, wy].
fn compute_rgb_to_rgb_matrix(src_prim: [f32; 8], dst_prim: [f32; 8]) -> [f32; 9] {
    fn primaries_to_xyz(p: [f32; 8]) -> [[f32; 3]; 3] {
        let [rx, ry, gx, gy, bx, by, wx, wy] = p;
        // macierz chromatyczności XYZ = M * diag(S)
        let x = |x, y| x / y;
        let z = |x, y| (1.0 - x - y) / y;
        let mat = [
            [x(rx, ry), x(gx, gy), x(bx, by)],
            [1.0, 1.0, 1.0],
            [z(rx, ry), z(gx, gy), z(bx, by)],
        ];
        // wektor S wyznaczamy z bieli:
        let inv = invert_3x3(mat);
        let s = [
            inv[0][0] * x(wx, wy) + inv[0][1] * 1.0 + inv[0][2] * z(wx, wy),
            inv[1][0] * x(wx, wy) + inv[1][1] * 1.0 + inv[1][2] * z(wx, wy),
            inv[2][0] * x(wx, wy) + inv[2][1] * 1.0 + inv[2][2] * z(wx, wy),
        ];
        // M = mat * diag(S)
        [
            [mat[0][0] * s[0], mat[0][1] * s[1], mat[0][2] * s[2]],
            [mat[1][0] * s[0], mat[1][1] * s[1], mat[1][2] * s[2]],
            [mat[2][0] * s[0], mat[2][1] * s[1], mat[2][2] * s[2]],
        ]
    }

    let src_xyz = primaries_to_xyz(src_prim);
    let dst_xyz = primaries_to_xyz(dst_prim);
    let dst_inv = invert_3x3(dst_xyz);
    // transform: dst_inv * src_xyz
    let mut m = [0.0f32; 9];
    for r in 0..3 {
        for c in 0..3 {
            m[r * 3 + c] = dst_inv[r][0] * src_xyz[0][c]
                + dst_inv[r][1] * src_xyz[1][c]
                + dst_inv[r][2] * src_xyz[2][c];
        }
    }
    m
}

fn invert_3x3(m: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
    let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
    let inv_det = 1.0 / det;
    [
        [
            (m[1][1] * m[2][2] - m[1][2] * m[2][1]) * inv_det,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * inv_det,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * inv_det,
        ],
        [
            (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * inv_det,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * inv_det,
            (m[0][2] * m[1][0] - m[0][0] * m[1][2]) * inv_det,
        ],
        [
            (m[1][0] * m[2][1] - m[1][1] * m[2][0]) * inv_det,
            (m[0][1] * m[2][0] - m[0][0] * m[2][1]) * inv_det,
            (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * inv_det,
        ],
    ]
}

/// Gamma sRGB: liniowy -> sRGB (IEC 61966-2-1)
fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

pub fn konwersja_mniejsze_na_float(
    foto: DynamicImage,
    profil_wejscia: ColorProfilePhoto,
) -> (DynamicImage, ColorProfilePhoto) {

    let (width, height) = foto.dimensions();
    let num_pixels = (width * height) as usize;

    // Określamy funkcję transferu i primaries z profilu wejściowego.
    let (transfer, primaries_enum) = match &profil_wejscia {
        ColorProfilePhoto::NCLX(nclx) => (nclx.transfer, Some(nclx.primaries)),
        ColorProfilePhoto::Exr(_) => panic!("Obraz EXR nie powinien trafić do konwersji w tę stronę"),
        ColorProfilePhoto::ICC(_) | ColorProfilePhoto::None => {
            // Domyślnie przyjmujemy sRGB
            (TransferCharacteristics::IEC_61966_2_1, Some(ColorPrimaries::ITU_R_BT_709_5))
        }
    };

    // Bufor na liniowe dane (RGBA f32)
    let mut rgba_linear = vec![0.0f32; num_pixels * 4];

    // Wypełnianie bufora – odczyt oryginalnych pikseli i linearyzacja
    match foto {
        DynamicImage::ImageLuma8(buf) => {
            for (i, &y) in buf.iter().enumerate() {
                let lin = inverse_transfer(y as f32 / 255.0, transfer);
                let offset = i * 4;
                rgba_linear[offset..offset+3].copy_from_slice(&[lin, lin, lin]);
                rgba_linear[offset+3] = 1.0;
            }
        }
        DynamicImage::ImageLumaA8(buf) => {
            for (i, pix) in buf.pixels().enumerate() {
                let y_lin = inverse_transfer(pix.0[0] as f32 / 255.0, transfer);
                let a = pix.0[1] as f32 / 255.0;
                let offset = i * 4;
                rgba_linear[offset..offset+3].copy_from_slice(&[y_lin, y_lin, y_lin]);
                rgba_linear[offset+3] = a;
            }
        }
        DynamicImage::ImageLuma16(buf) => {
            for (i, &y) in buf.iter().enumerate() {
                let lin = inverse_transfer(y as f32 / 65535.0, transfer);
                let offset = i * 4;
                rgba_linear[offset..offset+3].copy_from_slice(&[lin, lin, lin]);
                rgba_linear[offset+3] = 1.0;
            }
        }
        DynamicImage::ImageLumaA16(buf) => {
            for (i, pix) in buf.pixels().enumerate() {
                let y_lin = inverse_transfer(pix.0[0] as f32 / 65535.0, transfer);
                let a = pix.0[1] as f32 / 65535.0;
                let offset = i * 4;
                rgba_linear[offset..offset+3].copy_from_slice(&[y_lin, y_lin, y_lin]);
                rgba_linear[offset+3] = a;
            }
        }
        DynamicImage::ImageRgb8(buf) => {
            for (i, pix) in buf.pixels().enumerate() {
                let offset = i * 4;
                rgba_linear[offset]   = inverse_transfer(pix.0[0] as f32 / 255.0, transfer);
                rgba_linear[offset+1] = inverse_transfer(pix.0[1] as f32 / 255.0, transfer);
                rgba_linear[offset+2] = inverse_transfer(pix.0[2] as f32 / 255.0, transfer);
                rgba_linear[offset+3] = 1.0;
            }
        }
        DynamicImage::ImageRgba8(buf) => {
            for (i, pix) in buf.pixels().enumerate() {
                let offset = i * 4;
                rgba_linear[offset]   = inverse_transfer(pix.0[0] as f32 / 255.0, transfer);
                rgba_linear[offset+1] = inverse_transfer(pix.0[1] as f32 / 255.0, transfer);
                rgba_linear[offset+2] = inverse_transfer(pix.0[2] as f32 / 255.0, transfer);
                rgba_linear[offset+3] = pix.0[3] as f32 / 255.0;
            }
        }
        DynamicImage::ImageRgb16(buf) => {
            for (i, pix) in buf.pixels().enumerate() {
                let offset = i * 4;
                rgba_linear[offset]   = inverse_transfer(pix.0[0] as f32 / 65535.0, transfer);
                rgba_linear[offset+1] = inverse_transfer(pix.0[1] as f32 / 65535.0, transfer);
                rgba_linear[offset+2] = inverse_transfer(pix.0[2] as f32 / 65535.0, transfer);
                rgba_linear[offset+3] = 1.0;
            }
        }
        DynamicImage::ImageRgba16(buf) => {
            for (i, pix) in buf.pixels().enumerate() {
                let offset = i * 4;
                rgba_linear[offset]   = inverse_transfer(pix.0[0] as f32 / 65535.0, transfer);
                rgba_linear[offset+1] = inverse_transfer(pix.0[1] as f32 / 65535.0, transfer);
                rgba_linear[offset+2] = inverse_transfer(pix.0[2] as f32 / 65535.0, transfer);
                rgba_linear[offset+3] = pix.0[3] as f32 / 65535.0;
            }
        }
        _ => panic!("Nieobsługiwany format obrazu dla konwersji do EXR"),
    }

    // Pobieramy współrzędne chromatyczności dla wejściowych primaries
    let primaries_coords = if let Some(p) = primaries_enum {
        primaries_to_coordinates(p)
    } else {
        SRGB_PRIMARIES
    };

    let exr_space = if primaries_approx_equal(primaries_coords, SRGB_PRIMARIES) {
        PrzestrzeńExr::LinearSRGB
    } else {
        PrzestrzeńExr::LinearCustom(primaries_coords)
    };

    let buf = Rgba32FImage::from_raw(width, height, rgba_linear)
        .expect("Nie udało się utworzyć bufora Rgba32FImage");

    (DynamicImage::ImageRgba32F(buf), ColorProfilePhoto::Exr(exr_space))
}

// ---------------------------------------------------------------------------
// Funkcje pomocnicze

/// Stałe chromatyczności sRGB / BT.709
const SRGB_PRIMARIES: [f32; 8] = [0.64, 0.33, 0.30, 0.60, 0.15, 0.06, 0.3127, 0.3290];

/// Porównuje dwie 8‑liczbowe tablice chromatyczności z tolerancją.
fn primaries_approx_equal(a: [f32; 8], b: [f32; 8]) -> bool {
    a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < 1e-4)
}

/// Zwraca chromatyczności [rx,ry, gx,gy, bx,by, wx,wy] dla znanych standardów.
fn primaries_to_coordinates(p: ColorPrimaries) -> [f32; 8] {
    match p {
        ColorPrimaries::ITU_R_BT_709_5 =>
            [0.64, 0.33, 0.30, 0.60, 0.15, 0.06, 0.3127, 0.3290],
        ColorPrimaries::ITU_R_BT_2020_2_and_2100_0 =>
            [0.708, 0.292, 0.170, 0.797, 0.131, 0.046, 0.3127, 0.3290],
        ColorPrimaries::SMPTE_240M =>
            [0.630, 0.340, 0.310, 0.595, 0.155, 0.070, 0.3127, 0.3290],
        ColorPrimaries::GenericFilm =>
            [0.681, 0.319, 0.243, 0.692, 0.145, 0.049, 0.310, 0.316], // ciepła biel
        _ => SRGB_PRIMARIES, // dla nieznanych zwracamy sRGB
    }
}

/// Odwrotność funkcji transferu (gamma → liniowy).
fn inverse_transfer(value: f32, tf: TransferCharacteristics) -> f32 {
    match tf {
        TransferCharacteristics::IEC_61966_2_1 => srgb_to_linear(value),
        TransferCharacteristics::Linear => value,
        TransferCharacteristics::ITU_R_BT_709_5
        | TransferCharacteristics::ITU_R_BT_2020_2_10bit
        | TransferCharacteristics::ITU_R_BT_2020_2_12bit => bt709_to_linear(value),
        _ => {
            // Dla nieobsłużonych używamy sRGB (bezpieczny fallback)
            srgb_to_linear(value)
        }
    }
}

/// sRGB -> liniowy (IEC 61966‑2‑1)
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// BT.709 / BT.2020 (SDR) -> liniowy
fn bt709_to_linear(c: f32) -> f32 {
    if c < 0.081 {
        c / 4.5
    } else {
        ((c + 0.099) / 1.099).powf(1.0 / 0.45)
    }
}