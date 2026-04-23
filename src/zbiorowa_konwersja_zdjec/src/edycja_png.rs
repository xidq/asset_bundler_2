use crate::pomocnicze::usun_kanal_alpha;
use crate::wczytanie_zdjec::aktualizuj_postep;
use crate::zmiana_fot::zaszumianie;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptInterpolacja, OptRozdzielczościObrazów};
use enumy::statusy::LogTxDoBathKonwersjaZdjęć;
use futures::SinkExt;
use futures::channel::mpsc;
use futures::channel::mpsc::Sender;
use image::{ColorType, DynamicImage, GenericImageView, ImageBuffer, Rgba, imageops::FilterType};
use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn edycja_png(
    bufor: DynamicImage,
    rozdzielczości: &Vec<OptRozdzielczościObrazów>,
    ścieżka_wyjściowa: &Path,
    ścieżka_dopełniająca: &String,
    OptInterpolacja: &OptInterpolacja,
    nazwa_pliku: &str,
    // exif: &DaneExif, // PNG rzadko używa EXIF, ale zostawiamy dla spójności
    kompresja: &u8,
    // filtr_alfa: &bool,
    alfa_rgb: &(u16, u16, u16),
    bit_depth: &Vec<OptFormatyKoloruObrazOgólny>,
    zaszumianie_zmienna: Option<u8>,
    metryka_operacji: u32,
    obecna_operacja: Arc<Mutex<u32>>,
    procent_progress: Arc<Mutex<u8>>,
    mut tx: Sender<LogTxDoBathKonwersjaZdjęć>,
) -> Result<(), tokio::io::Error> {
    // 1. Wybór filtra interpolacji
    let filtr = match OptInterpolacja {
        OptInterpolacja::Nearest => FilterType::Nearest,
        OptInterpolacja::Triangle => FilterType::Triangle,
        OptInterpolacja::CatmullRom => FilterType::CatmullRom,
        OptInterpolacja::Gaussian => FilterType::Gaussian,
        OptInterpolacja::Lanczos3 => FilterType::Lanczos3,
    };

    // 2. Główna pętla rozdzielczości
    for wariant in rozdzielczości {
        let (docelowy_wymiar, nazwa_wariantu) = match wariant {
            OptRozdzielczościObrazów::R16 => (16, "_16"),
            OptRozdzielczościObrazów::R32 => (32, "_32"),
            OptRozdzielczościObrazów::R64 => (64, "_64"),
            OptRozdzielczościObrazów::R128 => (128, "_128"),
            OptRozdzielczościObrazów::R256 => (256, "_256"),
            OptRozdzielczościObrazów::R512 => (512, "_512"),
            OptRozdzielczościObrazów::R1k => (1024, "_1k"),
            OptRozdzielczościObrazów::R2k => (2048, "_2k"),
            OptRozdzielczościObrazów::R4k => (4096, "_4k"),
            OptRozdzielczościObrazów::R6k => (6144, "_6k"),
            OptRozdzielczościObrazów::R8k => (8192, "_8k"),
            OptRozdzielczościObrazów::R16k => (16384, "_16k"),
            OptRozdzielczościObrazów::Oryginalna => (0, ""),
        };

        for fdgfshd in bit_depth {
            // let mut oopr = obecna_operacja.lock().await;
            // *oopr += 1;
            // let obecnie = *oopr;
            // drop(oopr);
            //
            // let mut procenciki = procent_progress.lock().await;
            // if (((obecnie as f32 / metryka_operacji as f32)*100.).round() as u8) > *procenciki {
            //     *procenciki = (( obecnie as f32 / metryka_operacji as f32 )*100.).round() as u8;
            //     dbg!("{}%,  obecna operacja:{} / {}",*procenciki,obecnie,metryka_operacji);
            //     let _ = tx.send(LogTxDoBathKonwersjaZdjęć::Rozpoczęto(1, *procenciki)).await;
            // }
            // drop(procenciki);

            aktualizuj_postep(
                &obecna_operacja,
                &procent_progress,
                metryka_operacji,
                &mut tx,
            )
            .await;

            // --- OBSŁUGA BIT DEPTH I FORMATU ---
            let (final_img, nazwa_bd) = match fdgfshd {
                OptFormatyKoloruObrazOgólny::L8 => (
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
                OptFormatyKoloruObrazOgólny::L8a => (
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
                OptFormatyKoloruObrazOgólny::B8 => (
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
                OptFormatyKoloruObrazOgólny::B8a => (
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
                OptFormatyKoloruObrazOgólny::B16 => (
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
                OptFormatyKoloruObrazOgólny::B16a => (
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
                OptFormatyKoloruObrazOgólny::L16 => (
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
                OptFormatyKoloruObrazOgólny::L16a => (
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
                OptFormatyKoloruObrazOgólny::B32 => {
                    //placeholder
                    (DynamicImage::ImageRgb16(bufor.to_rgb16()), "_nima32b")
                }
                //placeholder
                OptFormatyKoloruObrazOgólny::B32a => {
                    (DynamicImage::ImageRgba16(bufor.to_rgba16()), "_nima32bt")
                }
            };
            aktualizuj_postep(
                &obecna_operacja,
                &procent_progress,
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
                &procent_progress,
                metryka_operacji,
                &mut tx,
            )
            .await;
        }
    }

    Ok(())
}
