
use encodery::check::sprawdzacz;
use encodery::halper::merge_sciezki;
use encodery::wczytaj_foto::wczytaj_zdjęcie;
use enumy::dane_do_przetwarzania::DaneKonw;
use enumy::enums_structs_io::FILTERFOTO;
use enumy::rozszerzenia::ext::ImgExt;
use enumy::statusy::LogTxKonw;
use futures::channel::mpsc;
use futures::executor::block_on;
use futures::SinkExt;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use walkdir::WalkDir;
use encodery::zapisywanie::generic::zapisywanie_generic;
use enumy::przetwarzanie::{PrzetwarzanieAvif, PrzetwarzanieFf, PrzetwarzanieJpg, PrzetwarzaniePng, PrzetwarzanieQoi, PrzetwarzanieTga, PrzetwarzanieWebp};

pub async fn ogarnianie_foto(
    zestaw_danych: DaneKonw,
    mut tx: mpsc::Sender<LogTxKonw>,
) -> Result<(), tokio::io::Error> {

    let start_czas = Instant::now();
    let obecna_operacja: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let saf = sprawdzacz(zestaw_danych, tx.clone()).await?;
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

        for rozszerzenie in &wsio_dane.rozszerzenia {
            match rozszerzenie {
                ImgExt::Jpg { bit_depth, .. } => {
                    // Jeśli JPG ma zaznaczone L8 i B8, to są 2 warianty
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                ImgExt::Png { bit_depth, .. } => {
                    // Jeśli PNG ma zaznaczone B8, B16, L16, to są 3 warianty
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                ImgExt::Webp { bit_depth, .. } => {
                    // Webp u Ciebie nie ma bit_depth w enumie, więc liczymy jako 1
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                ImgExt::Tga {bit_depth, ..} => {
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                ImgExt::Ff { .. }  => {
                    suma_wariantow_bit_depth += 1;
                }
                ImgExt::Qoi { bit_depth } => {
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                ImgExt::Avif { bit_depth,.. } => {
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
            }
        }


        let total_operacji = ścieżki_do_zdjęć.len() as u32 * ile_rozdzielczosci * suma_wariantow_bit_depth;
        let metryka_operacji = Some(total_operacji * 3);

        // let _ = tx.send(LogTxDoBathKonwersjaZdjęć::StatusBathKonwersjaZdjęćFiltrowaniePlików(ścieżki_do_zdjęć.len() as u32)).await;

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

                        let mut oopr = obecna_operacja.blocking_lock();
                        let ile_rozszerzen = wsio_dane.rozszerzenia.len() as u32;
                        let ile_rozdzielczosci = wsio_dane.opcje_rozdzielczości.len() as u32;
                        *oopr += ile_rozszerzen * ile_rozdzielczosci;
                        drop(oopr);




                        let nazwa_pliku = p.0.file_name()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_else(|| "Nieznany".to_string());

                        let powod_bledu = e.to_string();

                        // let _ = tx_dla_rayona.clone().send(LogTxDoBathKonwersjaZdjęć::PominiętePliki { sciezka: nazwa_pliku, powod: powod_bledu });
                        let wynik_wysylki = tx_dla_rayona.clone().try_send(LogTxKonw::Pominięte {
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



                for r in &wsio_dane.rozszerzenia {

                    block_on(async {
                        let tx_zadanie = tx_dla_rayona.clone();

                        match &r {
                            ImgExt::Jpg { jakosc, progresywny, bit_depth, sampling, quant, scans, } => {
                                let sciezka = merge_sciezki(&wsio_dane.ścieżka_wyjściowa,&p.2);
                                let dane = PrzetwarzanieJpg{
                                    bufor: bufor.clone(),
                                    rozdzielczosci: wsio_dane.opcje_rozdzielczości.clone(),
                                    sciezka_wyjsciowa: sciezka,
                                    nazwa: nazwa.clone(),
                                    interpolacja: wsio_dane.inter,
                                    jakosc: *jakosc,
                                    progresywny: *progresywny,
                                    bdepth: bit_depth.clone(),
                                    sampling: *sampling,
                                    quant: *quant,
                                    skany: *scans,
                                    alpha: wsio_dane.alfa_rgb,
                                    zaszumienie: wsio_dane.noising,
                                };
                                zapisywanie_generic(
                                    dane,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            ImgExt::Png { kompresja, bit_depth } => {
                                let sciezka = merge_sciezki(&wsio_dane.ścieżka_wyjściowa,&p.2);
                                let dane = PrzetwarzaniePng{
                                    bufor: bufor.clone(),
                                    rozdzielczosci: wsio_dane.opcje_rozdzielczości.clone(),
                                    sciezka_wyjsciowa: sciezka,
                                    nazwa: nazwa.clone(),
                                    interpolacja: wsio_dane.inter,
                                    kompresja: *kompresja,
                                    bdepth: bit_depth.clone(),
                                    alpha: wsio_dane.alfa_rgb,
                                    zaszumienie: wsio_dane.noising,
                                };
                                zapisywanie_generic(
                                    dane,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            ImgExt::Webp { jakosc , lossless, bit_depth} => {
                                let sciezka = merge_sciezki(&wsio_dane.ścieżka_wyjściowa,&p.2);
                                let dane = PrzetwarzanieWebp{
                                    bufor: bufor.clone(),
                                    rozdzielczosci: wsio_dane.opcje_rozdzielczości.clone(),
                                    sciezka_wyjsciowa: sciezka,
                                    nazwa: nazwa.clone(),
                                    interpolacja: wsio_dane.inter,
                                    bdepth: bit_depth.clone(),
                                    alpha: wsio_dane.alfa_rgb,
                                    zaszumienie: wsio_dane.noising,
                                    lossy: if *lossless { None } else { Some(*jakosc) },
                                };
                                zapisywanie_generic(
                                    dane,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            ImgExt::Tga { bit_depth } => {
                                let sciezka = merge_sciezki(&wsio_dane.ścieżka_wyjściowa,&p.2);
                                let dane = PrzetwarzanieTga{
                                    bufor: bufor.clone(),
                                    rozdzielczosci: wsio_dane.opcje_rozdzielczości.clone(),
                                    sciezka_wyjsciowa: sciezka,
                                    nazwa: nazwa.clone(),
                                    interpolacja: wsio_dane.inter,
                                    bdepth: bit_depth.clone(),
                                    alpha: wsio_dane.alfa_rgb,
                                    zaszumienie: wsio_dane.noising,
                                };
                                zapisywanie_generic(
                                    dane,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            ImgExt::Ff { metoda_kompresji } => {
                                let sciezka = merge_sciezki(&wsio_dane.ścieżka_wyjściowa,&p.2);
                                let dane = PrzetwarzanieFf{
                                    bufor: bufor.clone(),
                                    rozdzielczosci: wsio_dane.opcje_rozdzielczości.clone(),
                                    sciezka_wyjsciowa: sciezka,
                                    nazwa: nazwa.clone(),
                                    interpolacja: wsio_dane.inter,
                                    kompresja: vec![*metoda_kompresji],
                                    alpha: wsio_dane.alfa_rgb,
                                    zaszumienie: wsio_dane.noising,
                                };
                                zapisywanie_generic(
                                    dane,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            ImgExt::Qoi { bit_depth } => {
                                let sciezka = merge_sciezki(&wsio_dane.ścieżka_wyjściowa,&p.2);
                                let dane = PrzetwarzanieQoi{
                                    bufor: bufor.clone(),
                                    rozdzielczosci: wsio_dane.opcje_rozdzielczości.clone(),
                                    sciezka_wyjsciowa: sciezka,
                                    nazwa: nazwa.clone(),
                                    interpolacja: wsio_dane.inter,
                                    bdepth: bit_depth.clone(),
                                    alpha: wsio_dane.alfa_rgb,
                                    zaszumienie: wsio_dane.noising,
                                };
                                zapisywanie_generic(
                                    dane,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            ImgExt::Avif {
                                chroma,
                                speed,
                                metoda_kompresji,
                                lossy,
                                bit_depth
                            } => {
                                let sciezka = merge_sciezki(&wsio_dane.ścieżka_wyjściowa,&p.2);
                                let dane = PrzetwarzanieAvif{
                                    bufor: bufor.clone(),
                                    rozdzielczosci: wsio_dane.opcje_rozdzielczości.clone(),
                                    sciezka_wyjsciowa: sciezka,
                                    nazwa: nazwa.clone(),
                                    interpolacja: wsio_dane.inter,
                                    bdepth: bit_depth.clone(),
                                    alpha: wsio_dane.alfa_rgb,
                                    zaszumienie: wsio_dane.noising,
                                    chroma: chroma.clone(),
                                    speed: *speed,
                                    metoda_kompresji: metoda_kompresji.clone(),
                                    lossy: *lossy,
                                };
                                zapisywanie_generic(
                                    dane,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                        }
                        Ok::<(), tokio::io::Error>(())
                    })?;
                }
                Ok::<(), tokio::io::Error>(())
            })
        }).await.map_err(tokio::io::Error::other)?
    }.await;

    match wynik {
        Ok(_) => {
            let trwanie = start_czas.elapsed(); // Zwraca strukturę Duration

            // Formatujemy czas na ładny napis, np. "1.23s" lub "45ms"
            // Możesz użyć prostego formatowania:
            let czas_napis = format!("{:.2?}", trwanie);
            let _ = tx
                .send(LogTxKonw::Finito(czas_napis))
                .await;
            Ok(())
        }
        Err(e) => {
            // Jeśli cokolwiek powyżej sypnie błędem (przez znak zapytania),
            // wysyłamy opis błędu do UI zamiast po prostu "padać".
            let _ = tx
                .send(LogTxKonw::Błąd(e.to_string()))
                .await;
            Err(e)
        }
    }
}

fn wez_sprawdz_sciezki(
    sciezka: PathBuf,
    tx: &mut mpsc::Sender<LogTxKonw>,
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
                        LogTxKonw::FiltrowaniePlików(
                            Some(przetworzone_pliki),
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
    tx: &mut mpsc::Sender<LogTxKonw>,
) -> Vec<(PathBuf, String, String)> {
    // println!("jestem w zgarnij dane z pliku!!!!!");
    let mut przetworzone_pliki: u32 = 0;
    let mut do_wyjscia = Vec::new();
    let opt_rozszerzenia_plików_zdjęciowych: [&str; 16] =
        FILTERFOTO.map(|item| item.strip_prefix("ff.").unwrap_or(item));
    println!("jestem w zgarnij_dane_z_pliku");


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


                let nazwa_pliku = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                przetworzone_pliki += 1;
                let _ = tx.try_send(
                    LogTxKonw::FiltrowaniePlików(
                        Some(przetworzone_pliki),
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
    tx: &mut mpsc::Sender<LogTxKonw>,
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
                LogTxKonw::Pominięte {
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
