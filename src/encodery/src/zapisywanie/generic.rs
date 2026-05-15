use crate::zapisywanie::jpg::jpg_match;
use crate::send::wyslij_status;
use enumy::opcje::OptInterpolacja;
use enumy::przetwarzanie::{DaneDoPrzetwarzania, TypyPrzetwarzania};
use enumy::rozszerzenia::bdepth_impl::{BdepthEnum, BitDepth};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use image::imageops::FilterType;
use std::sync::Arc;
use tokio::sync::Mutex;
use enumy::rozszerzenia::kolor::DaneDodatkoweZdjec;
use crate::zapisywanie::avif::avif_match;
use crate::zapisywanie::ff::ff_match;
use crate::zapisywanie::png::png_match;
use crate::zapisywanie::qoi::qoi_match;
use crate::zapisywanie::tga::tga_match;
use crate::zapisywanie::webp::webp_match;

pub async fn zapisywanie_generic<T, F, G>(
    dane: F,
    metryka_operacji: Option<u32>,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<T>,
) -> Result<(), tokio::io::Error>
where T: Logi,
      F: DaneDoPrzetwarzania<G> + std::clone::Clone,
G: BitDepth + std::clone::Clone,
{
    
    


    let filtr = match &dane.interpolacja() {
        OptInterpolacja::Nearest => FilterType::Nearest,
        OptInterpolacja::Triangle => FilterType::Triangle,
        OptInterpolacja::CatmullRom => FilterType::CatmullRom,
        OptInterpolacja::Gaussian => FilterType::Gaussian,
        OptInterpolacja::Lanczos3 => FilterType::Lanczos3,
    };


    for wariant in dane.rozdzielczosci() {
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


        for wybór in dane.bdepth() {

            let mut oopr = obecna_operacja.lock().await;
            *oopr += 1;
            let obecnie = *oopr;
            drop(oopr);
            wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;

            match dane.clone().jako_enum(){
                // ImgExtTag::Jpg() =>

                TypyPrzetwarzania::PrzJpg(danee) => {
                    if let BdepthEnum::Jpg(bdepth) = wybór.clone().jako_enum(){
                        jpg_match(
                            danee,
                            docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu.to_string(),
                            filtr,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                },
                TypyPrzetwarzania::PrzPng(danee) => {
                    if let BdepthEnum::Png(bdepth) = wybór.clone().jako_enum(){
                        png_match(
                            danee,
                            docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu.to_string(),
                            filtr,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzAvif(danee) => {
                    if let BdepthEnum::Avif(bdepth) = wybór.clone().jako_enum(){
                        avif_match(
                            danee,
                            docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu.to_string(),
                            filtr,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzWebp(danee) => {
                    if let BdepthEnum::Webp(bdepth) = wybór.clone().jako_enum(){
                        webp_match(
                            danee,
                            docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu.to_string(),
                            filtr,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzQoi(danee) => {
                    if let BdepthEnum::Qoi(bdepth) = wybór.clone().jako_enum(){
                        qoi_match(
                            danee,
                            docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu.to_string(),
                            filtr,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzTga(danee) => {
                    if let BdepthEnum::Tga(bdepth) = wybór.clone().jako_enum(){
                        tga_match(
                            danee,
                            docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu.to_string(),
                            filtr,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzFf(danee) => {
                    if let BdepthEnum::Ff(bdepth) = wybór.clone().jako_enum(){
                        ff_match(
                            danee,
                            docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu.to_string(),
                            filtr,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
            };

        }
    }

    Ok(())
}

