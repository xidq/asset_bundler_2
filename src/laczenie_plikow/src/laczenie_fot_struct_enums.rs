use crate::metody_mielenia::laczenie_ff::laczenie_ff;
use crate::metody_mielenia::laczenie_jpg::laczenie_jpg;
use crate::metody_mielenia::laczenie_png::laczenie_png;
use crate::metody_mielenia::laczenie_qoi::laczenie_qoi;
use crate::metody_mielenia::laczenie_tga::laczenie_tga;
use crate::metody_mielenia::laczenie_webp::laczenie_webp;
use enumy::dane_do_przetwarzania::DaneDoŁączeniaZdjęć;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuAvif, OptFormatyKoloruObrazuQoi, OptRozszerzeniaPlikówZdjęciowychPojedyncze};
use futures::SinkExt;
use futures::channel::mpsc;
use image::DynamicImage;
use encodery::wczytaj_foto::wczytaj_zdjęcie;
pub use enumy::statusy::LogTxDoŁączeniaZdjęć;
use crate::metody_mielenia::laczenie::{laczenie_vac_to_dyn, ogarnij_sciezki_w_koncu};
use crate::metody_mielenia::laczenie_avif::laczenie_avif;

pub async fn fn_do_laczenia_fot(
    dane: DaneDoŁączeniaZdjęć,
    mut tx: mpsc::Sender<LogTxDoŁączeniaZdjęć>,
) -> Result<(), tokio::io::Error> {
    
    let kolor_alfa=(0_u16,0_u16,0_u16);
    let _ = tx.send(LogTxDoŁączeniaZdjęć::Start).await;

    let start_czas = std::time::Instant::now();

    let sciezki = [
        dane.sciezka_r,
        dane.sciezka_g,
        dane.sciezka_b,
        dane.sciezka_a,
    ];
    let mut surowe_obrazy = [None, None, None, None];
    let mut max_x = 0u32;
    let mut max_y = 0u32;

    // --- ETAP 1: SZUKAMY MAKSYMALNYCH WYMIARÓW ---
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

    if max_x == 0 || max_y == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Brak jakichkolwiek zdjęć",
        ));
    }

    // --- ETAP 2: PRZYGOTOWANIE KANAŁÓW ---
    let mut przygotuj_final = |opt_img: Option<DynamicImage>| -> DynamicImage {
        match opt_img {
            Some(img) => {
                // if img.width() == max_x || img.height() == max_y {
                //     if img.width() == max_x && img.height() == max_y {
                //         return img;
                //     } else {
                //         img.resize(max_x,max_y,image::imageops::FilterType::Lanczos3);
                //     }
                //     let mut tlo = DynamicImage::ImageLuma8(image::ImageBuffer::new(max_x, max_y));
                //     image::imageops::overlay(&mut tlo, &img, ((max_x - img.width()) / 2) as i64, ((max_y - img.height()) / 2) as i64);
                //     tlo
                // } else {
                //     img
                // }

                // 1. Jeśli wymiary są idealne, po prostu zwróć
                if img.width() == max_x && img.height() == max_y {
                    return img;
                }

                // 2. Skalowanie - MUSISZ przypisać wynik do nowej zmiennej
                // Używamy .resize, aby zachować proporcje.
                let przeskalowany = img.resize(max_x, max_y, image::imageops::FilterType::Lanczos3);

                // 3. Przygotowanie tła
                let mut tlo = DynamicImage::ImageLuma8(image::ImageBuffer::new(max_x, max_y));

                // 4. Obliczanie pozycji (centrowanie)
                // Teraz używamy wymiarów 'przeskalowany', które na pewno są <= max_x/y
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

    let wynik: Result<(), tokio::io::Error> = match dane.out_format {
        OptRozszerzeniaPlikówZdjęciowychPojedyncze::Png {
            bit_depth,
            kompresja,
        } => {
            laczenie_png(
                obrazki,
                &dane.sciezka_out,
                &dane.nazwa,
                &kompresja,
                &(0, 0, 0),
                &bit_depth,
                wymiar,
            )
            .await
        }
        OptRozszerzeniaPlikówZdjęciowychPojedyncze::Jpg {
            jakosc,
            progresywny,
            bit_depth,
        } => {
            laczenie_jpg(
                obrazki,
                &dane.sciezka_out,
                &dane.nazwa,
                &jakosc,
                &progresywny,
                &bit_depth,
                &(0, 0, 0),
                wymiar,
            )
            .await
        }
        OptRozszerzeniaPlikówZdjęciowychPojedyncze::Webp {
            jakosc,
            lossless,
            bit_depth,
        } => {
            laczenie_webp(
                obrazki,
                &dane.sciezka_out,
                &dane.nazwa,
                &jakosc,
                lossless,
                &bit_depth,
                &(0, 0, 0),
                wymiar,
            )
            .await
        }

        OptRozszerzeniaPlikówZdjęciowychPojedyncze::Tga { bit_depth } => {
            laczenie_tga(
                obrazki,
                &dane.sciezka_out,
                &dane.nazwa,
                &(0, 0, 0),
                &bit_depth,
                wymiar,
            )
            .await
        }
        OptRozszerzeniaPlikówZdjęciowychPojedyncze::Ff { metoda_kompresji } => {
            laczenie_ff(
                obrazki,
                &dane.sciezka_out,
                &dane.nazwa,
                &(0, 0, 0),
                &metoda_kompresji,
                wymiar,
            )
            .await
        }
        OptRozszerzeniaPlikówZdjęciowychPojedyncze::Qoi { bit_depth } => {
            laczenie_qoi(
                obrazki,
                &dane.sciezka_out,
                &dane.nazwa,
                &(0, 0, 0),
                &bit_depth,
                wymiar,
            )
            .await
        }

        OptRozszerzeniaPlikówZdjęciowychPojedyncze::Avif { 
            chroma, 
            speed, 
            metoda_kompresji, 
            lossy, 
            bit_depth 
        } => {
            let wrzód  =match bit_depth{
                OptFormatyKoloruObrazuAvif::B8 => {OptFormatyKoloruObrazOgólny::B8}
                OptFormatyKoloruObrazuAvif::B8a => {OptFormatyKoloruObrazOgólny::B8a}
                OptFormatyKoloruObrazuAvif::B10 => {OptFormatyKoloruObrazOgólny::B16}
                OptFormatyKoloruObrazuAvif::B10a => {OptFormatyKoloruObrazOgólny::B16a}
            };
            let obrazeczek = laczenie_vac_to_dyn(obrazki, &wrzód, wymiar).await?;
            let sciezka_vinal_final_chyba_v1 = ogarnij_sciezki_w_koncu(dane.sciezka_out, dane.nazwa).await?;
            laczenie_avif(obrazeczek, &sciezka_vinal_final_chyba_v1 , lossy, bit_depth, None, metoda_kompresji, speed, chroma)
                .await
        }
    };

    match wynik {
        Ok(_) => {
            let czas_napis = format!("{:.2?}", start_czas.elapsed());
            let _ = tx.send(LogTxDoŁączeniaZdjęć::Koniec).await;
            Ok(())
        }
        Err(e) => {
            let _ = tx.send(LogTxDoŁączeniaZdjęć::Błąd(e.to_string())).await;
            Err(e)
        }
    }
}
