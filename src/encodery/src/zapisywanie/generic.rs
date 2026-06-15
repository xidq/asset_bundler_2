use crate::zapisywanie::avif::avif_match;
use crate::zapisywanie::exr::exr_match;
use crate::zapisywanie::ff::ff_match;
use crate::zapisywanie::jpg::jpg_match;
use crate::zapisywanie::png::png_match;
use crate::zapisywanie::qoi::qoi_match;
use crate::zapisywanie::tga::tga_match;
use crate::zapisywanie::webp::webp_match;
use enumy::opcje::OptIstniejePlik;
use enumy::przetwarzanie::{DaneDoPrzetwarzania, TypyPrzetwarzania};
use enumy::rozszerzenia::bdepth_impl::{BdepthEnum, BitDepth};
use enumy::send::wyslij_status;
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use image::imageops::FilterType;
use std::path::PathBuf;
use std::sync::Arc;
use image::DynamicImage;
use tokio::sync::Mutex;
use enumy::inne_ui::WskaznikSzumu;
use crate::halper::zaszumianie;

/// data common for every extension here
pub struct InneDane<T>{
    pub wymiar: u32,
    pub bdepth: T,
    pub nazwa_wariantu: &'static str,
    pub filtr: FilterType,
    pub zastepowanie: OptIstniejePlik,
    // pub noising: WskaznikSzumu,
}
/// # Main encoding fn
/// Fn with generics where is decided what to do etc.
/// 
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

    // let filtr = match &dane.interpolacja() {
    //     OptInterpolacja::Nearest => FilterType::Nearest,
    //     OptInterpolacja::Triangle => FilterType::Triangle,
    //     OptInterpolacja::CatmullRom => FilterType::CatmullRom,
    //     OptInterpolacja::Gaussian => FilterType::Gaussian,
    //     OptInterpolacja::Lanczos3 => FilterType::Lanczos3,
    // };
    let filtr = dane.interpolacja().konwertuj();

    for wariant in dane.rozdzielczosci() {

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
                            wymiar: *wariant as u32,
                            bdepth,
                            nazwa_wariantu: wariant.rozszerzenie(),
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
                            wymiar: *wariant as u32,
                            bdepth,
                            nazwa_wariantu: wariant.rozszerzenie(),
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
                            wymiar: *wariant as u32,
                            bdepth,
                            nazwa_wariantu: wariant.rozszerzenie(),
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
                            wymiar: *wariant as u32,
                            bdepth,
                            nazwa_wariantu: wariant.rozszerzenie(),
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
                            wymiar: *wariant as u32,
                            bdepth,
                            nazwa_wariantu: wariant.rozszerzenie(),
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
                            wymiar: *wariant as u32,
                            bdepth,
                            nazwa_wariantu: wariant.rozszerzenie(),
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
                            wymiar: *wariant as u32,
                            bdepth,
                            nazwa_wariantu: wariant.rozszerzenie(),
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
                            wymiar: *wariant as u32,
                            bdepth,
                            nazwa_wariantu: wariant.rozszerzenie(),
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

pub fn operacje(szum: WskaznikSzumu, img: DynamicImage, wymiar: u32, filtr: FilterType) -> DynamicImage {
    if wymiar == 0 {
        match szum {
            WskaznikSzumu::Normalny { .. } | WskaznikSzumu::Perlin { .. } => { zaszumianie(szum, img) }
            WskaznikSzumu::NoNoise => { img }
        }
    } else {
        match szum {
            WskaznikSzumu::Normalny { .. } | WskaznikSzumu::Perlin { .. } => { zaszumianie(szum, img.resize(wymiar, wymiar, filtr)) }
            WskaznikSzumu::NoNoise => { img.resize(wymiar, wymiar, filtr) }
        }
    }
}