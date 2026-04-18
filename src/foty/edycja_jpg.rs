use std::error::Error;
use std::fs::{create_dir, create_dir_all, File};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use futures::channel::mpsc;
use futures::SinkExt;
use image::DynamicImage;
use image::imageops::FilterType;
use tokio::sync::Mutex;
use crate::foty::pomocnicze::usun_kanal_alpha;
use crate::foty::zmiana_fot::{zaszumianie, Interpolacja, Obraz, PostepMieleniaZdjec, Rozdzielczości};

pub async fn edycja_jpg(
    mut bufor:DynamicImage,
    rozdzielczości:&Vec<Rozdzielczości>,
    ścieżka_wyjściowa: &Path,
    ścieżka_dopełniająca:&String,
    interpolacja: &Interpolacja,
    nazwa_pliku: &str,
    // exif: &DaneExif,
    jakość: &u8,
    progresywny:&bool,
    bit_depth: &Vec<Obraz>,
    alfa_rgb:&(u16, u16, u16),
    do_zaszumienia:Option<u8>,
    metryka_operacji:u32,
    obecna_operacja: Arc<Mutex<u32>>,
    procent_progress: Arc<Mutex<u8>>,
    mut tx: mpsc::Sender<PostepMieleniaZdjec>,
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
    let filtr = match interpolacja {
        Interpolacja::Nearest => FilterType::Nearest,
        Interpolacja::Triangle => FilterType::Triangle,
        Interpolacja::CatmullRom => FilterType::CatmullRom,
        Interpolacja::Gaussian => FilterType::Gaussian,
        Interpolacja::Lanczos3 => FilterType::Lanczos3,
    };

    // 3. Iteracja przez wszystkie żądane rozdzielczości
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
            Rozdzielczości::Oryginalna => (0, ""), // 0 jako flag dla oryginału
        };
        // *obecna_operacja +=1;

        for wybór in bit_depth {
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


            let (final_img, nazwa_bd) = match wybór {
                Obraz::L8 => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageLuma8(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_luma8())
                        } else {
                            DynamicImage::ImageLuma8(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_luma8()).resize(docelowy_wymiar, docelowy_wymiar, filtr)
                        }
                    },
                    "_l8b"
                ),
                Obraz::B8 => (
                    {
                        if docelowy_wymiar == 0 {
                            DynamicImage::ImageRgb8(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8())
                        } else {
                            DynamicImage::ImageRgb8(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8()).resize(docelowy_wymiar, docelowy_wymiar, filtr)
                        }
                    },
                    "_8b"
                ),
                _ => {
                    // placeholder
                    (DynamicImage::ImageRgb8(bufor.to_rgb8()), "_nima32b")
                },
            };


            let final_finalv3_temp_final_ostatecznyv5 = match do_zaszumienia {
                Some(xoxo) => zaszumianie(xoxo, final_img),
                None => final_img
            };


            // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
            // 5. Budowanie nazwy pliku: nazwa + wariant + kolor + rozszerzenie
            let finalna_nazwa = format!("{}{}{}.jpg", nazwa_pliku, nazwa_wariantu, nazwa_bd);
            let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
            ścieżka_pliku.push(ścieżka_dopełniająca);
            // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
            if !ścieżka_pliku.exists() {
                create_dir_all(ścieżka_pliku.clone())?;
            }
            ścieżka_pliku.push(finalna_nazwa);
            // println!("{:?}",ścieżka_pliku);
            // 6. Zapis z uwzględnieniem jakości i progresywności
            // Biblioteka 'image' wymaga enkodera dla specyficznych ustawień JPG
            let mut output_file = File::create(&ścieżka_pliku)?;

            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output_file, *jakość);
            //
            // // Jeśli biblioteka wspiera progresywność w tej wersji (ustawienie flagi)
            // // Uwaga: standardowy encoder image-rs nie zawsze ma bezpośredni przełącznik progresywności w prostym API,
            // // ale JpegEncoder zazwyczaj generuje baseline.
            //
            encoder.encode_image(&final_finalv3_temp_final_ostatecznyv5).map_err(std::io::Error::other)?;
            // encoder.encode(
            //     final_finalv3_temp_final_ostatecznyv5.as_bytes(),
            //     final_finalv3_temp_final_ostatecznyv5.width(),
            //     final_finalv3_temp_final_ostatecznyv5.height(),
            //     final_finalv3_temp_final_ostatecznyv5.color().into() // To przekaże ColorType::L8 do enkodera
            // ).map_err(|e| format!("Błąd: {}", e))?;


        } // Koniec pętli rozdzielczości
    }

    Ok(())
}