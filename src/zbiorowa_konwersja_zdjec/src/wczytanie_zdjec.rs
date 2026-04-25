use futures::SinkExt;
use futures::channel::mpsc::Sender;
use image::ImageDecoder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use enumy::statusy::LogTxDoBathKonwersjaZdjęć;

pub fn wczytaj_zdjęcie(
    ścieżka: PathBuf,
) -> Result<(DynamicImage, String, Option<Vec<u8>>), std::io::Error> {
    let bajty = std::fs::read(&ścieżka)?;
    let nazwa = ścieżka
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("nieznany")
        .to_string();
    let rozszerzenie = ścieżka
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 1. Najpierw przygotowujemy surowe bajty (rozpakowane lub nie)
    let dane_obrazu = match rozszerzenie.as_str() {
        "zst" => {
            let mut decoder = zstd::stream::read::Decoder::new(&bajty[..])?;
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            rozpakowane
        }
        "bz2" => {
            let mut decoder = bzip2::read::BzDecoder::new(&bajty[..]);
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            rozpakowane
        }
        "xz" => {
            let mut decoder = xz2::read::XzDecoder::new(&bajty[..]);
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            rozpakowane
        }
        _ => bajty,
    };

    // 2. Tworzymy Reader, który automatycznie rozpozna format (JPG, PNG, itp.)
    // let cursor = std::io::Cursor::new(&dane_obrazu);
    // let reader = image::ImageReader::new(cursor).with_guessed_format()?;
    // 2. Poprawione rozpoznawanie formatu
    let cursor = std::io::Cursor::new(&dane_obrazu);
    let mut reader = image::ImageReader::new(cursor).with_guessed_format()?;

    // Jeśli automatyczne rozpoznanie po bajtach zawiodło (częste dla TGA)
    if reader.format().is_none() {
        // Sprawdzamy czy to nie był skompresowany TGA lub czy oryginał to TGA
        // Możemy spróbować wymusić format TGA jeśli rozszerzenie na to wskazuje
        if rozszerzenie == "tga"  {
            reader.set_format(image::ImageFormat::Tga);
        }
    }

    // 3. Dobieramy się do dekodera, żeby wyciągnąć EXIF
    let mut decoder = reader.into_decoder().map_err(std::io::Error::other)?;

    // Tutaj wyciągamy EXIF (metoda z traitu ImageDecoder, który wrzuciłeś)
    let exif = decoder.exif_metadata().ok().flatten();

    // 4. Dekodujemy sam obraz
    let img = DynamicImage::from_decoder(decoder).map_err(std::io::Error::other)?;

    Ok((img, nazwa, exif))
}

pub async fn aktualizuj_postep(
    obecna_op: &Arc<Mutex<u32>>,
    procent_pro: &Arc<Mutex<u8>>,
    metryka: u32,
    nadawca: &mut Sender<LogTxDoBathKonwersjaZdjęć>,
) {
    async move {
        let mut oopr = obecna_op.lock().await;
        *oopr += 1;
        let obecnie = *oopr;
        drop(oopr);

        let mut procenciki = procent_pro.lock().await;
        let nowy_procent = ((obecnie as f32 / metryka as f32) * 100.0).round() as u8;

        if nowy_procent > *procenciki {
            *procenciki = nowy_procent;
            let _ = nadawca
                .send(
                    LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćRozpoczęto(
                        1,
                        *procenciki,
                    ),
                )
                .await;
        }
    }
    .await;
}
