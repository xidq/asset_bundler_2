use dds::{ColorFormat, DataLayout, Decoder, ImageViewMut};
use encodery::zapisywanie::generic::zapisywanie_generic;
use enumy::dane_do_przetwarzania::DaneDdsUnpak;
use enumy::opcje::OptInterpolacja;
use enumy::przetwarzanie::{PrzetwarzanieAvif, PrzetwarzanieFf, PrzetwarzanieJpg, PrzetwarzaniePng, PrzetwarzanieQoi, PrzetwarzanieTga, PrzetwarzanieWebp};
use enumy::rozszerzenia::ext::ImgExt;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::statusy::LogTxDdsUnpak;
use futures::channel::mpsc::Sender;
use futures::executor::block_on;
use futures::SinkExt;
use image::DynamicImage;
use std::fs::File;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use enumy::rozszerzenia::kolor::ColorProfilePhoto;

pub async fn dds_to_image(
    dane: DaneDdsUnpak,
    mut tx: Sender<LogTxDdsUnpak>,
) -> Result<(), std::io::Error> {
    let start_czas = Instant::now();




    let obecna_operacja: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));



    let file = File::open(&dane.ścieżka_wejściowa)?;
    let mut decoder =
        Decoder::new(file).map_err(std::io::Error::other)?;

    let layout = decoder.layout();
    let size = layout.main_size(); // Rozmiar pojedynczej tekstury (level 0)

    // Rozpakowujemy informacje o tablicy tekstur z enuma DataLayout
    let (array_len, mip_count) = match layout {
        DataLayout::TextureArray(ref array) => {
            // W Twoim przypadku (TextureArray) używamy .len() dla ilości tekstur
            // i .get(0).iter_mips().count() dla ilości mipmap
            (
                array.len(),
                array.get(0).map(|t| t.iter_mips().count()).unwrap_or(1),
            )
        }
        DataLayout::Texture(ref tex) => (1, tex.iter_mips().count()),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Nieobsługiwany układ danych (Volume nie jest wspierany",
            ));
        }
    };

    let metryka_operacji = Some(array_len as u32 * 3);
    let _ = tx.try_send(LogTxDdsUnpak::Start);
    let wynik: Result<(), std::io::Error> = {

        for i in 0..array_len {
            // Przygotowujemy bufor na RGBA8 (4 bajty na piksel)
            let mut buffer = vec![0_u8; size.pixels() as usize * 4];

            {
                let view = ImageViewMut::new(
                    &mut buffer,
                    size,
                    ColorFormat::RGBA_U8
                ).ok_or_else(|| {
                    std::io::Error::other(
                        "Błąd ImageViewMut: Bufor ma nieprawidłowy rozmiar dla podanego formatu/wymiarów"
                    )
                })?;

                // Czytamy główną powierzchnię (level 0)
                decoder
                    .read_surface(view)
                    .map_err(|e| std::io::Error::other(format!("{:?}", e)))?;
            }

            // Jeśli są mipmapy, musimy je pominąć, aby kursor przeszedł do następnej tekstury w tablicy
            if mip_count > 1 {
                decoder
                    .skip_mipmaps()
                    .map_err(|e| std::io::Error::other(format!("{:?}", e)))?;
            }

            block_on(async {
                if let Some(img_buffer) = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(
                    size.width,
                    size.height,
                    buffer,
                ) {
                    let file_stem = dane.ścieżka_wejściowa
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("export");
                    let nazawawawa = format!("{}_{}", file_stem, i);

                    let dynamic_img = DynamicImage::ImageRgba8(img_buffer);


                    match dane.rozszerzenie.clone() {
                        ImgExt::Jpg { jakosc, progresywny, bit_depth, sampling, quant, scans, } => {
                            let dane = PrzetwarzanieJpg {
                                bufor: dynamic_img,
                                rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                                sciezka_wyjsciowa: dane.ścieżka_wyjściowa.clone(),
                                nazwa: nazawawawa.clone(),
                                interpolacja: OptInterpolacja::Lanczos3,
                                jakosc,
                                progresywny,
                                bdepth: bit_depth.clone(),
                                sampling,
                                quant,
                                skany: scans,
                                alpha: (0, 0, 0),
                                zaszumienie: None,
                                exif: None,
                                kolor: ColorProfilePhoto::None,
                            };
                            zapisywanie_generic(
                                dane,
                                metryka_operacji,
                                obecna_operacja.clone(),
                                tx.clone(),
                            ).await
                        }
                        ImgExt::Ff { metoda_kompresji } => {
                            let dane = PrzetwarzanieFf {
                                bufor: dynamic_img,
                                rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                                sciezka_wyjsciowa: dane.ścieżka_wyjściowa.clone(),
                                nazwa: nazawawawa.clone(),
                                interpolacja: OptInterpolacja::Lanczos3,
                                alpha: (0, 0, 0),
                                zaszumienie: None,
                                kompresja: vec![metoda_kompresji],
                            };
                            zapisywanie_generic(
                                dane,
                                metryka_operacji,
                                obecna_operacja.clone(),
                                tx.clone(),
                            ).await
                        }

                        ImgExt::Png { kompresja, bit_depth } => {
                            let dane = PrzetwarzaniePng {
                                bufor: dynamic_img,
                                rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                                sciezka_wyjsciowa: dane.ścieżka_wyjściowa.clone(),
                                nazwa: nazawawawa.clone(),
                                interpolacja: OptInterpolacja::Lanczos3,
                                bdepth: bit_depth.clone(),
                                alpha: (0, 0, 0),
                                zaszumienie: None,
                                kompresja,
                            };
                            zapisywanie_generic(
                                dane,
                                metryka_operacji,
                                obecna_operacja.clone(),
                                tx.clone(),
                            ).await
                        }
                        ImgExt::Webp { jakosc, lossless, bit_depth } => {
                            let dane = PrzetwarzanieWebp {
                                bufor: dynamic_img,
                                rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                                sciezka_wyjsciowa: dane.ścieżka_wyjściowa.clone(),
                                nazwa: nazawawawa.clone(),
                                interpolacja: OptInterpolacja::Lanczos3,
                                bdepth: bit_depth.clone(),
                                alpha: (0, 0, 0),
                                zaszumienie: None,
                                lossy: if lossless { None } else { Some(jakosc) },
                            };
                            zapisywanie_generic(
                                dane,
                                metryka_operacji,
                                obecna_operacja.clone(),
                                tx.clone(),
                            ).await
                        }
                        ImgExt::Tga { bit_depth } => {
                            let dane = PrzetwarzanieTga {
                                bufor: dynamic_img,
                                rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                                sciezka_wyjsciowa: dane.ścieżka_wyjściowa.clone(),
                                nazwa: nazawawawa.clone(),
                                interpolacja: OptInterpolacja::Lanczos3,
                                bdepth: bit_depth.clone(),
                                alpha: (0, 0, 0),
                                zaszumienie: None,
                            };
                            zapisywanie_generic(
                                dane,
                                metryka_operacji,
                                obecna_operacja.clone(),
                                tx.clone(),
                            ).await
                        }
                        ImgExt::Qoi { bit_depth } => {
                            let dane = PrzetwarzanieQoi {
                                bufor: dynamic_img,
                                rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                                sciezka_wyjsciowa: dane.ścieżka_wyjściowa.clone(),
                                nazwa: nazawawawa.clone(),
                                interpolacja: OptInterpolacja::Lanczos3,
                                bdepth: bit_depth.clone(),
                                alpha: (0, 0, 0),
                                zaszumienie: None,
                            };
                            zapisywanie_generic(
                                dane,
                                metryka_operacji,
                                obecna_operacja.clone(),
                                tx.clone(),
                            ).await
                        }
                        ImgExt::Avif { chroma, speed, metoda_kompresji, lossy, bit_depth } => {
                            let dane = PrzetwarzanieAvif {
                                bufor: dynamic_img,
                                rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                                sciezka_wyjsciowa: dane.ścieżka_wyjściowa.clone(),
                                nazwa: nazawawawa.clone(),
                                interpolacja: OptInterpolacja::Lanczos3,
                                bdepth: bit_depth.clone(),
                                alpha: (0, 0, 0),
                                zaszumienie: None,
                                chroma,
                                speed,
                                metoda_kompresji,
                                lossy,
                                exif: None,
                                kolor: ColorProfilePhoto::None,
                            };
                            zapisywanie_generic(
                                dane,
                                metryka_operacji,
                                obecna_operacja.clone(),
                                tx.clone(),
                            ).await
                        }
                    }
                    // let file_stem = dane.ścieżka_wejściowa.file_stem().and_then(|s| s.to_str()).unwrap_or("export");
                    // let out_path = dane.ścieżka_wyjściowa.join(format!("{}_{}.jpg", file_stem, i));

                    // dynamic_img.save_with_format(out_path, ImageFormat::Jpeg).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                } else {
                    Err(std::io::Error::other("Błąd tworzenia ImageBuffer"))
                }

                // Raportowanie postępu

            })?
        }
        Ok(())
    };


    match wynik {
        Ok(_) => {
            let trwanie = start_czas.elapsed(); 
            let czas_napis = format!("{:.2?}", trwanie);
            let _ = tx
                .send(LogTxDdsUnpak::Finito(czas_napis))
                .await;
            Ok(())
        }
        Err(e) => {
            // Jeśli cokolwiek powyżej sypnie błędem (przez znak zapytania),
            // wysyłamy opis błędu do UI zamiast po prostu "padać".
            let _ = tx
                .send(LogTxDdsUnpak::Błąd(e.to_string()))
                .await;
            Err(e)
        }
    }

}
