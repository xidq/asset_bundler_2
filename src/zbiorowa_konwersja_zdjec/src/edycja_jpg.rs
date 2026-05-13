use crate::wczytanie_zdjec::aktualizuj_postep;
use enumy::opcje::OptInterpolacja;
use enumy::statusy::{LogTxKonw, Logi};
use futures::channel::mpsc::Sender;
use image::imageops::FilterType;
use image::DynamicImage;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use encodery::jpg::{jpg_match, jpg_zapis};
use encodery::send::wyslij_status;
use enumy::przetwarzanie::{DaneDoPrzetwarzania, PrzetwarzanieJpg};
use enumy::rozszerzenia::bdepth::BdepthJpg;
use enumy::rozszerzenia::bdepth_impl::BitDepth;
use enumy::rozszerzenia::kolor::{ForJpgQuant, ForJpgSamplingFac};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
pub async fn edycja_jpg<T, F, G>(
    dane: F,
    metryka_operacji: Option<u32>,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<T>,
) -> Result<(), tokio::io::Error>
where T: Logi,
F: DaneDoPrzetwarzania<G>,
G: BitDepth,
{


    let filtr = match &dane.interpolacja() {
        OptInterpolacja::Nearest => FilterType::Nearest,
        OptInterpolacja::Triangle => FilterType::Triangle,
        OptInterpolacja::CatmullRom => FilterType::CatmullRom,
        OptInterpolacja::Gaussian => FilterType::Gaussian,
        OptInterpolacja::Lanczos3 => FilterType::Lanczos3,
    };


    for wariant in &dane.rozdzielczosci() {
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


        for wybór in &dane.bdepth() {
            let mut oopr = obecna_operacja.lock().await;
            *oopr += 1;
            let obecnie = *oopr;
            drop(oopr);
            wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;

            
            let (final_img, nazwa_bd) = jpg_match(&dane.bufor, docelowy_wymiar, filtr, dane.alpha, &wybór).await?;
                
    
            // aktualizuj_postep(
            //     &obecna_operacja,
            //
            //     metryka_operacji,
            //     &mut tx,
            // )
            // .await;
            let mut oopr = obecna_operacja.lock().await;
            *oopr += 1;
            let obecnie = *oopr;
            drop(oopr);
            wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;



            jpg_zapis(final_img,nazwa_bd,dane.clone(),nazwa_wariantu).await?;

            let mut oopr = obecna_operacja.lock().await;
            *oopr += 1;
            let obecnie = *oopr;
            drop(oopr);
            wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;
        }
    }

    Ok(())
}
