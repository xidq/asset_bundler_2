use crate::dds_export::save_rgba_image_with_mipmaps;
use enumy::dane_do_przetwarzania::DaneDdsPak;
use enumy::rozszerzenia::kompresje::ForDdsKompresja;
use enumy::statusy::LogTxDdsPak;
use futures::channel::mpsc;
use futures::SinkExt;
use image::GenericImageView;
use std::fs;
use std::fs::create_dir_all;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

pub async fn dds_ogarnij_ze_zdjec_do_paczki(
    dane: DaneDdsPak,
    mut tx: mpsc::Sender<LogTxDdsPak>,
) -> Result<(), std::io::Error> {
    let start_czas = Instant::now();
    let _ = tx.send(LogTxDdsPak::Start).await;
    let kompresja = match &dane.kompresja {
        ForDdsKompresja::Fast => {dds::CompressionQuality::Fast}
        ForDdsKompresja::Normal => {dds::CompressionQuality::Normal}
        ForDdsKompresja::High => {dds::CompressionQuality::High}
        ForDdsKompresja::Unreasonable => {dds::CompressionQuality::Unreasonable}
    };
    let mut przerób: f32 = 0.;


    let mut percent: u8 = 0;

    let wynik: Result<(), std::io::Error> = async {
        let mut images_data = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let mut first_image = true;
        let max_plikow = images_data.len() + 3;
        let lololo = match dane.ścieżka_wejściowa{
            None => {Vec::from([PathBuf::from("")])}
            Some(xxx) => {xxx}
        };
        

        dbg!(&max_plikow);

            lololo.iter().try_for_each(|e| -> Result<(), std::io::Error > {
                match e.is_file() {
                    true => {
                        // LOGIKA DLA POJEDYNCZEGO PLIKU
                        let path = e;
                        let rozszerzenie = path
                            .extension()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_lowercase();

                        let bajty = std::fs::read(path)?;

                        let dane_obrazu = match rozszerzenie.as_str() {
                            "zst" => {
                                let mut decoder = zstd::stream::read::Decoder::new(&bajty[..])?;
                                let mut rozpakowane = Vec::new();
                                std::io::Read::read_to_end(&mut decoder, &mut rozpakowane)?;
                                rozpakowane
                            }
                            "bz2" => {
                                let mut decoder = bzip2::read::BzDecoder::new(&bajty[..]);
                                let mut rozpakowane = Vec::new();
                                std::io::Read::read_to_end(&mut decoder, &mut rozpakowane)?;
                                rozpakowane
                            }
                            "xz" => {
                                let mut decoder = xz2::read::XzDecoder::new(&bajty[..]);
                                let mut rozpakowane = Vec::new();
                                std::io::Read::read_to_end(&mut decoder, &mut rozpakowane)?;
                                rozpakowane
                            }
                            _ => bajty,
                        };

                        let cursor = std::io::Cursor::new(&dane_obrazu);
                        let reader = image::ImageReader::new(cursor).with_guessed_format()?;
                        let decoder = reader.into_decoder().map_err(std::io::Error::other)?;
                        let img = image::DynamicImage::from_decoder(decoder).map_err(std::io::Error::other)?;

                        if first_image {
                            (width, height) = img.dimensions();
                            first_image = false;
                        } else {
                            if img.width() != width || img.height() != height {
                                Err(std::io::Error::other(
                                    "Image dimensions do not match",
                                ))?
                            }
                        }
                        images_data.push(img.to_rgba8().into_raw());

                        Ok(())
                    }


                    false => {
                        for entry in WalkDir::new(e)
                            .into_iter()
                            .filter_map(|e| e.ok())
                            .filter(|e| e.file_type().is_file())
                        {
                            dbg!("pętla w dds_wczytywanie_zdjec");
                            let rozszerzenie = entry
                                .path()
                                .extension()
                                .and_then(|s| s.to_str())
                                .unwrap_or("")
                                .to_lowercase();
                            let path = entry.path();
                            let bajty = std::fs::read(path)?;

                            let dane_obrazu = match rozszerzenie.as_str() {
                                "zst" => {
                                    dbg!("dekompresowanie zstd");
                                    let mut decoder = zstd::stream::read::Decoder::new(&bajty[..])?;
                                    let mut rozpakowane = Vec::new();
                                    decoder.read_to_end(&mut rozpakowane)?;
                                    rozpakowane
                                }
                                "bz2" => {
                                    dbg!("dekompresowanie bz2");
                                    let mut decoder = bzip2::read::BzDecoder::new(&bajty[..]);
                                    let mut rozpakowane = Vec::new();
                                    decoder.read_to_end(&mut rozpakowane)?;
                                    rozpakowane
                                }
                                "xz" => {
                                    dbg!("dekompresowanie xz");
                                    let mut decoder = xz2::read::XzDecoder::new(&bajty[..]);
                                    let mut rozpakowane = Vec::new();
                                    decoder.read_to_end(&mut rozpakowane)?;
                                    rozpakowane
                                }
                                _ => {
                                    dbg!("nie trzeba dekompresować");
                                    bajty
                                },
                            };

                            let cursor = std::io::Cursor::new(&dane_obrazu);
                            let reader = image::ImageReader::new(cursor).with_guessed_format()?;
                            let decoder = reader.into_decoder().map_err(std::io::Error::other)?;
                            let img = image::DynamicImage::from_decoder(decoder).map_err(std::io::Error::other)?;

                            if first_image {
                                (width, height) = img.dimensions();
                                first_image = false;
                            } else {
                                if img.width() != width || img.height() != height {
                                    Err(std::io::Error::other(
                                        "Image dimensions do not match",
                                    ))?
                                }
                            }
                            images_data.push(img.to_rgba8().into_raw());

                        } Ok(())
                    }
                }
            }
        )?;





        let refs_to_data: Vec<&[u8]> = images_data.iter().map(|v| v.as_slice()).collect();

        let finalna_nazwa = format!("{}.dds", dane.nazwa);
        let mut ścieżka_pliku = dane.ścieżka_wyjściowa.to_path_buf();

        if !ścieżka_pliku.exists() {
            create_dir_all(&ścieżka_pliku)?;
        }
        ścieżka_pliku.push(finalna_nazwa);
        let mut huehuehue = fs::File::create(ścieżka_pliku)?;

        save_rgba_image_with_mipmaps(
            &mut huehuehue,
            refs_to_data,
            width,
            height,
            &dane.format,
            kompresja,
            &mut przerób,
            max_plikow,
            &mut percent,
            tx.clone(),
        )
        .unwrap_or(());

        Ok(())
    }
    .await;

    match wynik {
        Ok(_) => {
            let trwanie = start_czas.elapsed();
            let czas_napis = format!("{:.2?}", trwanie);
            let _ = tx
                .send(LogTxDdsPak::Finito(czas_napis))
                .await;
            Ok(())
        }
        Err(e) => {
            let _ = tx
                .send(LogTxDdsPak::Błąd(e.to_string()))
                .await;
            Err(e)
        }
    }
}

// fn is_image_extension(path: &Path) -> bool {
//     let extensions = ["png", "jpg", "jpeg", "tga", "bmp"];
//     path.extension()
//         .and_then(|s| s.to_str())
//         .map(|s| extensions.contains(&s.to_lowercase().as_str()))
//         .unwrap_or(false)
// }
