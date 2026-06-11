use crate::halper::{usun_kanal_alpha, zaszumianie};
use crate::send::wyslij_status;
use crate::zapisywanie::generic::{get_higher_tier_copy, InneDane};
use enumy::opcje::OptIstniejePlik;
use enumy::przetwarzanie::PrzetwarzanieTga;
use enumy::rozszerzenia::bdepth::BdepthTga;
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use std::fs::{create_dir_all, File};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn tga_match<T>(
    dane: PrzetwarzanieTga,
    dane2: InneDane<BdepthTga>,
    // wymiar: u32,
    // bit_depth: BdepthTga,
    // nazwa_wariantu: String,
    // filtr: FilterType,
    metryka_operacji: Option<u32>,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<T>,
) -> Result<(), tokio::io::Error>
where T: Logi,
{
    
    // let obrazek_wymiarowany
    let (final_img, nazwa_bd, color_type, szer, wys) = match dane2.bdepth {
        BdepthTga::Luma8 => {
            let img = usun_kanal_alpha(dane.bufor.clone(), dane.alpha);

            let res = if dane2.wymiar == 0 {
                match dane.zaszumienie {
                    Some(x) => zaszumianie(x, img),
                    None => img,
                }
                    .to_luma8()
            } else {
                match dane.zaszumienie {
                    Some(x) => {
                        zaszumianie(x, img.resize(dane2.wymiar, dane2.wymiar, dane2.filtr))
                    }
                    None => img.resize(dane2.wymiar, dane2.wymiar, dane2.filtr),
                }
                    .to_luma8()
                // img.resize(wymiar, wymiar, filtr).to_luma8()
            };
            let (width, height) = res.dimensions();
            (
                res.into_raw(),
                "_g",
                image::ExtendedColorType::L8,
                width,
                height,
            )
        }

        BdepthTga::HighColor16 => {
            // Skalujemy bufor
            let xxx = if dane2.wymiar == 0 {
                match dane.zaszumienie {
                    Some(x) => zaszumianie(x,dane.bufor.clone()),
                    None =>dane.bufor.clone(),
                }
                    .to_rgba8()
            } else {
                match dane.zaszumienie {
                    Some(x) => zaszumianie(
                        x,
                        dane.bufor
                            .clone()
                            .resize(dane2.wymiar, dane2.wymiar, dane2.filtr),
                    ),
                    None => dane.bufor
                        .clone()
                        .resize(dane2.wymiar, dane2.wymiar, dane2.filtr),
                }
                    .to_rgba8()
            };
            let (width, height) = xxx.dimensions();

            let mut raw = Vec::new();
            for pixel in xxx.pixels() {
                // Zamiast pakować bity do u16 (którego encoder nie przyjmie),
                // robimy "udawane" 1-bitowe alpha w formacie 32-bitowym:
                raw.push(pixel[0]); // B
                raw.push(pixel[1]); // G
                raw.push(pixel[2]); // R
                raw.push(if pixel[3] > 128 { 255 } else { 0 }); // Alpha (tylko 0 lub 255)
            }
            (raw, "_hc16", image::ExtendedColorType::Rgba8, width, height)
        }
        BdepthTga::TrueColor24 => {
            // 1. Usuwamy alfę i przygotowujemy RGB8
            let img = usun_kanal_alpha(dane.bufor.clone(), dane.alpha);
            let res = if dane2.wymiar == 0 {
                match dane.zaszumienie {
                    Some(x) => zaszumianie(x, img),
                    None => img,
                }
                    .to_rgb8()
            } else {
                match dane.zaszumienie {
                    Some(x) => {
                        zaszumianie(x, img.resize(dane2.wymiar, dane2.wymiar, dane2.filtr))
                    }
                    None => img.resize(dane2.wymiar, dane2.wymiar, dane2.filtr),
                }
                    .to_rgb8()
            };

            // 2. TGA chce BGR, więc mapujemy piksele: [R, G, B] -> B, G, R
            // let mut raw = Vec::with_capacity((res.width() * res.height() * 3) as usize);
            // for p in res.pixels() {
            //     raw.push(p[2]); // Blue
            //     raw.push(p[1]); // Green
            //     raw.push(p[0]); // Red
            // }
            let (width, height) = res.dimensions();
            (
                res.into_raw(),
                "_tc24",
                image::ExtendedColorType::Rgb8,
                width,
                height,
            )
        }

        BdepthTga::TrueColorA32 => {
            dbg!("[debug] tga tc32");

            let res = if dane2.wymiar == 0 {
                match dane.zaszumienie {
                    Some(x) => zaszumianie(x,dane.bufor.clone()),
                    None =>dane.bufor.clone(),
                }
                    .to_rgba8()
            } else {
                match dane.zaszumienie {
                    Some(x) => zaszumianie(
                        x,
                        dane.bufor
                            .clone()
                            .resize(dane2.wymiar, dane2.wymiar, dane2.filtr),
                    ),
                    None => dane.bufor
                        .clone()
                        .resize(dane2.wymiar, dane2.wymiar, dane2.filtr),
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
    
    // let final_final_final_v3_xD = match dane.zaszumienie{
    //     Some(x) => zaszumianie(x,final_img),
    //     None => final_img,
    // };

    // 4. Budowanie nazwy
    let finalna_nazwa = format!("{}{}{}.tga", dane.nazwa, dane2.nazwa_wariantu, nazwa_bd);
    let mut ścieżka_pliku = dane.sciezka_wyjsciowa.to_path_buf();

    if !ścieżka_pliku.exists() {
        create_dir_all(&ścieżka_pliku)?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    if ścieżka_pliku.exists() {
        match dane2.zastepowanie{
            OptIstniejePlik::Zamień => {}
            OptIstniejePlik::Zostaw => {
                let mut oopr = obecna_operacja.lock().await;
                *oopr += 1;
                let obecnie = *oopr;
                drop(oopr);

                wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;
                return Ok(())
            }
            OptIstniejePlik::ZmieńNazwę => {ścieżka_pliku = get_higher_tier_copy(ścieżka_pliku)}
        }
    };

    // 5. Zapis z wykorzystaniem enkodera i naszych surowych danych
    let f = File::create(&ścieżka_pliku)?;

    // Ustalamy wymiary dla encodera
    // let (szer, wys) = if wymiar == 0 {
    //     (bufor.width(), bufor.height())
    // } else {
    //     (wymiar, wymiar)
    // };

    let encoder = image::codecs::tga::TgaEncoder::new(f);

    // Używamy .encode(), bo final_data to Vec<u8> (surowe bajty),
    // a color_type to ten wyciągnięty z match (np. Bgra8 lub Bgr8)
    encoder
        .encode(&final_img, szer, wys, color_type)
        .map_err(std::io::Error::other)?;


    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;


Ok(())
}