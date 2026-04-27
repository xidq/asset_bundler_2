
use crate::wczytanie_zdjec::aktualizuj_postep;

use enumy::opcje::{AvifChroma, AvifMetodaKompresji, OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuAvif, OptInterpolacja, OptRozdzielczościObrazów};
use enumy::statusy::LogTxDoBathKonwersjaZdjęć;
use futures::SinkExt;
use futures::channel::mpsc;
use image::{DynamicImage, GenericImageView};
use image::imageops::FilterType;
use std::fs::{File, create_dir_all};
use std::path::Path;
use std::sync::Arc;
use libheif_rs::{Channel, Chroma, ColorSpace, CompressionFormat, EncoderParameterValue, EncoderQuality, HeifContext, Image, LibHeif, RgbChroma};
use tokio::sync::Mutex;
use encodery::avif::{avif_match, avif_zapis};
use encodery::halper::{usun_kanal_alpha, zaszumianie};

pub async fn edycja_avif(
    bufor: DynamicImage,
    rozdzielczości: &Vec<OptRozdzielczościObrazów>,
    ścieżka_wyjściowa: &Path,
    ścieżka_dopełniająca: &String,
    opt_interpolacja: &OptInterpolacja,
    nazwa_pliku: &str,
    lossy: &Option<u8>,
    bit_depth: &Vec<OptFormatyKoloruObrazuAvif>,
    zaszumianie_zmienna: Option<u8>,
    alfa_rgb: &(u16, u16, u16),
    metoda_kompresji: &AvifMetodaKompresji,
    szybkość:i32,
    chrummaaa:&AvifChroma,
    metryka_operacji: u32,
    obecna_operacja: Arc<Mutex<u32>>,
    procent_progress: Arc<Mutex<u8>>,
    mut tx: mpsc::Sender<LogTxDoBathKonwersjaZdjęć>,
) -> Result<(), tokio::io::Error> {
    
    // dbg!("jestem w fn avif");

    let filtr = match opt_interpolacja {
        OptInterpolacja::Nearest => FilterType::Nearest,
        OptInterpolacja::Triangle => FilterType::Triangle,
        OptInterpolacja::CatmullRom => FilterType::CatmullRom,
        OptInterpolacja::Gaussian => FilterType::Gaussian,
        OptInterpolacja::Lanczos3 => FilterType::Lanczos3,
    };
    // dbg!("jestem po filtrze filtr");

    // 3. Iteracja przez wszystkie żądane rozdzielczości
    for wariant in rozdzielczości {
        // dbg!("jestem w pętli wariant");
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
            OptRozdzielczościObrazów::Oryginalna => (0, ""), // 0 jako flag dla oryginału
        };
        // dbg!("jestem za match z wariantami i rozdzielczosciami");
        // *obecna_operacja +=1;

        for wybór in bit_depth {
            // dbg!("jestem w pętli bit_depth");

            aktualizuj_postep(
                &obecna_operacja,
                &procent_progress,
                metryka_operacji,
                &mut tx,
            )
            .await;



            let buforeczek = match wybór{
                OptFormatyKoloruObrazuAvif::B8 => {
                    let res = if docelowy_wymiar == 0 {
                        DynamicImage::ImageRgb8(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8())
                    } else {
                        DynamicImage::ImageRgb8(
                            usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8(),
                        )
                            .resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                    };
                    res
                }
                OptFormatyKoloruObrazuAvif::B8a => {
                    let res = if docelowy_wymiar == 0 {
                        DynamicImage::ImageRgba8(bufor.to_rgba8())
                    } else {
                        DynamicImage::ImageRgba8(
                            bufor.to_rgba8(),
                        )
                            .resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                    };
                    res
                }
                OptFormatyKoloruObrazuAvif::B10 => {
                    let res = if docelowy_wymiar == 0 {
                        DynamicImage::ImageRgb16(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb16())
                    } else {
                        DynamicImage::ImageRgb16(
                            usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb16(),
                        )
                            .resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                    };
                    res
                }
                OptFormatyKoloruObrazuAvif::B10a => {
                    let res = if docelowy_wymiar == 0 {
                        DynamicImage::ImageRgba16(bufor.to_rgba16())
                    } else {
                        DynamicImage::ImageRgba16(
                            bufor.to_rgba16(),
                        )
                            .resize(
                                docelowy_wymiar,
                                docelowy_wymiar,
                                filtr,
                            )
                    };
                    res
                }
            };
            // dbg!("jestem po aktualizacji postępu");
            let final_final_final_v3_xd = match zaszumianie_zmienna {
                Some(x) => zaszumianie(x, buforeczek),
                None => buforeczek,
            };


            let (avf_img,nazwa_organu) = avif_match(final_final_final_v3_xd, wybór).await?;

            // let mnożnik =
            //     if avf_img.height() > avf_img.width() {
            //         (avf_img.height() as f64 / docelowy_wymiar as f64).round().clamp(0_f64,u32::MAX as f64) as u32
            //     } else {
            //         (avf_img.width() as f64 / docelowy_wymiar as f64).round().clamp(0_f64,u32::MAX as f64) as u32
            //     };
            // let (wymiar_w, wymiar_h) =
            //     (avf_img.width() * mnożnik, avf_img.height() * mnożnik );


            // avf_img.scale(wymiar_w, wymiar_h, None).expect("TODO: panic message");
            aktualizuj_postep(
                &obecna_operacja,
                &procent_progress,
                metryka_operacji,
                &mut tx,
            ).await;
            avif_zapis(avf_img, nazwa_organu, ścieżka_wyjściowa, ścieżka_dopełniająca, nazwa_pliku, *lossy, szybkość, &chrummaaa, &metoda_kompresji).await?;

            // 3. Aktualizacja postępu
            aktualizuj_postep(
                &obecna_operacja,
                &procent_progress,
                metryka_operacji,
                &mut tx,
            ).await;


        }
    }


    Ok(())
}


