
use crate::wczytanie_zdjec::aktualizuj_postep;

use enumy::opcje::OptInterpolacja;
use enumy::statusy::LogTxKonw;
use futures::channel::mpsc;
use image::DynamicImage;
use image::imageops::FilterType;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use encodery::avif::{avif_match, avif_zapis};
use encodery::halper::{usun_kanal_alpha, zaszumianie};
use enumy::rozszerzenia::bdepth::BdepthAvif;
use enumy::rozszerzenia::kolor::ForAvifChroma;
use enumy::rozszerzenia::kompresje::ForAvifKompresja;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
#[allow(clippy::too_many_arguments)]
pub async fn edycja_avif(
    bufor: DynamicImage,
    rozdzielczości: &Vec<Rozdzielczości>,
    ścieżka_wyjściowa: &Path,
    ścieżka_dopełniająca: &String,
    opt_interpolacja: &OptInterpolacja,
    nazwa_pliku: &str,
    lossy: &Option<u8>,
    bit_depth: &Vec<BdepthAvif>,
    zaszumianie_zmienna: Option<u8>,
    alfa_rgb: &(u16, u16, u16),
    metoda_kompresji: &ForAvifKompresja,
    szybkość:i32,
    chrummaaa:&ForAvifChroma,
    metryka_operacji: u32,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: mpsc::Sender<LogTxKonw>,
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
        let (docelowy_wymiar, _nazwa_wariantu) = match wariant {
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
            Rozdzielczości::Oryginalna => (0, ""), // 0 jako flag dla oryginału
        };
        // dbg!("jestem za match z wariantami i rozdzielczosciami");
        // *obecna_operacja +=1;

        for wybór in bit_depth {
            // dbg!("jestem w pętli bit_depth");

            aktualizuj_postep(
                &obecna_operacja,
                
                metryka_operacji,
                &mut tx,
            )
            .await;



            let buforeczek = match wybór{
                BdepthAvif::Rgb8 => {
                    if docelowy_wymiar == 0 {
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
                    }
                }
                BdepthAvif::Rgb8Alpha => {
                    if docelowy_wymiar == 0 {
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
                    }
                }
                BdepthAvif::Rgb10 => {
                    if docelowy_wymiar == 0 {
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
                    }
                }
                BdepthAvif::Rgb10Alpha => {
                    if docelowy_wymiar == 0 {
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
                    }

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
                
                metryka_operacji,
                &mut tx,
            ).await;
            avif_zapis(avf_img, nazwa_organu, ścieżka_wyjściowa, ścieżka_dopełniająca, nazwa_pliku, *lossy, szybkość, chrummaaa, metoda_kompresji).await?;

            // 3. Aktualizacja postępu
            aktualizuj_postep(
                &obecna_operacja,
                
                metryka_operacji,
                &mut tx,
            ).await;


        }
    }


    Ok(())
}


