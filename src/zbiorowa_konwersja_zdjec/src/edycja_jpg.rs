use crate::wczytanie_zdjec::aktualizuj_postep;
use enumy::opcje::OptInterpolacja;
use enumy::statusy::LogTxKonw;
use futures::channel::mpsc::Sender;
use image::imageops::FilterType;
use image::DynamicImage;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use encodery::jpg::{jpg_match, jpg_zapis};
use enumy::rozszerzenia::bdepth::BdepthJpg;
use enumy::rozszerzenia::kolor::{ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
#[allow(clippy::too_many_arguments)]
pub async fn edycja_jpg(
    bufor: DynamicImage,
    rozdzielczości: &Vec<Rozdzielczości>,
    ścieżka_wyjściowa: &Path,
    opt_interpolacja: &OptInterpolacja,
    nazwa_pliku: &str,
    zbiór_danych: (&u8, &bool, &Vec<BdepthJpg>, &ForJpgSamplingFac, &ForJpgQuant, &u8),
    alfa_rgb: &(u16, u16, u16),
    do_zaszumienia: Option<u8>,
    metryka_operacji: u32,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<LogTxKonw>,
) -> Result<(), tokio::io::Error> {

    let filtr = match opt_interpolacja {
        OptInterpolacja::Nearest => FilterType::Nearest,
        OptInterpolacja::Triangle => FilterType::Triangle,
        OptInterpolacja::CatmullRom => FilterType::CatmullRom,
        OptInterpolacja::Gaussian => FilterType::Gaussian,
        OptInterpolacja::Lanczos3 => FilterType::Lanczos3,
    };


    for wariant in rozdzielczości {
        let (docelowy_wymiar, nazwa_wariantu) = match wariant {
            Rozdzielczości::R16 => (16, "_16"),
            Rozdzielczości::R32 => (32, "_32"),
            Rozdzielczości::R64 => (64, "_64"),
            Rozdzielczości::R128 => (128, "_128"),
            Rozdzielczości::R256 => (256, "_256"),
            Rozdzielczości::R512 => (512, "_512"),
            Rozdzielczości::R1k => (1024, "_1024"),
            Rozdzielczości::R2k => (2048, "_2k"),
            Rozdzielczości::R4k => (4096, "_4k"),
            Rozdzielczości::R6k => (6144, "_6k"),
            Rozdzielczości::R8k => (8192, "_8k"),
            Rozdzielczości::R16k => (16384, "_16k"),
            Rozdzielczości::Oryginalna => (0, ""),
        };


        for wybór in zbiór_danych.2 {

            aktualizuj_postep(
                &obecna_operacja,
                
                metryka_operacji,
                &mut tx,
            )
            .await;
            
            let (final_img, nazwa_bd) = jpg_match(&bufor, docelowy_wymiar, filtr, alfa_rgb,  wybór).await?;
                
    
            aktualizuj_postep(
                &obecna_operacja,
                
                metryka_operacji,
                &mut tx,
            )
            .await;


            jpg_zapis(final_img,nazwa_bd,ścieżka_wyjściowa,do_zaszumienia,nazwa_pliku,nazwa_wariantu,zbiór_danych.0,zbiór_danych.1,zbiór_danych.3,zbiór_danych.4,zbiór_danych.5).await?;

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
