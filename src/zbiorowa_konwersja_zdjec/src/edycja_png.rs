use crate::wczytanie_zdjec::aktualizuj_postep;
use enumy::opcje::OptInterpolacja;
use enumy::statusy::LogTxKonw;
use futures::channel::mpsc::Sender;
use image::{imageops::FilterType, DynamicImage};
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use encodery::halper::{usun_kanal_alpha, zaszumianie};
use enumy::rozszerzenia::bdepth::BdepthPng;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
#[allow(clippy::too_many_arguments)]
pub async fn edycja_png(
    bufor: DynamicImage,
    rozdzielczości: &Vec<Rozdzielczości>,
    ścieżka_wyjściowa: &Path,
    ścieżka_dopełniająca: &String,
    opt_interpolacja: &OptInterpolacja,
    nazwa_pliku: &str,
    // exif: &DaneExif, // PNG rzadko używa EXIF, ale zostawiamy dla spójności
    kompresja: &u8,
    // filtr_alfa: &bool,
    alfa_rgb: &(u16, u16, u16),
    bit_depth: &Vec<BdepthPng>,
    zaszumianie_zmienna: Option<u8>,
    metryka_operacji: u32,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<LogTxKonw>,
) -> Result<(), tokio::io::Error> {
    // 1. Wybór filtra interpolacji
    let filtr = match opt_interpolacja {
        OptInterpolacja::Nearest => FilterType::Nearest,
        OptInterpolacja::Triangle => FilterType::Triangle,
        OptInterpolacja::CatmullRom => FilterType::CatmullRom,
        OptInterpolacja::Gaussian => FilterType::Gaussian,
        OptInterpolacja::Lanczos3 => FilterType::Lanczos3,
    };

    // 2. Główna pętla rozdzielczości
    for wariant in rozdzielczości {
        let (docelowy_wymiar, nazwa_wariantu) = match wariant {
            Rozdzielczości::R16 => (16, "_16"),
            Rozdzielczości::R32 => (32, "_32"),
            Rozdzielczości::R64 => (64, "_64"),
            Rozdzielczości::R128 => (128, "_128"),
            Rozdzielczości::R256 => (256, "_256"),
            Rozdzielczości::R512 => (512, "_512"),
            Rozdzielczości::R1k => (1024, "_1k"),
            Rozdzielczości::R2k => (2048, "_2k"),
            Rozdzielczości::R4k => (4096, "_4k"),
            Rozdzielczości::R6k => (6144, "_6k"),
            Rozdzielczości::R8k => (8192, "_8k"),
            Rozdzielczości::R16k => (16384, "_16k"),
            Rozdzielczości::Oryginalna => (0, ""),
        };

        for fdgfshd in bit_depth {


            aktualizuj_postep(
                &obecna_operacja,
                metryka_operacji,
                &mut tx,
            )
            .await;

            // --- OBSŁUGA BIT DEPTH I FORMATU ---
            let (final_img, nazwa_bd) = match fdgfshd {
                BdepthPng::Luma8 => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageLuma8(
                                usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_luma8(),
                            )
                        } else {
                            DynamicImage::ImageLuma8(
                                usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_luma8(),
                            )
                            .resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                        }
                    },
                    "_l8b",
                ),
                BdepthPng::Luma8Alpha => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageLumaA8(bufor.to_luma_alpha8())
                        } else {
                            DynamicImage::ImageLumaA8(bufor.to_luma_alpha8()).resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                        }
                    },
                    "_l8bt",
                ),
                BdepthPng::Rgb8 => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageRgb8(
                                usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8(),
                            )
                        } else {
                            DynamicImage::ImageRgb8(
                                usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8(),
                            )
                            .resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                        }
                    },
                    "_8b",
                ),
                BdepthPng::Rgb8Alpha => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageRgba8(bufor.to_rgba8())
                        } else {
                            DynamicImage::ImageRgba8(bufor.to_rgba8()).resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                        }
                    },
                    "_8bt",
                ),
                BdepthPng::Rgb16 => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageRgb16(bufor.to_rgb16())
                        } else {
                            DynamicImage::ImageRgb16(
                                usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb16(),
                            )
                            .resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                        }
                    },
                    "_16b",
                ),
                BdepthPng::Rgb16Alpha => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageRgba16(bufor.to_rgba16())
                        } else {
                            DynamicImage::ImageRgba16(bufor.to_rgba16()).resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                        }
                    },
                    "_16bt",
                ),
                BdepthPng::Luma16 => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageLuma16(bufor.to_luma16())
                        } else {
                            DynamicImage::ImageLuma16(
                                usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_luma16(),
                            )
                            .resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                        }
                    },
                    "_16b",
                ),
                BdepthPng::Luma16Alpha => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageLumaA16(bufor.to_luma_alpha16())
                        } else {
                            DynamicImage::ImageLumaA16(bufor.to_luma_alpha16()).resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                        }
                    },
                    "_16bt",
                ),
            };
            aktualizuj_postep(
                &obecna_operacja,
                metryka_operacji,
                &mut tx,
            )
            .await;
            let final_final_final_v3_xd = match zaszumianie_zmienna {
                Some(x) => zaszumianie(x, final_img),
                None => final_img,
            };

            // 4. Budowanie nazwy
            let finalna_nazwa = format!("{}{}{}.png", nazwa_pliku, nazwa_wariantu, nazwa_bd);
            let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
            ścieżka_pliku.push(ścieżka_dopełniająca);
            if !ścieżka_pliku.exists() {
                create_dir_all(ścieżka_pliku.clone())?;
            }
            ścieżka_pliku.push(finalna_nazwa);

            // 5. Zapis z kompresją
            let f = File::create(&ścieżka_pliku)?;

            // Mapowanie u8 kompresji (0-9) na poziomy PNG (Best, Fast, Default)
            let speed = match *kompresja {
                0 => image::codecs::png::CompressionType::Uncompressed,
                _ => image::codecs::png::CompressionType::Level(*kompresja),
            };

            let encoder = image::codecs::png::PngEncoder::new_with_quality(
                f,
                speed,
                image::codecs::png::FilterType::Adaptive,
            );

            final_final_final_v3_xd
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?;
            aktualizuj_postep(
                &obecna_operacja,
                metryka_operacji,
                &mut tx,
            )
            .await;
        }
    }

    Ok(())
}
