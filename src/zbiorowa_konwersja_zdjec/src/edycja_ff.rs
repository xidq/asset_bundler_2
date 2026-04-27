use crate::wczytanie_zdjec::aktualizuj_postep;
use bzip2::Compression;
use bzip2::write::BzEncoder;
use enumy::opcje::{OptInterpolacja, OptMetodaKompresjiZdjecia, OptRozdzielczościObrazów};
use enumy::statusy::LogTxDoBathKonwersjaZdjęć;
use futures::SinkExt;
use futures::channel::mpsc;
use futures::channel::mpsc::Sender;
use image::DynamicImage;
use image::imageops::FilterType;
use std::error::Error;
use std::fs::{File, create_dir, create_dir_all};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use xz2::write::XzEncoder;
use encodery::halper::{usun_kanal_alpha, zaszumianie};

pub async fn edycja_ff(
    mut bufor: DynamicImage,
    rozdzielczości: &Vec<OptRozdzielczościObrazów>,
    ścieżka_wyjściowa: &Path,
    ścieżka_dopełniająca: &String,
    OptInterpolacja: &OptInterpolacja,
    nazwa_pliku: &str,
    alfa_rgb: &(u16, u16, u16),
    do_zaszumienia: Option<u8>,
    metryka_operacji: u32,
    obecna_operacja: Arc<Mutex<u32>>,
    procent_progress: Arc<Mutex<u8>>,
    wybrana_kompresja: &OptMetodaKompresjiZdjecia,
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
        aktualizuj_postep(
            &obecna_operacja,
            &procent_progress,
            metryka_operacji,
            &mut tx,
        )
        .await;

        // {
        //     let mut oopr = obecna_operacja.lock().await;
        //     *oopr += 1;
        //     let obecnie = *oopr;
        //     drop(oopr);
        //
        //     let mut procenciki = procent_progress.lock().await;
        //     if (((obecnie as f32 / metryka_operacji as f32) * 100.).round() as u8) > *procenciki {
        //         *procenciki = ((obecnie as f32 / metryka_operacji as f32) * 100.).round() as u8;
        //         dbg!("{}%,  obecna operacja:{} / {}",*procenciki,obecnie,metryka_operacji);
        //         let _ = tx.send(LogTxDoBathKonwersjaZdjęć::Rozpoczęto(1, *procenciki)).await;
        //     }
        //     drop(procenciki);
        // }

        let bombozooo = if docelowy_wymiar == 0 {
            DynamicImage::ImageRgba16(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgba16())
        } else {
            DynamicImage::ImageRgba16(usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgba16())
                .resize(docelowy_wymiar, docelowy_wymiar, filtr)
        };

        let final_finalv3_temp_final_ostatecznyv5 = match do_zaszumienia {
            Some(xoxo) => zaszumianie(xoxo, bombozooo),
            None => bombozooo,
        };

        let lambadziara = wybrana_kompresja;
        // for lambadziara in wybrana_kompresja {
        let dodatkowa_nazwa = match lambadziara {
            OptMetodaKompresjiZdjecia::Zstd(_) => ".zst",
            OptMetodaKompresjiZdjecia::Bzip2(_) => ".bz2",
            OptMetodaKompresjiZdjecia::Xz(_) => ".xz",
            OptMetodaKompresjiZdjecia::Brak => "",
        };

        // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
        let finalna_nazwa = format!("{}{}.ff{}", nazwa_pliku, nazwa_wariantu, dodatkowa_nazwa);
        let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
        ścieżka_pliku.push(ścieżka_dopełniająca);
        // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
        if !ścieżka_pliku.exists() {
            create_dir_all(ścieżka_pliku.clone())?;
        }
        ścieżka_pliku.push(finalna_nazwa);

        let output_file = File::create(&ścieżka_pliku)?;

        aktualizuj_postep(
            &obecna_operacja,
            &procent_progress,
            metryka_operacji,
            &mut tx,
        )
        .await;
        match lambadziara {
            OptMetodaKompresjiZdjecia::Zstd(x) => {
                //kompresja 1-22 || 3def
                let compressor = zstd::Encoder::new(
                    output_file,
                    (*x as f32 / 9.).round().clamp(1., 22.) as i32,
                )?
                .auto_finish();

                // 2. Dodajesz buforowanie dla wydajności
                let buffered_writer = std::io::BufWriter::new(compressor);

                // 3. Reszta bez zmian
                let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
                final_finalv3_temp_final_ostatecznyv5
                    .write_with_encoder(encoder)
                    .map_err(std::io::Error::other)?;
            }
            OptMetodaKompresjiZdjecia::Bzip2(x) => {
                //kompresja 1-9
                let bz_encoder = BzEncoder::new(
                    output_file,
                    Compression::new((*x as f32 / 22.).round().clamp(1., 9.) as u32),
                );
                let buffered_writer = std::io::BufWriter::new(bz_encoder);

                let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
                final_finalv3_temp_final_ostatecznyv5
                    .write_with_encoder(encoder)
                    .map_err(std::io::Error::other)?
            }
            OptMetodaKompresjiZdjecia::Xz(x) => {
                // 1-9 || 6def
                let xz_encoder =
                    XzEncoder::new(output_file, (*x as f32 / 22.).round().clamp(1., 9.) as u32);
                let buffered_writer = std::io::BufWriter::new(xz_encoder);

                let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
                final_finalv3_temp_final_ostatecznyv5
                    .write_with_encoder(encoder)
                    .map_err(std::io::Error::other)?
            }
            OptMetodaKompresjiZdjecia::Brak => {
                let buffered_writer = std::io::BufWriter::new(output_file);

                let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);

                final_finalv3_temp_final_ostatecznyv5
                    .write_with_encoder(encoder)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
            }
        }

        aktualizuj_postep(
            &obecna_operacja,
            &procent_progress,
            metryka_operacji,
            &mut tx,
        )
        .await;
    }

    Ok(())
}
