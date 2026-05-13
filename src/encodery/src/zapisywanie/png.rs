use std::fs::{create_dir_all, File};
use std::sync::Arc;
use futures::channel::mpsc::Sender;
use image::DynamicImage;
use image::imageops::FilterType;
use tokio::sync::Mutex;
use enumy::przetwarzanie::{PrzetwarzanieJpg, PrzetwarzaniePng};
use enumy::rozszerzenia::bdepth::{BdepthJpg, BdepthPng};
use enumy::statusy::Logi;
use crate::halper::{usun_kanal_alpha, zaszumianie};
use crate::send::wyslij_status;

pub async fn png_match<T>(
    dane: PrzetwarzaniePng,
    wymiar: u32,
    bit_depth: BdepthPng,
    nazwa_wariantu: String,
    filtr: FilterType,
    metryka_operacji: Option<u32>,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<T>,
) -> Result<(), tokio::io::Error>
where T: Logi,
{
    let (final_img, nazwa_bd) = match bit_depth {
        BdepthPng::Luma8 => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageLuma8(
                        usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_luma8(),
                    )
                } else {
                    DynamicImage::ImageLuma8(
                        usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_luma8(),
                    )
                        .resize(
                            wymiar,
                            wymiar,
                            filtr,
                        )
                }
            },
            "_l8b",
        ),
        BdepthPng::Luma8Alpha => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageLumaA8(dane.bufor.to_luma_alpha8())
                } else {
                    DynamicImage::ImageLumaA8(dane.bufor.to_luma_alpha8()).resize(
                        wymiar,
                        wymiar,
                        filtr,
                    )
                }
            },
            "_l8bt",
        ),
        BdepthPng::Rgb8 => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageRgb8(
                        usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_rgb8(),
                    )
                } else {
                    DynamicImage::ImageRgb8(
                        usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_rgb8(),
                    )
                        .resize(
                            wymiar,
                            wymiar,
                            filtr,
                        )
                }
            },
            "_8b",
        ),
        BdepthPng::Rgb8Alpha => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageRgba8(dane.bufor.to_rgba8())
                } else {
                    DynamicImage::ImageRgba8(dane.bufor.to_rgba8()).resize(
                        wymiar,
                        wymiar,
                        filtr,
                    )
                }
            },
            "_8bt",
        ),
        BdepthPng::Rgb16 => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageRgb16(dane.bufor.to_rgb16())
                } else {
                    DynamicImage::ImageRgb16(
                        usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_rgb16(),
                    )
                        .resize(
                            wymiar,
                            wymiar,
                            filtr,
                        )
                }
            },
            "_16b",
        ),
        BdepthPng::Rgb16Alpha => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageRgba16(dane.bufor.to_rgba16())
                } else {
                    DynamicImage::ImageRgba16(dane.bufor.to_rgba16()).resize(
                        wymiar,
                        wymiar,
                        filtr,
                    )
                }
            },
            "_16bt",
        ),
        BdepthPng::Luma16 => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageLuma16(dane.bufor.to_luma16())
                } else {
                    DynamicImage::ImageLuma16(
                        usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_luma16(),
                    )
                        .resize(
                            wymiar,
                            wymiar,
                            filtr,
                        )
                }
            },
            "_16b",
        ),
        BdepthPng::Luma16Alpha => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageLumaA16(dane.bufor.to_luma_alpha16())
                } else {
                    DynamicImage::ImageLumaA16(dane.bufor.to_luma_alpha16()).resize(
                        wymiar,
                        wymiar,
                        filtr,
                    )
                }
            },
            "_16bt",
        ),
    };

    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);
    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;
    
    let final_final_final_v3_xd = match dane.zaszumienie {
        Some(x) => zaszumianie(x, final_img),
        None => final_img,
    };

    // 4. Budowanie nazwy
    let finalna_nazwa = format!("{}{}{}.png", dane.nazwa, nazwa_wariantu, nazwa_bd);
    let mut ścieżka_pliku = dane.sciezka_wyjsciowa.to_path_buf();
    if !ścieżka_pliku.exists() {
        create_dir_all(ścieżka_pliku.clone())?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    // 5. Zapis z kompresją
    let f = File::create(&ścieżka_pliku)?;

    // Mapowanie u8 kompresji (0-9) na poziomy PNG (Best, Fast, Default)
    let speed = match dane.kompresja {
        0 => image::codecs::png::CompressionType::Uncompressed,
        _ => image::codecs::png::CompressionType::Level(dane.kompresja),
    };

    let encoder = image::codecs::png::PngEncoder::new_with_quality(
        f,
        speed,
        image::codecs::png::FilterType::Adaptive,
    );

    final_final_final_v3_xd
        .write_with_encoder(encoder)
        .map_err(std::io::Error::other)?;

    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);
    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;

    Ok(())
}