use crate::metody_mielenia::laczenie::laczenie_vac_to_dyn;
use encodery::wczytaj_foto::wczytaj_zdjęcie;
use encodery::zapisywanie::generic::zapisywanie_generic;
use enumy::dane_do_przetwarzania::DaneMerge;
use enumy::opcje::OptInterpolacja;
use enumy::przetwarzanie::{PrzetwarzanieAvif, PrzetwarzanieFf, PrzetwarzanieJpg, PrzetwarzaniePng, PrzetwarzanieQoi, PrzetwarzanieTga, PrzetwarzanieWebp};
use enumy::rozszerzenia::bdepth::BdepthQoi;
use enumy::rozszerzenia::ext::ImgExtSingle;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
pub use enumy::statusy::LogTxMerge;
use futures::channel::mpsc;
use futures::executor::block_on;
use futures::SinkExt;
use image::DynamicImage;
use std::sync::Arc;
use tokio::sync::Mutex;



// fn that is entry point for merging images by channels

pub async fn fn_do_laczenia_fot(
    dane: DaneMerge,
    mut tx: mpsc::Sender<LogTxMerge>,
) -> Result<(), tokio::io::Error> {
    
    let _ = tx.send(LogTxMerge::Start).await;


    let metryka_operacji = Some(3);
    let obecna_operacja: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));


    // let start_czas = std::time::Instant::now();

    let sciezki = [
        dane.sciezka_r,
        dane.sciezka_g,
        dane.sciezka_b,
        dane.sciezka_a,
    ];
    let mut surowe_obrazy = [None, None, None, None];
    let mut max_x = 0u32;
    let mut max_y = 0u32;

    // Here, we're searching for dimensions, ofc images are optional soo...
    // taking max dimention from x and y axis and store that in let above
    for (i, opt_p) in sciezki.iter().enumerate() {
        if let Some(p) = opt_p {
            let (img, _) = wczytaj_zdjęcie(p.clone())?;
            if img.width() > max_x {
                max_x = img.width();
            }
            if img.height() > max_y {
                max_y = img.height();
            }
            surowe_obrazy[i] = Some(img);
        }
    }

    // When there's no image, so there's a error message ;)
    // that's second check for images, first is b4 starting this fn in ui

    if max_x == 0 || max_y == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Brak jakichkolwiek zdjęć",
        ));
    }

    // Channel preparation, if there's no image so we'r making custom ones just to merge.
    let przygotuj_final = |opt_img: Option<DynamicImage>| -> DynamicImage {
        match opt_img {
            Some(img) => {

                if img.width() == max_x && img.height() == max_y {
                    return img;
                }


                let przeskalowany = img.resize(max_x, max_y, image::imageops::FilterType::Lanczos3);

                // 3. Przygotowanie tła
                let mut tlo = DynamicImage::ImageLuma8(image::ImageBuffer::new(max_x, max_y));


                let x_pos = (max_x - przeskalowany.width()) / 2;
                let y_pos = (max_y - przeskalowany.height()) / 2;

                // 5. Nakładanie
                image::imageops::overlay(&mut tlo, &przeskalowany, x_pos as i64, y_pos as i64);
                tlo
            }
            None => DynamicImage::ImageLuma8(image::ImageBuffer::new(max_x, max_y)),
        }
    };

    let img_r = przygotuj_final(surowe_obrazy[0].take());
    let img_g = przygotuj_final(surowe_obrazy[1].take());
    let img_b = przygotuj_final(surowe_obrazy[2].take());
    let img_a = przygotuj_final(surowe_obrazy[3].take());

    // let mut ścieżka_wyjściowa = dane.sciezka_out.clone();
    // ścieżka_wyjściowa.push(dane.nazwa);

    // let ilosc_bitow = if let rozszerzenia_plików_zdjęciowych::Png { bit_depth, .. } = &dane.out_format {
    //     // Pobieramy pierwszy element z Vec.
    //     // .cloned() jest potrzebne, jeśli Obraz nie implementuje Copy
    //     bit_depth.first().cloned().unwrap_or(Obraz::B8)
    // } else {
    //     // Wartość domyślna, jeśli out_format to nie Png
    //     Obraz::B8
    // };
    let wymiar = (max_x, max_y);
    let obrazki = Vec::from([img_r, img_g, img_b, img_a]);
    // let bfor = laczenie_vac_to_dyn(obrazki,bit_depth,wymiar);

    let wynik: Result<(), tokio::io::Error> = block_on(async {
        match dane.rozszerzenie {
            ImgExtSingle::Png {
                bit_depth,
                kompresja,
            } => {
                let dane = PrzetwarzaniePng {
                    bufor: laczenie_vac_to_dyn(obrazki, bit_depth, wymiar).await.ok().unwrap(),
                    rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                    sciezka_wyjsciowa: dane.sciezka_out,
                    nazwa: dane.nazwa.clone(),
                    interpolacja: OptInterpolacja::Lanczos3,
                    kompresja: kompresja.clone(),
                    bdepth: vec![bit_depth.clone()],
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
            ImgExtSingle::Jpg {
                jakosc,
                progresywny,
                bit_depth, sampling: _, quant: _, scans: _,
            } => {
                let dane = PrzetwarzanieJpg {
                    bufor: laczenie_vac_to_dyn(obrazki, bit_depth, wymiar).await.ok().unwrap(),
                    rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                    sciezka_wyjsciowa: dane.sciezka_out,
                    nazwa: dane.nazwa.clone(),
                    interpolacja: OptInterpolacja::Lanczos3,
                    jakosc,
                    bdepth: vec![bit_depth.clone()],
                    sampling: Default::default(),
                    quant: Default::default(),
                    skany: 4,
                    alpha: (0, 0, 0),
                    zaszumienie: None,
                    progresywny
                };
                zapisywanie_generic(
                    dane,
                    metryka_operacji,
                    obecna_operacja.clone(),
                    tx.clone(),
                ).await
            }
            ImgExtSingle::Webp {
                jakosc,
                lossless,
                bit_depth,
            } => {
                let dane = PrzetwarzanieWebp {
                    bufor: laczenie_vac_to_dyn(obrazki, bit_depth, wymiar).await.ok().unwrap(),
                    rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                    sciezka_wyjsciowa: dane.sciezka_out,
                    nazwa: dane.nazwa.clone(),
                    interpolacja: OptInterpolacja::Lanczos3,
                    bdepth: vec![bit_depth.clone()],
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

            ImgExtSingle::Tga { bit_depth } => {
                let dane = PrzetwarzanieTga {
                    bufor: laczenie_vac_to_dyn(obrazki, bit_depth, wymiar).await.ok().unwrap(),
                    rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                    sciezka_wyjsciowa: dane.sciezka_out,
                    nazwa: dane.nazwa.clone(),
                    interpolacja: OptInterpolacja::Lanczos3,
                    bdepth: vec![bit_depth.clone()],
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
            ImgExtSingle::Ff { metoda_kompresji } => {
                let dane = PrzetwarzanieFf {
                    bufor: laczenie_vac_to_dyn(obrazki, BdepthQoi::Color32, wymiar).await.ok().unwrap(),
                    rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                    sciezka_wyjsciowa: dane.sciezka_out,
                    nazwa: dane.nazwa.clone(),
                    interpolacja: OptInterpolacja::Lanczos3,
                    kompresja: vec![metoda_kompresji],
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
            ImgExtSingle::Qoi { bit_depth } => {
                let dane = PrzetwarzanieQoi {
                    bufor: laczenie_vac_to_dyn(obrazki, bit_depth, wymiar).await.ok().unwrap(),
                    rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                    sciezka_wyjsciowa: dane.sciezka_out,
                    nazwa: dane.nazwa.clone(),
                    interpolacja: OptInterpolacja::Lanczos3,
                    bdepth: vec![bit_depth.clone()],
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

            ImgExtSingle::Avif {
                chroma,
                speed,
                metoda_kompresji,
                lossy,
                bit_depth
            } => {
                let dane = PrzetwarzanieAvif {
                    bufor: laczenie_vac_to_dyn(obrazki, bit_depth, wymiar).await.ok().unwrap(),
                    rozdzielczosci: vec![Rozdzielczości::Oryginalna],
                    sciezka_wyjsciowa: dane.sciezka_out,
                    nazwa: dane.nazwa.clone(),
                    interpolacja: OptInterpolacja::Lanczos3,
                    bdepth: vec![bit_depth.clone()],
                    alpha: (0, 0, 0),
                    zaszumienie: None,
                    chroma,
                    speed,
                    metoda_kompresji,
                    lossy,
                };
                zapisywanie_generic(
                    dane,
                    metryka_operacji,
                    obecna_operacja.clone(),
                    tx.clone(),
                ).await
            }
        }
    });

    match wynik {
        Ok(_) => {
            // let czas_napis = format!("{:.2?}", start_czas.elapsed());
            let _ = tx.send(LogTxMerge::Finito(String::new())).await;
            Ok(())
        }
        Err(e) => {
            let _ = tx.send(LogTxMerge::Błąd(e.to_string())).await;
            Err(e)
        }
    }
}
