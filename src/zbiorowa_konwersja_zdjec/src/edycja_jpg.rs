use crate::pomocnicze::usun_kanal_alpha;
use crate::wczytanie_zdjec::aktualizuj_postep;
use crate::zmiana_fot::zaszumianie;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptInterpolacja, OptRozdzielczościObrazów};
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
use tokio::sync::Mutex;

pub async fn edycja_jpg(
    mut bufor: DynamicImage,
    rozdzielczości: &Vec<OptRozdzielczościObrazów>,
    ścieżka_wyjściowa: &Path,
    ścieżka_dopełniająca: &String,
    OptInterpolacja: &OptInterpolacja,
    nazwa_pliku: &str,
    exif_data: &Option<Vec<u8>>,
    jakość: &u8,
    progresywny: &bool,
    bit_depth: &Vec<OptFormatyKoloruObrazOgólny>,
    alfa_rgb: &(u16, u16, u16),
    do_zaszumienia: Option<u8>,
    metryka_operacji: u32,
    obecna_operacja: Arc<Mutex<u32>>,
    procent_progress: Arc<Mutex<u8>>,
    mut tx: Sender<LogTxDoBathKonwersjaZdjęć>,
) -> Result<(), tokio::io::Error> {
    // println!(" [edycja_jpg] ścieżka dopełniająaca: {:?}\nścieżka wyjściowa: {:?}", ścieżka_dopełniająca,ścieżka_wyjściowa);
    // 1. Obsługa koloru (B/W)
    let mut przyrostek_koloru = "";
    // if !*kolor {
    //     bufor = bufor.grayscale();
    //     przyrostek_koloru = "_bw";
    // }
    // println!("[jpg] ma w vec: {:?}",bit_depth);

    // 2. Wybór filtra interpolacji
    let filtr = match OptInterpolacja {
        OptInterpolacja::Nearest => FilterType::Nearest,
        OptInterpolacja::Triangle => FilterType::Triangle,
        OptInterpolacja::CatmullRom => FilterType::CatmullRom,
        OptInterpolacja::Gaussian => FilterType::Gaussian,
        OptInterpolacja::Lanczos3 => FilterType::Lanczos3,
    };

    // 3. Iteracja przez wszystkie żądane rozdzielczości
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
            OptRozdzielczościObrazów::Oryginalna => (0, ""), // 0 jako flag dla oryginału
        };
        // *obecna_operacja +=1;

        for wybór in bit_depth {
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

            let (final_img, nazwa_bd) = match wybór {
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
                _ => {
                    // placeholder
                    (DynamicImage::ImageRgb8(bufor.to_rgb8()), "_nima32b")
                }
            };
            aktualizuj_postep(
                &obecna_operacja,
                &procent_progress,
                metryka_operacji,
                &mut tx,
            )
            .await;

            let final_finalv3_temp_final_ostatecznyv5 = match do_zaszumienia {
                Some(xoxo) => zaszumianie(xoxo, final_img),
                None => final_img,
            };

            // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
            let finalna_nazwa = format!("{}{}{}.jpg", nazwa_pliku, nazwa_wariantu, nazwa_bd);
            let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
            ścieżka_pliku.push(ścieżka_dopełniająca);
            // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
            if !ścieżka_pliku.exists() {
                create_dir_all(ścieżka_pliku.clone())?;
            }
            ścieżka_pliku.push(finalna_nazwa);

            let mut output_file = File::create(&ścieżka_pliku)?;

            let mut encoder =
                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output_file, *jakość);

            if let Some(data) = exif_data {
                // Używamy metody z traitu ImageEncoder
                // Ważne: musisz mieć 'use image::ImageEncoder;' na górze pliku!
                let _ = encoder.set_exif_metadata(data.clone());
            }

            // encoder.encode_image(&final_finalv3_temp_final_ostatecznyv5).map_err(std::io::Error::other)?;
            let width = final_finalv3_temp_final_ostatecznyv5.width();
            let height = final_finalv3_temp_final_ostatecznyv5.height();
            let color = final_finalv3_temp_final_ostatecznyv5.color().into(); // Konwersja na ExtendedColorType

            // 4. Zapisujemy obraz używając write_image (metoda z traitu ImageEncoder)
            encoder
                .write_image(
                    final_finalv3_temp_final_ostatecznyv5.as_bytes(),
                    width,
                    height,
                    color,
                )
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
