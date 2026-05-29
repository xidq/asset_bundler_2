use crate::halper::{ogarnij_icc, usun_kanal_alpha, zaszumianie};
use crate::send::wyslij_status;
use enumy::przetwarzanie::PrzetwarzanieWebp;
use enumy::rozszerzenia::bdepth::BdepthWebp;
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use image::imageops::FilterType;
use lcms2::PixelFormat;
use std::fs::create_dir_all;
use std::sync::Arc;
use image::GenericImageView;
use tokio::sync::Mutex;
use img_parts::webp::WebP;
use img_parts::ImageEXIF;
use bytes::Bytes;

pub async fn webp_match<T>(
    dane: PrzetwarzanieWebp,
    wymiar: u32,
    bit_depth: BdepthWebp,
    nazwa_wariantu: String,
    filtr: FilterType,
    metryka_operacji: Option<u32>,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<T>,
) -> Result<(), tokio::io::Error>
where T: Logi,
{

    let foto =
        if wymiar == 0 {
            dane.bufor
        } else {
            dane.bufor
                .resize(
                    wymiar,
                    wymiar,
                    filtr,
                )
        };
    let (width, height) = foto.dimensions();

    let (final_img, nazwa_bd, icc) = match bit_depth {
        BdepthWebp::Rgb8Alpha => ( foto, "_8ba", PixelFormat::RGBA_8),
        BdepthWebp::Rgb8 => (usun_kanal_alpha(foto.clone(), dane.alpha), "_8b", PixelFormat::RGB_8),

    };


    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;

    let final_finalv3_temp_final_ostatecznyv5 = match dane.zaszumienie {
        Some(xoxo) => zaszumianie(xoxo, final_img),
        None => final_img,
    };

    // 1. KRYTYCZNE: Przepuszczamy obraz przez LCMS2, tak jak w PNG i AVIF!
    let plikkk = ogarnij_icc(
        dane.kolor.clone(),
        &final_finalv3_temp_final_ostatecznyv5,
        icc
    );

    // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
    // 5. Budowanie nazwy pliku: nazwa + wariant + kolor + rozszerzenie
    let finalna_nazwa = format!("{}{}{}.webp", dane.nazwa, nazwa_wariantu, nazwa_bd);
    let mut ścieżka_pliku = dane.sciezka_wyjsciowa.to_path_buf();
    // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
    if !ścieżka_pliku.exists() {
        create_dir_all(ścieżka_pliku.clone())?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    // let encoder = webp::Encoder::from_image(&final_finalv3_temp_final_ostatecznyv5)
    //     .map_err(tokio::io::Error::other)?;
    //
    // // 2. Kodujesz z wybraną jakością (lossy) -> zwraca WebPMemory
    // // *strata to Twoja wartość u8 (0-100)
    // let webp_data = match dane.lossy {
    //     None => encoder.encode_lossless(),
    //     Some(xx) => encoder.encode(xx as f32)
    // };
    //
    // // 3. Zapisujesz gotowe bajty do pliku (zastępuje File::create i encode_image)
    // std::fs::write(&ścieżka_pliku, &*webp_data)?;

    let encoder = match icc {
        PixelFormat::RGBA_8 => webp::Encoder::from_rgba(&plikkk, width, height),
        PixelFormat::RGB_8 => webp::Encoder::from_rgb(&plikkk, width, height),
        _ => return Err(tokio::io::Error::other("WebP obsługuje natywnie tylko 8-bitowe formaty RGB/RGBA")),
    };


    let webp_data = match dane.lossy {
        None => encoder.encode_lossless(),
        Some(xx) => encoder.encode(xx as f32)
    };

    let mut webp_image = WebP::from_bytes(Bytes::copy_from_slice(&webp_data))
        .map_err(|_| tokio::io::Error::other("img-parts: Nie udało się sparsować strumienia WebP"))?;

    // img-parts automatycznie przekonwertuje nagłówek na VP8X, jeśli dodasz EXIF
    if let Some(ref exif_bytes) = dane.exif &&
        !exif_bytes.is_empty() {
            webp_image.set_exif(Some(Bytes::copy_from_slice(exif_bytes)));

    }

    // Opcjonalnie: jak z jakiegoś powodu zachowanie tego zacnego sRGB ICC
    // było by potrzebne chooć to sht, to jest metoda:
    // webp_image.set_icc_profile(Some(Bytes::copy_from_slice(icc_bytes)));

    let mut ostateczne_bajty = Vec::new();
    webp_image.encoder().write_to(&mut ostateczne_bajty)
        .map_err(|_| tokio::io::Error::other("Błąd podczas składania pliku WebP z metadanymi"))?;

    std::fs::write(&ścieżka_pliku, ostateczne_bajty)?;


    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;



    Ok(())
}