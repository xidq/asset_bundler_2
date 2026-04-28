use crate::wczytanie_zdjec::aktualizuj_postep;
use enumy::opcje::{JpgQuant, JpgSamplingFac, OptFormatyKoloruObrazOgólny, OptInterpolacja, OptRozdzielczościObrazów};
use enumy::statusy::LogTxDoBathKonwersjaZdjęć;
use futures::SinkExt;
use futures::channel::mpsc;
use futures::channel::mpsc::Sender;
use image::imageops::FilterType;
use image::{DynamicImage, ImageEncoder};
use std::error::Error;
use std::fs::{File, create_dir, create_dir_all};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use jpeg_encoder::{ColorType, Encoder, QuantizationTableType, SamplingFactor};
use tokio::sync::Mutex;
use encodery::halper::{usun_kanal_alpha, zaszumianie};
use encodery::jpg::{jpg_match, jpg_zapis};

pub async fn edycja_jpg(
    bufor: DynamicImage,
    rozdzielczości: &Vec<OptRozdzielczościObrazów>,
    ścieżka_wyjściowa: &Path,
    opt_interpolacja: &OptInterpolacja,
    nazwa_pliku: &str,
    zbiór_danych: (&u8, &bool, &Vec<OptFormatyKoloruObrazOgólny>, &JpgSamplingFac, &JpgQuant, &u8),
    alfa_rgb: &(u16, u16, u16),
    do_zaszumienia: Option<u8>,
    metryka_operacji: u32,
    obecna_operacja: Arc<Mutex<u32>>,
    procent_progress: Arc<Mutex<u8>>,
    mut tx: Sender<LogTxDoBathKonwersjaZdjęć>,
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
            OptRozdzielczościObrazów::R16 => (16, "_16"),
            OptRozdzielczościObrazów::R32 => (32, "_32"),
            OptRozdzielczościObrazów::R64 => (64, "_64"),
            OptRozdzielczościObrazów::R128 => (128, "_128"),
            OptRozdzielczościObrazów::R256 => (256, "_256"),
            OptRozdzielczościObrazów::R512 => (512, "_512"),
            OptRozdzielczościObrazów::R1k => (1024, "_1024"),
            OptRozdzielczościObrazów::R2k => (2048, "_2k"),
            OptRozdzielczościObrazów::R4k => (4096, "_4k"),
            OptRozdzielczościObrazów::R6k => (6144, "_6k"),
            OptRozdzielczościObrazów::R8k => (8192, "_8k"),
            OptRozdzielczościObrazów::R16k => (16384, "_16k"),
            OptRozdzielczościObrazów::Oryginalna => (0, ""),
        };


        for wybór in zbiór_danych.2 {

            aktualizuj_postep(
                &obecna_operacja,
                &procent_progress,
                metryka_operacji,
                &mut tx,
            )
            .await;
            
            let (final_img, nazwa_bd) = jpg_match(&bufor, docelowy_wymiar, filtr, alfa_rgb,  wybór).await?;
                
    
            aktualizuj_postep(
                &obecna_operacja,
                &procent_progress,
                metryka_operacji,
                &mut tx,
            )
            .await;


            jpg_zapis(final_img,nazwa_bd,ścieżka_wyjściowa,do_zaszumienia,nazwa_pliku,nazwa_wariantu,zbiór_danych.0,zbiór_danych.1,zbiór_danych.3,zbiór_danych.4,zbiór_danych.5).await?;

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
