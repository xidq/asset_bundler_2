use crate::send::wyslij_status;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::opcje::OptInterpolacja;
use enumy::rozszerzenia::bdepth::{BdepthAvif, BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::ext::ImgExt;
use enumy::rozszerzenia::kompresje::ForFfKompresja;
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;

pub async fn sprawdzacz<T>(
    pumpum: DaneKonw,
    mut tx: Sender<T>
) -> Result<DaneKonw, std::io::Error>
where T: Logi{

    wyslij_status(&mut tx, Some(T::status(("data_chck_start", String::new())))).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let mut dane = pumpum.clone();

    if !pumpum.ścieżka_wejściowa.exists() {
        let msg = format!("❌ {}",pumpum.ścieżka_wejściowa.to_string_lossy());
        wyslij_status(&mut tx, Some(T::status(("data_chck_input_path", msg.to_string())))).await;
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, msg));
    }
    wyslij_status(&mut tx, Some(T::status(("data_chck_input_path", "✓".to_string())))).await;

    if !pumpum.ścieżka_wyjściowa.exists() {
        let msg = format!("❌ {}",pumpum.ścieżka_wyjściowa.to_string_lossy());
        wyslij_status(&mut tx, Some(T::status(("data_chck_output_path", msg.clone())))).await;
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, msg));
    }
    wyslij_status(&mut tx, Some(T::status(("data_chck_output_path", "✓".to_string())))).await;


    for format in &mut dane.rozszerzenia {
        match format {
            ImgExt::Jpg { jakosc, progresywny:_, bit_depth, sampling:_, quant:_, scans } => {

                if *jakosc > 100 { *jakosc = 100; }
                if *jakosc == 0 { *jakosc = 1; }


                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b, BdepthJpg::Rgb8 | BdepthJpg::Luma8));

                if bul || bit_depth.is_empty() {
                    wyslij_status(&mut tx, Some(T::blad(
                        "data_chck_jpg_bdepth_err".to_string()
                    ))).await;

                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla JPG: {:?}", bit_depth)
                    ));
                }

                *scans = (*scans).clamp(2_u8,64_u8);


                wyslij_status(&mut tx, Some(T::status(("data_chck_jpg_bdepth",  "✓".to_string())))).await;
            }

            ImgExt::Png { kompresja, bit_depth } => {

                *kompresja = (*kompresja).clamp(0_u8, 9_u8);


                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b,
                    BdepthPng::Rgb8 |
                    BdepthPng::Luma8 |
                    BdepthPng::Rgb16 |
                    BdepthPng::Luma16 |
                    BdepthPng::Rgb8Alpha |
                    BdepthPng::Luma8Alpha |
                    BdepthPng::Rgb16Alpha |
                    BdepthPng::Luma16Alpha
                ));

                if bul || bit_depth.is_empty() {
                    wyslij_status(&mut tx, Some(T::blad(
                        "data_chck_png_bdepth_err".to_string()
                    ))).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla Png: {:?}", bit_depth)
                    ));
                }
                wyslij_status(&mut tx, Some(T::status(("data_chck_png_bdepth",  "✓".to_string())))).await;
            }

            ImgExt::Webp { jakosc, lossless:_, bit_depth } => {

                if *jakosc > 100 { *jakosc = 100; }
                if *jakosc == 0 { *jakosc = 1; }

                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b, BdepthWebp::Rgb8 | BdepthWebp::Rgb8Alpha));

                //WYWAL ERR JAK COŚ NIE TEGES!!!!!!!!!! YAYA!!!!!!!!!!!!!!!!!!
                if bul || bit_depth.is_empty() {
                    wyslij_status(&mut tx, Some(T::blad(
                        "data_chck_webp_bdepth_err".to_string()
                    ))).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla Webp: {:?}", bit_depth)
                    ));
                }
                wyslij_status(&mut tx, Some(T::status(("data_chck_webp_bdepth",  "✓".to_string())))).await;
            }
            ImgExt::Tga {  bit_depth } => {
                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b,
                    BdepthTga::TrueColorA32 |
                    BdepthTga::TrueColor24 |
                    BdepthTga::HighColor16 |
                    BdepthTga::Luma8
                ));

                //WYWAL ERR JAK COŚ NIE TEGES!!!!!!!!!! YAYA!!!!!!!!!!!!!!!!!!
                if bul || bit_depth.is_empty() {
                    wyslij_status(&mut tx, Some(T::blad(
                        "data_chck_tga_bdepth_err".to_string()
                    ))).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla Tga: {:?}", bit_depth)
                    ));
                }
                wyslij_status(&mut tx, Some(T::status(("data_chck_tga_bdepth",  "✓".to_string())))).await;
            }
            ImgExt::Ff { metoda_kompresji } => {



                let bul = matches!(metoda_kompresji,
                    ForFfKompresja::Bzip2(_) |
                    ForFfKompresja::Zstd(_) |
                    ForFfKompresja::Xz(_) |
                    ForFfKompresja::Brak
                );

                //WYWAL ERR JAK COŚ NIE TEGES!!!!!!!!!! YAYA!!!!!!!!!!!!!!!!!!
                if !bul {
                    wyslij_status(&mut tx, Some(T::blad(
                        "Ff: Wykryto nieobsługiwaną kompresję".to_string()
                    ))).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format kompresji dla Ff: {:?}", metoda_kompresji)
                    ));
                }
                match metoda_kompresji {
                    ForFfKompresja::Zstd(xx) => {*xx = (*xx).clamp(1_u8, 22_u8);}
                    ForFfKompresja::Bzip2(xx) => {*xx = (*xx).clamp(1_u8, 22_u8);}
                    ForFfKompresja::Xz(xx) => {*xx = (*xx).clamp(1_u8, 22_u8);}
                    ForFfKompresja::Brak => {}
                }
                wyslij_status(&mut tx, Some(T::status(("data_chck_ff_bdepth",  "✓".to_string())))).await;
            }

            ImgExt::Qoi { bit_depth } => {
                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b,
                    BdepthQoi::Color24 |
                    BdepthQoi::Color32
                ));
                // if bit_depth.len() > 1 {
                //     bit_depth.truncate(1); // Przykład: QOI może mieć tylko jeden format na raz
                // }
                //WYWAL ERR JAK COŚ NIE TEGES!!!!!!!!!! YAYA!!!!!!!!!!!!!!!!!!
                if bul || bit_depth.is_empty() {
                    wyslij_status(&mut tx, Some(T::blad(
                        "data_chck_qoi_bdepth_err".to_string()
                    ))).await;

                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla Qoi: {:?}", bit_depth)
                    ));
                }
                wyslij_status(&mut tx, Some(T::status(("data_chck_qoi_bdepth",  "✓".to_string())))).await;
            }

            ImgExt::Avif { speed, lossy, bit_depth, .. } => {
                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b,
                    BdepthAvif::Rgb8Alpha |
                    BdepthAvif::Rgb10Alpha |
                    BdepthAvif::Rgb10 |
                    BdepthAvif::Rgb8
                ));

                //WYWAL ERR JAK COŚ NIE TEGES!!!!!!!!!! YAYA!!!!!!!!!!!!!!!!!!
                if bul || bit_depth.is_empty() {
                    wyslij_status(&mut tx, Some(T::blad(
                        "data_chck_avif_bdepth_err".to_string()
                    ))).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla Avif: {:?}", bit_depth)
                    ));
                }

                *speed = (*speed).clamp(0,100);

                *lossy = match lossy{
                    None => {None}
                    Some(xx) => {
                        if *xx == 0 {
                            None
                        } else {
                            Some((*xx).clamp(0,100))
                        }
                    }
                };

                wyslij_status(&mut tx, Some(T::status(("data_chck_avif_bdepth",  "✓".to_string())))).await;


            }
        }
    }
    if !dane.opcje_rozdzielczości.iter().any(|_| true) {
        // To samo co: if dane.opcje_rozdzielczości.is_empty()
        let msg = "Nie wybrano żadnej rozdzielczości wyjściowej!";
        wyslij_status(&mut tx, Some(T::blad(msg.to_string()))).await;
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg));
    }
    let czy_mamy_rozdzielczosc = dane.opcje_rozdzielczości.iter().any(|r| {
        match r {
            Rozdzielczości::R16 => {true}
            Rozdzielczości::R32 => {true}
            Rozdzielczości::R64 => {true}
            Rozdzielczości::R128 => {true}
            Rozdzielczości::R256 => {true}
            Rozdzielczości::R512 => {true}
            Rozdzielczości::R1k => {true}
            Rozdzielczości::R2k => {true}
            Rozdzielczości::R4k => {true}
            Rozdzielczości::R6k => {true}
            Rozdzielczości::R8k => {true}
            Rozdzielczości::R16k => {true}
            Rozdzielczości::Oryginalna => {true}
        }
    });

    if !czy_mamy_rozdzielczosc {
        let msg = "data_chck_res_err";
        wyslij_status(&mut tx, Some(T::blad(msg.to_string()))).await;
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg));
    }

    wyslij_status(&mut tx, Some(T::status(("data_chck_res",  "✓".to_string())))).await;

    dane.alfa_rgb = (dane.alfa_rgb.0.clamp(0_u16,u16::MAX),dane.alfa_rgb.1.clamp(0_u16,u16::MAX),dane.alfa_rgb.2.clamp(0_u16,u16::MAX));

    dane.noising = Some(dane.noising.unwrap_or(0_u8).clamp(0_u8, 100_u8));

    let bbbbb = match dane.inter {
        OptInterpolacja::Nearest => {true}
        OptInterpolacja::Triangle => {true}
        OptInterpolacja::CatmullRom => {true}
        OptInterpolacja::Gaussian => {true}
        OptInterpolacja::Lanczos3 => {true}
    };

    if !bbbbb {
        let msg = "Błędna lub brak interpolacji";
        wyslij_status(&mut tx, Some(T::blad(msg.to_string()))).await;
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg));
    }
    wyslij_status(&mut tx, Some(T::status(("data_chck_inter",  "✓".to_string())))).await;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    wyslij_status(&mut tx, Some(T::status(("data_chck_fin","✓".to_string())))).await;

    Ok(dane)

}