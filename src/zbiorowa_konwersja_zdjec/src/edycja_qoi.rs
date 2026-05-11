use crate::wczytanie_zdjec::aktualizuj_postep;
use enumy::opcje::OptInterpolacja;
use enumy::statusy::LogTxKonw;
use futures::channel::mpsc::Sender;
use image::ImageEncoder;
use image::{imageops::FilterType, DynamicImage, GenericImageView};
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use encodery::halper::{usun_kanal_alpha, zaszumianie};
use enumy::rozszerzenia::bdepth::BdepthQoi;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;

pub async fn edycja_qoi(
    bufor: DynamicImage,
    rozdzielczości: &Vec<Rozdzielczości>,
    ścieżka_wyjściowa: &Path,
    ścieżka_dopełniająca: &String,
    OptInterpolacja: &OptInterpolacja,
    nazwa_pliku: &str,
    alfa_rgb: &(u16, u16, u16),
    bit_depth: &Vec<BdepthQoi>,
    zaszumianie_zmienna: Option<u8>,
    metryka_operacji: u32,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<LogTxKonw>,
) -> Result<(), tokio::io::Error> {
    dbg!("jestem w qoi");

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
        // *obecna_operacja +=1;

        // if (((*obecna_operacja as f32 / metryka_operacji as f32)*100.).round() as u8) > *procent_progress {
        //     *procent_progress = (( *obecna_operacja as f32 / metryka_operacji as f32 )*100.).round() as u8;
        //     println!("{}%,  obecna operacja:{} / {}",procent_progress,obecna_operacja,metryka_operacji);
        //     let _ = tx.send(LogTxDoBathKonwersjaZdjęć::Rozpoczęto(1,*procent_progress)).await;
        // }

        for fdgfshd in bit_depth {
            dbg!("rozpoczynam loop qoi");
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
                
                metryka_operacji,
                &mut tx,
            )
            .await;

            // --- OBSŁUGA BIT DEPTH I FORMATU ---
            let (final_img, nazwa_bd, color_type, szer, wys) = match fdgfshd {
                BdepthQoi::Color24 => {
                    // 1. Usuwamy alfę i przygotowujemy RGB8
                    dbg!("[debug] qoi tc24");
                    let img = usun_kanal_alpha(bufor.clone(), *alfa_rgb);
                    let res = if docelowy_wymiar == 0 {
                        match zaszumianie_zmienna {
                            Some(x) => zaszumianie(x, img),
                            None => img,
                        }
                        .to_rgb8()
                    } else {
                        match zaszumianie_zmienna {
                            Some(x) => {
                                zaszumianie(x, img.resize(docelowy_wymiar, docelowy_wymiar, filtr))
                            }
                            None => img.resize(docelowy_wymiar, docelowy_wymiar, filtr),
                        }
                        .to_rgb8()
                    };

                    let (width, height) = res.dimensions();
                    (
                        res.into_raw(),
                        "_tc24",
                        image::ExtendedColorType::Rgb8,
                        width,
                        height,
                    )
                }

                BdepthQoi::Color32 => {
                    dbg!("[debug] qoi tc32");
                    // 1. Tutaj zostawiamy alfę, więc prosto do RGBA8
                    let obrazek = bufor.clone();
                    let res = if docelowy_wymiar == 0 {
                        match zaszumianie_zmienna {
                            Some(x) => zaszumianie(x, bufor.clone()),
                            None => bufor.clone(),
                        }
                        .to_rgba8()
                    } else {
                        match zaszumianie_zmienna {
                            Some(x) => zaszumianie(
                                x,
                                bufor
                                    .clone()
                                    .resize(docelowy_wymiar, docelowy_wymiar, filtr),
                            ),
                            None => bufor
                                .clone()
                                .resize(docelowy_wymiar, docelowy_wymiar, filtr),
                        }
                        .to_rgba8()
                    };
                    let (width, height) = res.dimensions();
                    (
                        res.into_raw(),
                        "_tc32",
                        image::ExtendedColorType::Rgba8,
                        width,
                        height,
                    )
                }
            };
            aktualizuj_postep(
                &obecna_operacja,
                
                metryka_operacji,
                &mut tx,
            )
            .await;

            // 4. Budowanie nazwy
            let finalna_nazwa = format!("{}{}{}.qoi", nazwa_pliku, nazwa_wariantu, nazwa_bd);
            let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
            ścieżka_pliku.push(ścieżka_dopełniająca);

            if !ścieżka_pliku.exists() {
                create_dir_all(&ścieżka_pliku)?;
            }
            ścieżka_pliku.push(finalna_nazwa);
            dbg!("ścieżka pliku: {}", &ścieżka_pliku);

            // 5. Zapis z wykorzystaniem enkodera i naszych surowych danych
            let f = File::create(&ścieżka_pliku)?;

            let encoder = image::codecs::qoi::QoiEncoder::new(f);

            encoder
                .write_image(&final_img, szer, wys, color_type)
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
