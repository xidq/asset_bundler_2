use crate::wczytywanie::strukty::DaneDoWczytywania;
use enumy::rozszerzenia::kolor::{ColorProfilePhoto, PrzestrzeńExr};
use exr::prelude::*;
use image::{DynamicImage, GenericImageView, Rgba32FImage};
use libheif_rs::{ColorPrimaries, TransferCharacteristics};
use std::io::Cursor;

// struct ExrOdbiornik {
//     width: usize,
//     data: Vec<f32>,
// }
pub fn exr_loading(bajty: &[u8]) -> std::result::Result<DaneDoWczytywania, std::io::Error> {

    let kursor = Cursor::new(bajty);

    // Odczytaj pierwszą płaską warstwę ze wszystkimi kanałami
    let image = read()
        .no_deep_data()
        .largest_resolution_level()
        .all_channels()                     // wczytaj wszystkie kanały bez narzucania ich typu
        .all_layers().all_attributes()      // wszystkie warstwy + atrybuty
        .from_buffered(kursor)
        .map_err(|e| std::io::Error::other(format!("Błąd dekodowania EXR: {}", e)))?;

    let layers = image.layer_data;
    // Wybieramy pierwszą warstwę (zazwyczaj jest tylko jedna)
    let layer = layers
        .into_iter()
        .next()
        .ok_or_else(|| std::io::Error::other("Plik EXR nie zawiera żadnej warstwy"))?;

    let size = layer.size;
    let width = size.x() as u32;
    let height = size.y() as u32;
    let channels = layer.channel_data; // AnyChannels<FlatSamples>

    // Znajdź kanały RGBA
    let mut r_samples = None;
    let mut g_samples = None;
    let mut b_samples = None;
    let mut a_samples = None;

    for channel in &channels.list {   // &Vec<AnyChannel<FlatSamples>> → możemy pożyczyć
        let r = Text::from("R");
        let g = Text::from("G");
        let b = Text::from("B");
        let a = Text::from("A");

        match channel.name.clone() {
            name if name == r => r_samples = Some(&channel.sample_data),
            name if name == g => g_samples = Some(&channel.sample_data),
            name if name == b  => b_samples = Some(&channel.sample_data),
            name if name == a => a_samples = Some(&channel.sample_data),
            _ => {}
        }
    }

    let r = r_samples.ok_or_else(|| std::io::Error::other("Brak kanału R"))?;
    let g = g_samples.ok_or_else(|| std::io::Error::other("Brak kanału G"))?;
    let b = b_samples.ok_or_else(|| std::io::Error::other("Brak kanału B"))?;
    let a = a_samples; // opcjonalny

    // Sprawdzamy typ próbek na podstawie kanału R (zakładamy, że wszystkie są tego samego typu)


    let num_pixels = (width * height) as usize;
    let mut rgba_f32 = vec![0.0f32; num_pixels * 4];   // tymczasowy bufor f32

    // Funkcja kopiująca dane kanału do bufora RGBA (składowa: 0=R, 1=G, 2=B, 3=A)
    fn copy_channel(samples: &FlatSamples, rgba: &mut [f32], component: usize, _num_pixels: usize) {
        match samples {
            FlatSamples::F16(data) => {
                for (i, v) in data.iter().enumerate() {
                    rgba[i * 4 + component] = v.to_f32();
                }
            }
            FlatSamples::F32(data) => {
                rgba[component..]
                    .iter_mut()
                    .step_by(4)
                    .zip(data.iter())
                    .for_each(|(dest, &src)| *dest = src);
            }
            FlatSamples::U32(data) => {
                for (i, &v) in data.iter().enumerate() {
                    rgba[i * 4 + component] = v as f32;
                }
            }
        }
    }

    copy_channel(r, &mut rgba_f32, 0, num_pixels);
    copy_channel(g, &mut rgba_f32, 1, num_pixels);
    copy_channel(b, &mut rgba_f32, 2, num_pixels);

    if let Some(a_samples) = a {
        copy_channel(a_samples, &mut rgba_f32, 3, num_pixels);
    } else {
        // jeśli brak kanału alfa – ustaw 1.0
        rgba_f32.iter_mut().skip(3).step_by(4).for_each(|v| *v = 1.0);
    }

    // Tworzymy DynamicImage w zależności od oryginalnej głębi bitowej
    let bufor = Rgba32FImage::from_raw(width, height, rgba_f32)
        .ok_or_else(|| std::io::Error::other("Nie udało się utworzyć Rgba32FImage"))?;

    let profil_koloru = if let Some(ref chroma) = image.attributes.chromaticities {
        ColorProfilePhoto::Exr(PrzestrzeńExr::LinearCustom([chroma.red.x(), chroma.red.y(),
            chroma.green.x(), chroma.green.y(),
            chroma.blue.x(), chroma.blue.y(),
            chroma.white.x(), chroma.white.y(),]))
    } else {
        ColorProfilePhoto::Exr(PrzestrzeńExr::LinearSRGB)
    };

    Ok(DaneDoWczytywania {
        dane: DynamicImage::ImageRgba32F(bufor),
        exif: None,
        kolor: profil_koloru,
    })
}
#[allow(dead_code)]
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
#[allow(dead_code)]
const SRGB_PRIMARIES: [f32; 8] = [0.64, 0.33, 0.30, 0.60, 0.15, 0.06, 0.3127, 0.3290];

/// Porównuje dwie 8‑liczbowe tablice chromatyczności z tolerancją.
#[allow(dead_code)]
fn primaries_approx_equal(a: [f32; 8], b: [f32; 8]) -> bool {
    a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < 1e-4)
}

/// Zwraca chromatyczności [rx,ry, gx,gy, bx,by, wx,wy] dla znanych standardów.
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// BT.709 / BT.2020 (SDR) -> liniowy
#[allow(dead_code)]
fn bt709_to_linear(c: f32) -> f32 {
    if c < 0.081 {
        c / 4.5
    } else {
        ((c + 0.099) / 1.099).powf(1.0 / 0.45)
    }
}