use futures::channel::mpsc::Sender;
use futures::SinkExt;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::opcje::OptInterpolacja;
use enumy::rozszerzenia::kompresje::ForFfKompresja;
use enumy::rozszerzenia::bdepth::{BdepthJpg, BdepthPng, BdepthQoi, BdepthTga, BdepthWebp};
use enumy::rozszerzenia::rozdzielczosci::Rozdzielczości;
use enumy::rozszerzenia::rozszerzenia::ImgExt;
use enumy::statusy::LogTxDoBathKonwersjaZdjęć;



pub async fn sprawdz_czy_wsio_ok(
    pumpum: DaneKonw,
    mut tx: Sender<LogTxDoBathKonwersjaZdjęć>
) -> Result<DaneKonw, std::io::Error>{

    let mut wyslij_update_status = {
        async |msg: String|tx
        .send(
            LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćChecking(
                msg
            ),
        )
        .await};

    if let Err(e) = wyslij_update_status("Odebrano dane, rozpoczynam analizę".to_string()).await{
        eprintln!("Nie udało się wysłać statusu do UI: {}", e);
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let mut dane = pumpum.clone();

    if !pumpum.ścieżka_wejściowa.exists() {
        let msg = "Wrong input path";
        let _ = wyslij_update_status("Ścieżka wejściowa: ".to_string() + msg).await.ok();
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, msg));
    }
    let _ = wyslij_update_status("Odebrano dane, rozpoczynam analizę".to_string()).await.ok();

    if !pumpum.ścieżka_wejściowa.exists() {
        let msg = "Wrong output path".to_string();
        let _ = wyslij_update_status(msg.clone()).await;
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, msg));
    }
    let _ =wyslij_update_status("Odebrano dane, rozpoczynam analizę".to_string()).await.ok();


    for format in &mut dane.rozszerzenia {
        match format {
            ImgExt::Jpg { jakosc, progresywny, bit_depth, sampling, quant, scans } => {
                // 1. Korygowanie jakości (0-100)
                if *jakosc > 100 { *jakosc = 100; }
                if *jakosc == 0 { *jakosc = 1; }

                // 2. Walidacja bit_depth dla JPG
                // Załóżmy, że JPG obsługuje tylko B8 (8-bit)
                // bit_depth.retain(|b| matches!(b, OptFormatyKoloruObrazOgólny::B8 | OptFormatyKoloruObrazOgólny::B8));
                //
                // let huehuehue = match random::<bool>() {
                //     true => {OptFormatyKoloruObrazOgólny::B8}
                //     false => {OptFormatyKoloruObrazOgólny::L8}
                // };
                // // Jeśli po czyszczeniu jest pusty, dodaj losowy
                // if bit_depth.is_empty() {
                //     bit_depth.push(huehuehue);
                // }
                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b, BdepthJpg::Rgb8 | BdepthJpg::Luma8));

                //WYWAL ERR JAK COŚ NIE TEGES!!!!!!!!!! YAYA!!!!!!!!!!!!!!!!!!
                if bul || bit_depth.is_empty() {
                    let _ =wyslij_update_status("Sprawdzanie danych Jpg: Err".to_string()).await.ok();
                    let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(
                        "JPG: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!".to_string()
                    )).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla JPG: {:?}", bit_depth)
                    ));
                }

                *scans = (*scans).clamp(2_u8,64_u8);

                
                let _ =wyslij_update_status("Sprawdzanie danych Jpg: Git!".to_string()).await.ok();
            }

            ImgExt::Png { kompresja, bit_depth } => {
                // 1. Korygowanie kompresji (np. 0-9)
                *kompresja = (*kompresja).clamp(0_u8, 9_u8);

                // 2. Porównanie/Korekta bit_depth
                // Możesz np. usunąć duplikaty
                // bit_depth.sort();
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

                //WYWAL ERR JAK COŚ NIE TEGES!!!!!!!!!! YAYA!!!!!!!!!!!!!!!!!!
                if bul || bit_depth.is_empty() {
                    let _ = wyslij_update_status("Sprawdzanie danych Png: Err".to_string()).await.ok();
                    let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(
                        "Png: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!".to_string()
                    )).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla Png: {:?}", bit_depth)
                    ));
                }
                let _ = wyslij_update_status("Sprawdzanie danych Png: Git!".to_string()).await.ok();
            }

            ImgExt::Webp { jakosc, lossless, bit_depth } => {

                if *jakosc > 100 { *jakosc = 100; }
                if *jakosc == 0 { *jakosc = 1; }

                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b, BdepthWebp::Rgb8 | BdepthWebp::Rgb8Alpha));

                //WYWAL ERR JAK COŚ NIE TEGES!!!!!!!!!! YAYA!!!!!!!!!!!!!!!!!!
                if bul || bit_depth.is_empty() {
                    let _ = wyslij_update_status("Sprawdzanie danych Webp: Err".to_string()).await.ok();
                    let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(
                        "Webp: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!".to_string()
                    )).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla Webp: {:?}", bit_depth)
                    ));
                }
                let _ = wyslij_update_status("Sprawdzanie danych Webp: Git!".to_string()).await.ok();
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
                    let _ = wyslij_update_status("Sprawdzanie danych Tga: Err".to_string()).await.ok();
                    let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(
                        "Tga: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!".to_string()
                    )).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla Tga: {:?}", bit_depth)
                    ));
                }
                let _ = wyslij_update_status("Sprawdzanie danych Tga: Git!".to_string()).await.ok();
            }
            ImgExt::Ff {  metoda_kompresji } => {

                let bul = matches!(metoda_kompresji,
                    ForFfKompresja::Bzip2(_) |
                    ForFfKompresja::Zstd(_) |
                    ForFfKompresja::Xz(_) |
                    ForFfKompresja::Brak
                );

                //WYWAL ERR JAK COŚ NIE TEGES!!!!!!!!!! YAYA!!!!!!!!!!!!!!!!!!
                if !bul {
                    let _ = wyslij_update_status("Sprawdzanie danych Ff: Err".to_string()).await.ok();
                    let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(
                        "Ff: Wykryto nieobsługiwaną kompresję".to_string()
                    )).await;

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
                let _ = wyslij_update_status("Sprawdzanie danych Ff: Git!".to_string()).await.ok();
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
                    let _ = wyslij_update_status("Sprawdzanie danych Qoi: Err".to_string()).await.ok();
                    let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(
                        "Qoi: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!".to_string()
                    )).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla Qoi: {:?}", bit_depth)
                    ));
                }
                let _ = wyslij_update_status("Sprawdzanie danych Qoi: Git!".to_string()).await.ok();
            }

            _ => {} // Reszta formatów (Tga, Ff, Webp)
        }
    }
    // Sprawdzamy, czy istnieje jakikolwiek element (zakładając, że typ OptRozdzielczościObrazów sam w sobie jest poprawny)
    if !dane.opcje_rozdzielczości.iter().any(|_| true) {
        // To samo co: if dane.opcje_rozdzielczości.is_empty()
        let msg = "Nie wybrano żadnej rozdzielczości wyjściowej!";
        let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(msg.to_string())).await;
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
        let _ = wyslij_update_status("Sprawdzanie rozdzielczości: Err".to_string()).await.ok();
        let msg = "Błędna lub brak rozdzielczości";
        let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(msg.to_string())).await;
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg));
    }
    
    let _ = wyslij_update_status("Sprawdzanie rozdzielczości: Git!".to_string()).await.ok();
    
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
        let _ = wyslij_update_status("Sprawdzanie interpolacji: Err".to_string()).await.ok();
        let msg = "Błędna lub brak interpolacji";
        let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(msg.to_string())).await;
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, msg));
    }
    let _ = wyslij_update_status("Sprawdzanie interpolacji: Git!".to_string()).await.ok();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let _ = wyslij_update_status("Dane sprawdzone".to_string()).await.ok();

    Ok(dane)

}