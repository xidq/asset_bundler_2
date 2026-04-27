use std::io::ErrorKind;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use futures::channel::mpsc::Sender;
use futures::SinkExt;
use image::{
    ColorType, DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, LumaA, Rgb, Rgba,
};
use rand::{random, random_bool, random_iter};
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::opcje::{OptFormatyKoloruObrazOgólny, OptFormatyKoloruObrazuQoi, OptFormatyKoloruObrazuTga, OptInterpolacja, OptMetodaKompresjiZdjecia, OptRozdzielczościObrazów, OptRozszerzeniaPlikówZdjęciowych};
use enumy::statusy::LogTxDoBathKonwersjaZdjęć;



pub async fn sprawdz_czy_wsio_ok(
    pumpum:DaneDoBathKonwersjaZdjec,
    mut tx: Sender<LogTxDoBathKonwersjaZdjęć>
) -> Result<DaneDoBathKonwersjaZdjec, std::io::Error>{

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


    for format in &mut dane.rozszerzenia_plików_zdjęciowych {
        match format {
            OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, progresywny, bit_depth } => {
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
                let bul = bit_depth.iter().any(|b| !matches!(b, OptFormatyKoloruObrazOgólny::B8 | OptFormatyKoloruObrazOgólny::L8));

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
                let _ =wyslij_update_status("Sprawdzanie danych Jpg: Git!".to_string()).await.ok();
            }

            OptRozszerzeniaPlikówZdjęciowych::Png { kompresja, bit_depth } => {
                // 1. Korygowanie kompresji (np. 0-9)
                *kompresja = (*kompresja).clamp(0_u8, 9_u8);

                // 2. Porównanie/Korekta bit_depth
                // Możesz np. usunąć duplikaty
                // bit_depth.sort();
                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b, 
                    OptFormatyKoloruObrazOgólny::B8 | 
                    OptFormatyKoloruObrazOgólny::L8 |
                    OptFormatyKoloruObrazOgólny::B16 | 
                    OptFormatyKoloruObrazOgólny::L16 |
                    OptFormatyKoloruObrazOgólny::B8a | 
                    OptFormatyKoloruObrazOgólny::L8a |
                    OptFormatyKoloruObrazOgólny::B16a | 
                    OptFormatyKoloruObrazOgólny::L16a 
                ));

                //WYWAL ERR JAK COŚ NIE TEGES!!!!!!!!!! YAYA!!!!!!!!!!!!!!!!!!
                if bul || bit_depth.is_empty() {
                    let _ = wyslij_update_status("Sprawdzanie danych Png: Err".to_string()).await.ok();
                    let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(
                        "JPG: Wykryto nieobsługiwaną głębię bitową lub brak wyboru!".to_string()
                    )).await;

                    // PRZERWANIE: Zwracamy błąd, co kończy działanie pętli i całej funkcji
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Niepoprawny format koloru dla Png: {:?}", bit_depth)
                    ));
                }
                let _ = wyslij_update_status("Sprawdzanie danych Png: Git!".to_string()).await.ok();
            }

            OptRozszerzeniaPlikówZdjęciowych::Webp { jakosc, lossless, bit_depth } => {

                if *jakosc > 100 { *jakosc = 100; }
                if *jakosc == 0 { *jakosc = 1; }

                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b, OptFormatyKoloruObrazOgólny::B8 | OptFormatyKoloruObrazOgólny::B8a));

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
            OptRozszerzeniaPlikówZdjęciowych::Tga {  bit_depth } => {
                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b, 
                    OptFormatyKoloruObrazuTga::TrueColorA32 | 
                    OptFormatyKoloruObrazuTga::TrueColor24 |
                    OptFormatyKoloruObrazuTga::HighColor16 |
                    OptFormatyKoloruObrazuTga::Szary8
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
            OptRozszerzeniaPlikówZdjęciowych::Ff {  metoda_kompresji } => {

                let bul = matches!(metoda_kompresji,
                    OptMetodaKompresjiZdjecia::Bzip2(_) |
                    OptMetodaKompresjiZdjecia::Zstd(_) |
                    OptMetodaKompresjiZdjecia::Xz(_) |
                    OptMetodaKompresjiZdjecia::Brak
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
                    OptMetodaKompresjiZdjecia::Zstd(xx) => {*xx = (*xx).clamp(1_u8,22_u8);}
                    OptMetodaKompresjiZdjecia::Bzip2(xx) => {*xx = (*xx).clamp(1_u8,22_u8);}
                    OptMetodaKompresjiZdjecia::Xz(xx) => {*xx = (*xx).clamp(1_u8,22_u8);}
                    OptMetodaKompresjiZdjecia::Brak => {}
                }
                let _ = wyslij_update_status("Sprawdzanie danych Ff: Git!".to_string()).await.ok();
            }

            OptRozszerzeniaPlikówZdjęciowych::Qoi { bit_depth } => {
                bit_depth.dedup();
                let bul = bit_depth.iter().any(|b| !matches!(b,
                    OptFormatyKoloruObrazuQoi::Color24 |
                    OptFormatyKoloruObrazuQoi::ColorA32
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
            OptRozdzielczościObrazów::R16 => {true}
            OptRozdzielczościObrazów::R32 => {true}
            OptRozdzielczościObrazów::R64 => {true}
            OptRozdzielczościObrazów::R128 => {true}
            OptRozdzielczościObrazów::R256 => {true}
            OptRozdzielczościObrazów::R512 => {true}
            OptRozdzielczościObrazów::R1k => {true}
            OptRozdzielczościObrazów::R2k => {true}
            OptRozdzielczościObrazów::R4k => {true}
            OptRozdzielczościObrazów::R6k => {true}
            OptRozdzielczościObrazów::R8k => {true}
            OptRozdzielczościObrazów::R16k => {true}
            OptRozdzielczościObrazów::Oryginalna => {true}
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