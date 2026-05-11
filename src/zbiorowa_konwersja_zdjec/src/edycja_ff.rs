use crate::wczytanie_zdjec::aktualizuj_postep;
use bzip2::Compression;
use bzip2::write::BzEncoder;
use enumy::opcje::OptInterpolacja;
use enumy::statusy::LogTxKonw;
use futures::channel::mpsc::Sender;
use image::DynamicImage;
use image::imageops::FilterType;
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use xz2::write::XzEncoder;
use encodery::halper::{usun_kanal_alpha, zaszumianie};
use enumy::rozszerzenia::kompresje::ForFfKompresja;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
#[allow(clippy::too_many_arguments)]
pub async fn edycja_ff(
    bufor: DynamicImage,
    rozdzielczości: &Vec<Rozdzielczości>,
    ścieżka_wyjściowa: &Path,
    ścieżka_dopełniająca: &String,
    opt_interpolacja: &OptInterpolacja,
    nazwa_pliku: &str,
    alfa_rgb: &(u16, u16, u16),
    do_zaszumienia: Option<u8>,
    metryka_operacji: u32,
    obecna_operacja: Arc<Mutex<u32>>,
    wybrana_kompresja: &ForFfKompresja,
    mut tx: Sender<LogTxKonw>,
) -> Result<(), tokio::io::Error> {



    let filtr = match opt_interpolacja {
        OptInterpolacja::Nearest => FilterType::Nearest,
        OptInterpolacja::Triangle => FilterType::Triangle,
        OptInterpolacja::CatmullRom => FilterType::CatmullRom,
        OptInterpolacja::Gaussian => FilterType::Gaussian,
        OptInterpolacja::Lanczos3 => FilterType::Lanczos3,
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
        aktualizuj_postep(
            &obecna_operacja,
            
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
            ForFfKompresja::Zstd(_) => ".zst",
            ForFfKompresja::Bzip2(_) => ".bz2",
            ForFfKompresja::Xz(_) => ".xz",
            ForFfKompresja::Brak => "",
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
            
            metryka_operacji,
            &mut tx,
        )
        .await;
        match lambadziara {
            ForFfKompresja::Zstd(x) => {
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
            ForFfKompresja::Bzip2(x) => {
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
            ForFfKompresja::Xz(x) => {
                // 1-9 || 6def
                let xz_encoder =
                    XzEncoder::new(output_file, (*x as f32 / 22.).round().clamp(1., 9.) as u32);
                let buffered_writer = std::io::BufWriter::new(xz_encoder);

                let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
                final_finalv3_temp_final_ostatecznyv5
                    .write_with_encoder(encoder)
                    .map_err(std::io::Error::other)?
            }
            ForFfKompresja::Brak => {
                let buffered_writer = std::io::BufWriter::new(output_file);

                let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);

                final_finalv3_temp_final_ostatecznyv5
                    .write_with_encoder(encoder)
                    .map_err(std::io::Error::other)?
            }
        }

        aktualizuj_postep(
            &obecna_operacja,
            
            metryka_operacji,
            &mut tx,
        )
        .await;
    }

    Ok(())
}
