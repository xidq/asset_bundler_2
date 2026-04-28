use crate::edycja_ff::edycja_ff;
use crate::edycja_jpg::edycja_jpg;
use crate::edycja_png::edycja_png;
use crate::edycja_qoi::edycja_qoi;
use crate::edycja_tga::edycja_tga;
use crate::edycja_webp::edycja_webp;
use enumy::dane_do_przetwarzania::DaneDoBathKonwersjaZdjec;
use enumy::enums_structs_io::FILTERFOTO;
use enumy::opcje::OptRozszerzeniaPlikówZdjęciowych;
use enumy::statusy::LogTxDoBathKonwersjaZdjęć;
use futures::SinkExt;
use futures::channel::mpsc;
use futures::executor::block_on;
use image::{DynamicImage, GenericImageView};
use rand::RngExt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use walkdir::WalkDir;
use encodery::halper::merge_sciezki;
use encodery::wczytaj_foto::wczytaj_zdjęcie;
use crate::edycja_avif::edycja_avif;
use crate::pomocnicze::sprawdz_czy_wsio_ok;

pub async fn ogarnianie_foto(
    zestaw_danych: DaneDoBathKonwersjaZdjec,
    mut tx: mpsc::Sender<LogTxDoBathKonwersjaZdjęć>,
) -> Result<(), tokio::io::Error> {
    let start_czas = Instant::now();
    let obecna_operacja: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let procent_progress: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
    let saf = sprawdz_czy_wsio_ok(zestaw_danych, tx.clone()).await?;
    let wsio_dane = Arc::new(saf);

    let wynik = async {



        let ścieżki_do_zdjęć = if !wsio_dane.ścieżka_wejściowa.is_file() {
            // println!("to jest na foldery");
            wez_sprawdz_sciezki(wsio_dane.ścieżka_wejściowa.clone(), &mut tx)
        } else {
            // println!("to jest na pliki");
            zgarnij_dane_z_pliku(wsio_dane.ścieżka_wejściowa.clone(), &mut tx)
        };


        let ile_rozdzielczosci = wsio_dane.opcje_rozdzielczości.len() as u32;

        let mut suma_wariantow_bit_depth = 0u32;

        for rozszerzenie in &wsio_dane.rozszerzenia_plików_zdjęciowych {
            match rozszerzenie {
                OptRozszerzeniaPlikówZdjęciowych::Jpg { bit_depth, .. } => {
                    // Jeśli JPG ma zaznaczone L8 i B8, to są 2 warianty
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                OptRozszerzeniaPlikówZdjęciowych::Png { bit_depth, .. } => {
                    // Jeśli PNG ma zaznaczone B8, B16, L16, to są 3 warianty
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                OptRozszerzeniaPlikówZdjęciowych::Webp { bit_depth, .. } => {
                    // Webp u Ciebie nie ma bit_depth w enumie, więc liczymy jako 1
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                OptRozszerzeniaPlikówZdjęciowych::Tga {bit_depth, ..} => {
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                OptRozszerzeniaPlikówZdjęciowych::Ff { .. }  => {
                    suma_wariantow_bit_depth += 1;
                }
                OptRozszerzeniaPlikówZdjęciowych::Qoi { bit_depth } => {
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                OptRozszerzeniaPlikówZdjęciowych::Avif { bit_depth,.. } => {
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
            }
        }


        let total_operacji = ścieżki_do_zdjęć.len() as u32 * ile_rozdzielczosci * suma_wariantow_bit_depth;
        let metryka_operacji = total_operacji * 3;

        let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćFiltrowaniePlików(ścieżki_do_zdjęć.len() as u32)).await;

        let tx_dla_rayona = tx.clone();

            // for p in ścieżki_do_zdjęć{
            //     let (bufor, nazwa) = wczytaj_zdjęcie(p.0)?;
        tokio::task::spawn_blocking(move || {
            ścieżki_do_zdjęć.par_iter().try_for_each(|p| {
                // let (bufor, nazwa) = wczytaj_zdjęcie(p.0.clone())?;

                let (bufor, nazwa) = match wczytaj_zdjęcie(p.0.clone()) {
                    Ok(dane) => dane,
                    Err(e) => {
                        eprintln!("Pomijam uszkodzony plik {:?}: {}", p.0, e);
                        // Opcjonalnie: wyślij tx.blocking_send z informacją o błędzie konkretnego pliku

                        // WAŻNE: Musimy i tak podbić licznik o tyle, ile ten plik miał mieć operacji!
                        let mut oopr = obecna_operacja.blocking_lock();
                        let ile_rozszerzen = wsio_dane.rozszerzenia_plików_zdjęciowych.len() as u32;
                        let ile_rozdzielczosci = wsio_dane.opcje_rozdzielczości.len() as u32;
                        *oopr += ile_rozszerzen * ile_rozdzielczosci;
                        drop(oopr);




                        let nazwa_pliku = p.0.file_name()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_else(|| "Nieznany".to_string());

                        let powod_bledu = e.to_string();

                        // let _ = tx_dla_rayona.clone().send(LogTxDoBathKonwersjaZdjęć::PominiętePliki { sciezka: nazwa_pliku, powod: powod_bledu });
                        let wynik_wysylki = tx_dla_rayona.clone().try_send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćPominiętePliki {
                            sciezka: nazwa_pliku.clone(),
                            powod: powod_bledu
                        });

                        // match wynik_wysylki {
                        //     Ok(_) => println!("POSZŁO DO KANAŁU: {}", nazwa_pliku),
                        //     Err(err) => eprintln!("KANAŁ ZAMKNIĘTY LUB PADŁ! Błąd: {}", err),
                        // }
                        match wynik_wysylki {
                            Ok(_) => println!("POSZŁO DO UI: {}", nazwa_pliku),
                            Err(err) => {
                                if err.is_disconnected() {
                                    eprintln!("KANAŁ ZAMKNIĘTY!");
                                } else if err.is_full() {
                                    eprintln!("KANAŁ JEST PEŁNY! (Zwiększ bufor w Iced)");
                                }
                            }
                        }

                        return Ok::<(), tokio::io::Error>(()); // Kontynuuj pętlę (pomiń ten plik)
                    }
                };
                let cimcirimcim = false;



                for r in &wsio_dane.rozszerzenia_plików_zdjęciowych {

                    block_on(async {
                        let tx_zadanie = tx_dla_rayona.clone();

                        match &r {
                            OptRozszerzeniaPlikówZdjęciowych::Jpg { jakosc, progresywny, bit_depth, sampling, quant, scans, } => {
                                let sciezka = merge_sciezki(&wsio_dane.ścieżka_wyjściowa,&p.2);
                                let zbiór_danych = (jakosc,progresywny,bit_depth,sampling,quant,scans);
                                edycja_jpg(
                                    bufor.clone(),
                                    &wsio_dane.opcje_rozdzielczości,
                                    &sciezka,
                                    &wsio_dane.inter,
                                    &nazwa,
                                    zbiór_danych,
                                    &wsio_dane.alfa_rgb,
                                    wsio_dane.noising,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    procent_progress.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            OptRozszerzeniaPlikówZdjęciowych::Png { kompresja, bit_depth } => {
                                edycja_png(
                                    bufor.clone(),
                                    &wsio_dane.opcje_rozdzielczości,
                                    &wsio_dane.ścieżka_wyjściowa,
                                    &p.2,
                                    &wsio_dane.inter,
                                    &nazwa,
                                    kompresja,
                                    &wsio_dane.alfa_rgb,
                                    bit_depth,
                                    wsio_dane.noising,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    procent_progress.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            OptRozszerzeniaPlikówZdjęciowych::Webp { jakosc , lossless, bit_depth} => {
                                edycja_webp(
                                    bufor.clone(),
                                    &wsio_dane.opcje_rozdzielczości,
                                    &wsio_dane.ścieżka_wyjściowa,
                                    &p.2, //ścieżka dopełniająca
                                    &wsio_dane.inter,
                                    &nazwa,
                                    jakosc,
                                    *lossless,
                                    bit_depth,
                                    &wsio_dane.alfa_rgb,
                                    wsio_dane.noising,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    procent_progress.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            OptRozszerzeniaPlikówZdjęciowych::Tga { bit_depth } => {
                                edycja_tga(
                                    bufor.clone(),
                                    &wsio_dane.opcje_rozdzielczości,
                                    &wsio_dane.ścieżka_wyjściowa,
                                    &p.2,
                                    &wsio_dane.inter,
                                    &nazwa,
                                    &wsio_dane.alfa_rgb,
                                    bit_depth,
                                    wsio_dane.noising,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    procent_progress.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            OptRozszerzeniaPlikówZdjęciowych::Ff { metoda_kompresji } => {
                                edycja_ff(
                                    bufor.clone(),
                                    &wsio_dane.opcje_rozdzielczości,
                                    &wsio_dane.ścieżka_wyjściowa,
                                    &p.2,
                                    &wsio_dane.inter,
                                    &nazwa,
                                    &wsio_dane.alfa_rgb,
                                    wsio_dane.noising,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    procent_progress.clone(),
                                    metoda_kompresji,
                                    tx_zadanie,
                                ).await?;
                            }
                            OptRozszerzeniaPlikówZdjęciowych::Qoi { bit_depth } => {
                                dbg!("wchodzę w fn edycja_qoi");
                                edycja_qoi(
                                    bufor.clone(),
                                    &wsio_dane.opcje_rozdzielczości,
                                    &wsio_dane.ścieżka_wyjściowa,
                                    &p.2,
                                    &wsio_dane.inter,
                                    &nazwa,
                                    &wsio_dane.alfa_rgb,
                                    bit_depth,
                                    wsio_dane.noising,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    procent_progress.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            OptRozszerzeniaPlikówZdjęciowych::Avif {
                                chroma,
                                speed,
                                metoda_kompresji,
                                lossy,
                                bit_depth
                            } => {
                                edycja_avif(
                                    bufor.clone(),
                                    &wsio_dane.opcje_rozdzielczości,
                                    &wsio_dane.ścieżka_wyjściowa,
                                    &p.2,
                                    &wsio_dane.inter,
                                    &nazwa,
                                    lossy,
                                    bit_depth,
                                    None,
                                    &wsio_dane.alfa_rgb,
                                    metoda_kompresji,
                                    *speed,
                                    chroma,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    procent_progress.clone(),
                                    tx_zadanie
                                ).await?;
                            }
                        }
                        Ok::<(), tokio::io::Error>(())
                    })?;
                }
                Ok::<(), tokio::io::Error>(())
            })
        }).await.map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::Other, e.to_string()))?
    }.await;

    match wynik {
        Ok(_) => {
            let trwanie = start_czas.elapsed(); // Zwraca strukturę Duration

            // Formatujemy czas na ładny napis, np. "1.23s" lub "45ms"
            // Możesz użyć prostego formatowania:
            let czas_napis = format!("{:.2?}", trwanie);
            let _ = tx
                .send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćKoniec(czas_napis))
                .await;
            Ok(())
        }
        Err(e) => {
            // Jeśli cokolwiek powyżej sypnie błędem (przez znak zapytania),
            // wysyłamy opis błędu do UI zamiast po prostu "padać".
            let _ = tx
                .send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćBłąd(e.to_string()))
                .await;
            Err(e)
        }
    }
}

fn wez_sprawdz_sciezki(
    sciezka: PathBuf,
    tx: &mut mpsc::Sender<LogTxDoBathKonwersjaZdjęć>,
) -> Vec<(PathBuf, String, String)> {
    let opt_rozszerzenia_plików_zdjęciowych: [&str; 16] =
        FILTERFOTO.map(|item| item.strip_prefix("ff.").unwrap_or(item));
    // FILTERFOTO
    let mut przetworzone_pliki: u32 = 0;
    let mut do_wyjscia = Vec::new();

    // WalkDir jako iterator
    for entry in WalkDir::new(&sciezka).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

            if opt_rozszerzenia_plików_zdjęciowych.contains(&ext.to_lowercase().as_str()) {
                let pełna_ścieżka = path.to_path_buf();

                // BRAMKARZ - sprawdzamy czy ścieżka jest git
                // Jeśli nie, funkcja czy_sciezka_jest_git sama wyśle raport przez tx
                if let Some(sprawdzona_ścieżka) = czy_sciezka_jest_git(pełna_ścieżka, tx) {
                    let nazwa_pliku = sprawdzona_ścieżka
                        .file_stem()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "nieznany".to_string());

                    let ścieżka_dopełniająca = sprawdzona_ścieżka
                        .strip_prefix(&sciezka)
                        .ok()
                        .and_then(|p| p.parent())
                        .map(|p| p.to_string_lossy().into_owned())
                        .unwrap_or_default();

                    // NALICZANIE
                    przetworzone_pliki += 1;
                    let _ = tx.try_send(
                        LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćFiltrowaniePlików(
                            przetworzone_pliki,
                        ),
                    );

                    do_wyjscia.push((sprawdzona_ścieżka, nazwa_pliku, ścieżka_dopełniająca));
                }
            }
        }
    }

    do_wyjscia
}

fn zgarnij_dane_z_pliku(
    ścieżka: PathBuf,
    tx: &mut mpsc::Sender<LogTxDoBathKonwersjaZdjęć>,
) -> Vec<(PathBuf, String, String)> {
    // println!("jestem w zgarnij dane z pliku!!!!!");
    let mut przetworzone_pliki: u32 = 0;
    let mut do_wyjscia = Vec::new();
    let opt_rozszerzenia_plików_zdjęciowych: [&str; 16] =
        FILTERFOTO.map(|item| item.strip_prefix("ff.").unwrap_or(item));
    println!("jestem w zgarnij_dane_z_pliku");

    // Używamy WalkDir, żeby ogarnąć foldery i podfoldery
    for entry in WalkDir::new(&ścieżka).into_iter().flatten() {
        let path = entry.path();

        if path.is_file() {
            // 1. Sprawdzamy rozszerzenie
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            if opt_rozszerzenia_plików_zdjęciowych.contains(&ext.as_str()) {
                // 2. Wyciągamy ścieżkę do folderu (bez nazwy pliku)
                // parent() zwraca ścieżkę o jeden poziom wyżej
                let sciezka_bez_pliku = path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(PathBuf::new);

                // 3. Wyciągamy nazwę pliku bez rozszerzenia_plików_zdjęciowych (file_stem)
                let nazwa_pliku = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                przetworzone_pliki += 1;
                let _ = tx.try_send(
                    LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćFiltrowaniePlików(
                        przetworzone_pliki,
                    ),
                );
                // println!("[zgarnij_dane_z_pliku]ścieżka: {:?}\nnazwa pliku:{:?}\nścieżka dopełniająca: {}",ścieżka.clone(), nazwa_pliku,String::from(""));
                do_wyjscia.push((ścieżka.clone(), nazwa_pliku, String::from("")));
            }
        }
    }

    do_wyjscia
}


// fn losuj_i_wyrownaj(v: f64, rng: &mut impl Rng, max_delta: f64, max_val: f64) -> f64 {
//
//     let delta: f64 = rng.random_range(-max_delta..=max_delta);
//     let val = v + delta;
//
//     // Clamp zależny od bit-depth (max_val)
//     val.clamp(0.0, max_val)
//
// }
// fn wyczysc_sciezke(s: String) -> String {
//     s.chars().map(|c| {
//         match c {
//             // Zamień japońskie nawiasy, ukośniki i inne dziwadła na bezpieczny znak
//             '｢' | '｣' | '／' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
//             // Opcjonalnie: jeśli chcesz być super bezpieczny, usuń znaki spoza ASCII
//             _ if !c.is_ascii() && c.is_control() => '_',
//             _ => c,
//         }
//     }).collect()
// }

fn czy_sciezka_jest_git(
    pelna: PathBuf,
    tx: &mut mpsc::Sender<LogTxDoBathKonwersjaZdjęć>,
) -> Option<PathBuf> {
    let s = pelna.to_string_lossy();
    // let zakazane_znaki = ['／', '\0', '｢', '｣', '\\', '*', '?', '"', '<', '>', '|'];

    // Szukamy pierwszego wystąpienia zakazanego znaku
    // if let Some(znaleziony_znak) = s.chars().find(|c| zakazane_znaki.contains(c)) {
    //     let _ = tx.try_send(LogTxDoBathKonwersjaZdjęć::PominiętePliki {
    //         sciezka: s.to_string(),
    //         powod: format!("Niedozwolony znak ({}) w ścieżce", znaleziony_znak)
    //     });
    //     return None;
    // }

    // 2. Limit bajtowy dla Linuxa (255 bajtów na folder/plik)
    for komponent in pelna.components() {
        if komponent.as_os_str().as_encoded_bytes().len() > 250 {
            let _ = tx.try_send(
                LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćPominiętePliki {
                    sciezka: s.to_string(),
                    powod: format!(
                        "Człon ścieżki przekracza limit 250 bajtów, jest {} bajtów",
                        komponent.as_os_str().as_encoded_bytes().len()
                    ),
                },
            );
            return None;
        }
    }

    // Jak wszystko przeszło, zwracamy ścieżkę z powrotem
    Some(pelna)
}
