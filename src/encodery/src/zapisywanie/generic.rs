use std::path::PathBuf;
use crate::send::wyslij_status;
use crate::zapisywanie::avif::avif_match;
use crate::zapisywanie::ff::ff_match;
use crate::zapisywanie::jpg::jpg_match;
use crate::zapisywanie::png::png_match;
use crate::zapisywanie::qoi::qoi_match;
use crate::zapisywanie::tga::tga_match;
use crate::zapisywanie::webp::webp_match;
use enumy::opcje::{OptInterpolacja, OptIstniejePlik};
use enumy::przetwarzanie::{DaneDoPrzetwarzania, TypyPrzetwarzania};
use enumy::rozszerzenia::bdepth_impl::{BdepthEnum, BitDepth};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use image::imageops::FilterType;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::zapisywanie::exr::exr_match;

pub struct InneDane<T>{
    pub wymiar: u32,
    pub bdepth: T,
    pub nazwa_wariantu: String,
    pub filtr: FilterType,
    pub zastepowanie: OptIstniejePlik,
}

pub async fn zapisywanie_generic<T, F, G>(
    dane: F,
    metodyka: OptIstniejePlik,
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
                        let data = InneDane{
                            wymiar: docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu: nazwa_wariantu.to_string(),
                            filtr,
                            zastepowanie: metodyka,
                        };
                        jpg_match(
                            danee,
                            data,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                },
                TypyPrzetwarzania::PrzPng(danee) => {
                    if let BdepthEnum::Png(bdepth) = wybór.clone().jako_enum(){
                        let data = InneDane{
                            wymiar: docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu: nazwa_wariantu.to_string(),
                            filtr,
                            zastepowanie: metodyka,
                        };
                        png_match(
                            danee,
                            data,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzAvif(danee) => {
                    if let BdepthEnum::Avif(bdepth) = wybór.clone().jako_enum(){
                        let data = InneDane{
                            wymiar: docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu: nazwa_wariantu.to_string(),
                            filtr,
                            zastepowanie: metodyka,
                        };
                        avif_match(
                            danee,
                            data,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzWebp(danee) => {
                    if let BdepthEnum::Webp(bdepth) = wybór.clone().jako_enum(){
                        let data = InneDane{
                            wymiar: docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu: nazwa_wariantu.to_string(),
                            filtr,
                            zastepowanie: metodyka,
                        };
                        webp_match(
                            danee,
                            data,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzQoi(danee) => {
                    if let BdepthEnum::Qoi(bdepth) = wybór.clone().jako_enum(){
                        let data = InneDane{
                            wymiar: docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu: nazwa_wariantu.to_string(),
                            filtr,
                            zastepowanie: metodyka,
                        };
                        qoi_match(
                            danee,
                            data,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzTga(danee) => {
                    if let BdepthEnum::Tga(bdepth) = wybór.clone().jako_enum(){
                        let data = InneDane{
                            wymiar: docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu: nazwa_wariantu.to_string(),
                            filtr,
                            zastepowanie: metodyka,
                        };
                        tga_match(
                            danee,
                            data,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzFf(danee) => {
                    if let BdepthEnum::Ff(bdepth) = wybór.clone().jako_enum(){
                        let data = InneDane{
                            wymiar: docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu: nazwa_wariantu.to_string(),
                            filtr,
                            zastepowanie: metodyka,
                        };
                        ff_match(
                            danee,
                            data,
                            metryka_operacji,
                            obecna_operacja.clone(),
                            tx.clone()
                        ).await?
                    }
                }
                TypyPrzetwarzania::PrzExr(danee) => {
                    if let BdepthEnum::Exr(bdepth) = wybór.clone().jako_enum(){
                        let data = InneDane{
                            wymiar: docelowy_wymiar,
                            bdepth,
                            nazwa_wariantu: nazwa_wariantu.to_string(),
                            filtr,
                            zastepowanie: metodyka,
                        };
                        exr_match(
                            danee,
                            data,
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

pub fn get_higher_tier_copy(sciezka: PathBuf) -> PathBuf {
    // Jeśli plik w ogóle nie istnieje, zwracamy oryginalną ścieżkę bez zmian
    if !sciezka.exists() {
        return sciezka;
    }

    // Wyciągamy czystą nazwę pliku (bez folderów i bez rozszerzenia)
    // Jeśli nie uda się pobrać (np. ścieżka to ".."), bezpiecznym fallbackiem jest cała ścieżka
    let nazwa_bazowa = match sciezka.file_stem() {
        Some(stem) => stem.to_string_lossy().into_owned(),
        None => return sciezka,
    };

    // Pobieramy rozszerzenie (np. "jpg"). Jeśli brak, używamy pustego ciągu
    let rozszerzenie = match sciezka.extension() {
        Some(ext) => format!(".{}", ext.to_string_lossy()),
        None => String::new(),
    };

    let mut licznik = 1;
    let mut nowa_sciezka = sciezka.clone();

    // Pętla kręci się tak długo, jak długo generowana ścieżka istnieje na dysku
    while nowa_sciezka.exists() {
        // Składamy nową nazwę: "nazwa (1).jpg"
        let nowa_nazwa = format!("{} ({}){}", nazwa_bazowa, licznik, rozszerzenie);

        // Podmieniamy samą końcówkę ścieżki
        nowa_sciezka = sciezka.with_file_name(nowa_nazwa);

        licznik += 1;
    }

    nowa_sciezka
}