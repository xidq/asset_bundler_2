use std::fs::{create_dir_all, File};
use std::sync::Arc;
use futures::channel::mpsc::Sender;
use image::ImageEncoder;
use image::imageops::FilterType;
use tokio::sync::Mutex;
use enumy::przetwarzanie::{PrzetwarzanieQoi, PrzetwarzanieWebp};
use enumy::rozszerzenia::bdepth::{BdepthQoi, BdepthWebp};
use enumy::statusy::Logi;
use crate::halper::{usun_kanal_alpha, zaszumianie};
use crate::send::wyslij_status;

pub async fn qoi_match<T>(
    dane: PrzetwarzanieQoi,
    wymiar: u32,
    bit_depth: BdepthQoi,
    nazwa_wariantu: String,
    filtr: FilterType,
    metryka_operacji: Option<u32>,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<T>,
) -> Result<(), tokio::io::Error>
where T: Logi,
{
    let (final_img, nazwa_bd, color_type, szer, wys) = match bit_depth {
        BdepthQoi::Color24 => {
            // 1. Usuwamy alfę i przygotowujemy RGB8
            dbg!("[debug] qoi tc24");
            let img = usun_kanal_alpha(dane.bufor.clone(), dane.alpha);
            let res = if wymiar == 0 {
                match dane.zaszumienie {
                    Some(x) => zaszumianie(x, img),
                    None => img,
                }
                    .to_rgb8()
            } else {
                match dane.zaszumienie {
                    Some(x) => {
                        zaszumianie(x, img.resize(wymiar, wymiar, filtr))
                    }
                    None => img.resize(wymiar, wymiar, filtr),
                }
                    .to_rgb8()
            };

            let (width, height) = res.dimensions();
            (
                res.into_raw(),
                "_tc24",
                image::ExtendedColorType::Rgb8,
                width,
                height,
            )
        }

        BdepthQoi::Color32 => {
            dbg!("[debug] qoi tc32");

            let res = if wymiar == 0 {
                match dane.zaszumienie {
                    Some(x) => zaszumianie(x, dane.bufor.clone()),
                    None => dane.bufor.clone(),
                }
                    .to_rgba8()
            } else {
                match dane.zaszumienie {
                    Some(x) => zaszumianie(
                        x,
                        dane.bufor
                            .clone()
                            .resize(wymiar, wymiar, filtr),
                    ),
                    None => dane.bufor
                        .clone()
                        .resize(wymiar, wymiar, filtr),
                }
                    .to_rgba8()
            };
            let (width, height) = res.dimensions();
            (
                res.into_raw(),
                "_tc32",
                image::ExtendedColorType::Rgba8,
                width,
                height,
            )
        }
    };


    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;

    // 4. Budowanie nazwy
    let finalna_nazwa = format!("{}{}{}.qoi", dane.nazwa, nazwa_wariantu, nazwa_bd);
    let mut ścieżka_pliku = dane.sciezka_wyjsciowa.to_path_buf();

    if !ścieżka_pliku.exists() {
        create_dir_all(&ścieżka_pliku)?;
    }
    ścieżka_pliku.push(finalna_nazwa);
    dbg!("ścieżka pliku: {}", &ścieżka_pliku);

    // 5. Zapis z wykorzystaniem enkodera i naszych surowych danych
    let f = File::create(&ścieżka_pliku)?;

    let encoder = image::codecs::qoi::QoiEncoder::new(f);

    encoder
        .write_image(&final_img, szer, wys, color_type)
        .map_err(std::io::Error::other)?;


    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;


    Ok(())
}