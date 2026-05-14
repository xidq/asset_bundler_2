use crate::halper::{usun_kanal_alpha, zaszumianie};
use crate::send::wyslij_status;
use enumy::przetwarzanie::PrzetwarzanieWebp;
use enumy::rozszerzenia::bdepth::BdepthWebp;
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use image::imageops::FilterType;
use image::DynamicImage;
use std::fs::create_dir_all;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    let (final_img, nazwa_bd) = match bit_depth {
        BdepthWebp::Rgb8Alpha => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageRgba8(
                        usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_rgba8(),
                    )
                } else {
                    DynamicImage::ImageRgba8(
                        usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_rgba8(),
                    )
                        .resize(
                            wymiar,
                            wymiar,
                            filtr,
                        )
                }
            },
            "_8ba",
        ),
        BdepthWebp::Rgb8 => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageRgb8(
                        usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_rgb8(),
                    )
                } else {
                    DynamicImage::ImageRgb8(
                        usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_rgb8(),
                    )
                        .resize(
                            wymiar,
                            wymiar,
                            filtr,
                        )
                }
            },
            "_8b",
        ),
        // _ => {
        //     // placeholder
        //     (DynamicImage::ImageRgb8(bufor.to_rgb8()), "_nimainnych")
        // }
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

    // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
    // 5. Budowanie nazwy pliku: nazwa + wariant + kolor + rozszerzenie
    let finalna_nazwa = format!("{}{}{}.webp", dane.nazwa, nazwa_wariantu, nazwa_bd);
    let mut ścieżka_pliku = dane.sciezka_wyjsciowa.to_path_buf();
    // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
    if !ścieżka_pliku.exists() {
        create_dir_all(ścieżka_pliku.clone())?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    let encoder = webp::Encoder::from_image(&final_finalv3_temp_final_ostatecznyv5)
        .map_err(tokio::io::Error::other)?;

    // 2. Kodujesz z wybraną jakością (lossy) -> zwraca WebPMemory
    // *strata to Twoja wartość u8 (0-100)
    let webp_data = match dane.lossy {
        None => encoder.encode_lossless(),
        Some(xx) => encoder.encode(xx as f32)
    };

    // 3. Zapisujesz gotowe bajty do pliku (zastępuje File::create i encode_image)
    std::fs::write(&ścieżka_pliku, &*webp_data)?;


    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;



Ok(())
}