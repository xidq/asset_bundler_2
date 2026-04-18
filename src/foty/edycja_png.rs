use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use futures::channel::mpsc;
use futures::SinkExt;
use image::{DynamicImage, imageops::FilterType, Rgba, ImageBuffer, GenericImageView, ColorType};
use tokio::sync::Mutex;
use crate::foty::pomocnicze::usun_kanal_alpha;
use crate::foty::zmiana_fot::{Interpolacja, Rozdzielczości, Obraz, Rozszerzenia, zaszumianie, Zaszumianie, PostepMieleniaZdjec};


pub async fn edycja_png(
    bufor: DynamicImage,
    rozdzielczości: &Vec<Rozdzielczości>,
    ścieżka_wyjściowa: &Path,
    ścieżka_dopełniająca:&String,
    interpolacja: &Interpolacja,
    nazwa_pliku: &str,
    // exif: &DaneExif, // PNG rzadko używa EXIF, ale zostawiamy dla spójności
    kompresja: &u8,
    // filtr_alfa: &bool,
    alfa_rgb: &(u16, u16, u16),
    bit_depth: &Vec<Obraz>,
    zaszumianie_zmienna: Option<u8>,
    metryka_operacji:u32,
    obecna_operacja: Arc<Mutex<u32>>,
    procent_progress: Arc<Mutex<u8>>,
    mut tx: mpsc::Sender<PostepMieleniaZdjec>
) -> Result<(), tokio::io::Error> {

    // 1. Wybór filtra interpolacji
    let filtr = match interpolacja {
        Interpolacja::Nearest => FilterType::Nearest,
        Interpolacja::Triangle => FilterType::Triangle,
        Interpolacja::CatmullRom => FilterType::CatmullRom,
        Interpolacja::Gaussian => FilterType::Gaussian,
        Interpolacja::Lanczos3 => FilterType::Lanczos3,
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


        for fdgfshd in bit_depth {
            let mut oopr = obecna_operacja.lock().await;
            *oopr += 1;
            let obecnie = *oopr;
            drop(oopr);
            
            let mut procenciki = procent_progress.lock().await;
            if (((obecnie as f32 / metryka_operacji as f32)*100.).round() as u8) > *procenciki {
                *procenciki = (( obecnie as f32 / metryka_operacji as f32 )*100.).round() as u8;
                println!("{}%,  obecna operacja:{} / {}",*procenciki,obecnie,metryka_operacji);
                let _ = tx.send(PostepMieleniaZdjec::Rozpoczęto(1, *procenciki)).await;
            }
            drop(procenciki);

            // --- OBSŁUGA BIT DEPTH I FORMATU ---
            let (final_img, nazwa_bd) = match fdgfshd {
                Obraz::L8 =>(
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageLuma8(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_luma8())
                        }else{
                            DynamicImage::ImageLuma8(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_luma8()).resize(docelowy_wymiar, docelowy_wymiar, filtr)
                        }
                    },
                    "_l8b"
                ),
                Obraz::L8a => ({
                       if docelowy_wymiar == 0 {
                           DynamicImage::ImageLumaA8(bufor.to_luma_alpha8())
                       }else{
                           DynamicImage::ImageLumaA8(bufor.to_luma_alpha8()).resize(docelowy_wymiar, docelowy_wymiar, filtr)
                       }
                   }, "_l8bt"),
                Obraz::B8 => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageRgb8(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8())
                        }else{
                            DynamicImage::ImageRgb8(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8()).resize(docelowy_wymiar, docelowy_wymiar, filtr)
                        }
                    },
                    "_8b"
                ),
                Obraz::B8a => ({
                       if docelowy_wymiar == 0 {
                           DynamicImage::ImageRgba8(bufor.to_rgba8())
                       }else{
                           DynamicImage::ImageRgba8(bufor.to_rgba8()).resize(docelowy_wymiar, docelowy_wymiar, filtr)
                       }
                   }, "_8bt"),
                Obraz::B16 => ({
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageRgb16(bufor.to_rgb16())
                        }else{
                            DynamicImage::ImageRgb16(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb16()).resize(docelowy_wymiar, docelowy_wymiar, filtr)
                        }
                   }, "_16b"),
                Obraz::B16a => ({
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageRgba16(bufor.to_rgba16())
                        }else{
                            DynamicImage::ImageRgba16(bufor.to_rgba16()).resize(docelowy_wymiar, docelowy_wymiar, filtr)
                        }
                    }, "_16bt"),
                Obraz::L16 => ({
                       if docelowy_wymiar == 0 {
                           DynamicImage::ImageLuma16(bufor.to_luma16())
                       }else{
                           DynamicImage::ImageLuma16(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_luma16()).resize(docelowy_wymiar, docelowy_wymiar, filtr)
                       }
                   }, "_16b"),
                Obraz::L16a => ({
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageLumaA16(bufor.to_luma_alpha16())
                        }else{
                            DynamicImage::ImageLumaA16(bufor.to_luma_alpha16()).resize(docelowy_wymiar, docelowy_wymiar, filtr)
                        }
                    }, "_16bt"),
                Obraz::B32 => {
                //placeholder
                    (DynamicImage::ImageRgb16(bufor.to_rgb16()), "_nima32b")
                },
                //placeholder
                Obraz::B32a => (DynamicImage::ImageRgba16(bufor.to_rgba16()), "_nima32bt"),
            };
            
            let final_final_final_v3_xd = match zaszumianie_zmienna{
                Some(x) => zaszumianie(x,final_img),
                None => final_img,
            };


            // 4. Budowanie nazwy
            let finalna_nazwa = format!("{}{}{}.png", nazwa_pliku, nazwa_wariantu, nazwa_bd);
            let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
            ścieżka_pliku.push(ścieżka_dopełniająca);
            if !ścieżka_pliku.exists(){
                create_dir_all(ścieżka_pliku.clone())?;
            }
            ścieżka_pliku.push(finalna_nazwa);

            // 5. Zapis z kompresją
            let f = File::create(&ścieżka_pliku)?;

            // Mapowanie u8 kompresji (0-9) na poziomy PNG (Best, Fast, Default)
            let speed = match *kompresja {
                0 => image::codecs::png::CompressionType::Uncompressed,
                _ => image::codecs::png::CompressionType::Level(*kompresja),
            };

            let encoder = image::codecs::png::PngEncoder::new_with_quality(
                f,
                speed,
                image::codecs::png::FilterType::Adaptive,
            );

        final_final_final_v3_xd.write_with_encoder(encoder).map_err(std::io::Error::other)?;

    }}

    Ok(())
}