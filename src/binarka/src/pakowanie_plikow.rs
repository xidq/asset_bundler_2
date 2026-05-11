use enumy::dane_do_przetwarzania::DaneBinPak;
use enumy::fn_ogolne_przeliczeniowe::przelicz_czas;
use enumy::opcje::OptKompresjaPlikówFiltracjaPlików;
use enumy::statusy::LogTxDoKompresjiPliku;
use iced::futures::channel::mpsc;
use iced::futures::SinkExt;
use kompresja::zetestede::kompresujsuj;
use std::path::{Path, PathBuf};
use std::time::Instant;
use szyfrowanie::xor_sz::szyfruj_xor;

async fn zgarnij_pliki(
    ścieżka: PathBuf,
    opcja: &bool,
    filter: OptKompresjaPlikówFiltracjaPlików,
    mut tx: mpsc::Sender<LogTxDoKompresjiPliku>,
) -> Result<Vec<(String, Vec<u8>)>, tokio::io::Error> {
    let mut lista_plików = Vec::new();
    let mut licznik:u32 = 0;
    let mut foldery_do_przejrzenia = vec![ścieżka.clone()];
    let _ = tx
        .send(LogTxDoKompresjiPliku::StatusKompresjaPlikówZnalezionePliki { pliki: 0 })
        .await;

    while let Some(aktualny_folder) = foldery_do_przejrzenia.pop() {
        let mut wejścia = tokio::fs::read_dir(aktualny_folder).await?;

        while let Some(entry) = wejścia.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                foldery_do_przejrzenia.push(path);
            } else if path.is_file() {
                if !czy_plik_pasuje(&path, &filter) {
                    continue;
                }

                let nazwa = match opcja {
                    true => path
                        .strip_prefix(&ścieżka)
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    false => path.file_name().unwrap().to_string_lossy().into_owned(),
                };

                let tresc = tokio::fs::read(&path).await?;
                lista_plików.push((nazwa, tresc));
                licznik += 1;

                let _ = tx
                    .send(
                        LogTxDoKompresjiPliku::StatusKompresjaPlikówZnalezionePliki {
                            pliki: licznik,
                        },
                    )
                    .await;
            }
        }
    }

    Ok(lista_plików)
}
fn czy_plik_pasuje(plik: &Path, filtr: &OptKompresjaPlikówFiltracjaPlików) -> bool {

    let ext = plik
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match filtr {
        OptKompresjaPlikówFiltracjaPlików::Graficzne => matches!(
            ext.as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg"
        ),
        OptKompresjaPlikówFiltracjaPlików::Audio => {
            matches!(ext.as_str(), "mp3" | "wav" | "ogg" | "flac" | "m4a")
        }
        OptKompresjaPlikówFiltracjaPlików::Tekstowe => {
            matches!(ext.as_str(), "txt" | "md" | "json" | "cfg" | "ini" | "log")
        }
        OptKompresjaPlikówFiltracjaPlików::Pdf => ext == "pdf",
        _ => true,
    }
}

async fn tworzenie_binarki(
    pliki: Vec<(String, Vec<u8>)>,
    ścieżka_wyjściowa: PathBuf,
    nazwa_pliku: String,
    mut tx: mpsc::Sender<LogTxDoKompresjiPliku>, // Dodajemy kanał tutaj
) -> Result<(), tokio::io::Error> {
    let mut akt_stat = async |aktualny:u32,suma:Option<u32>|{
        if let Err(e) = tx.send(LogTxDoKompresjiPliku::StatusKompresjaPlikówPakowanie {
            aktualny,
            suma,
        }).await {
            dbg!("Błąd wysyłania statusu do kanału: ", e);
        }
        };


    let mut blobloblob = Vec::new();
    let suma = pliki.len() as u32;
    blobloblob.extend_from_slice(&(pliki.len() as u32).to_le_bytes());


    #[allow(clippy::explicit_counter_loop)]
    for (i, (nazwa, dane)) in pliki.into_iter().enumerate() {
        let n_bytes = nazwa.as_bytes();

        // Pakowanie metadanych i danych
        blobloblob.extend_from_slice(&(n_bytes.len() as u32).to_le_bytes());
        blobloblob.extend_from_slice(n_bytes);
        blobloblob.extend_from_slice(&(dane.len() as u64).to_le_bytes());
        blobloblob.extend_from_slice(&dane);


        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        akt_stat((i + 1) as u32, Some(suma)).await;

    }

    let ścieżka_temp = ścieżka_wyjściowa.join(format!("{}.jrz_tmp", nazwa_pliku));

    if let Some(parent) = ścieżka_temp.parent() && !parent.exists() {
            tokio::fs::create_dir_all(parent).await?;
    }


    if let Err(e) = tokio::fs::write(&ścieżka_temp, blobloblob).await {
            let _ = tx
                .send(LogTxDoKompresjiPliku::StatusKompresjaPlikówBłąd(
                    e.to_string(),
                ))
                .await;
    }

    Ok(())
}

pub async fn ogarnianie_eksportu(
    zestaw_danych: DaneBinPak,
    mut tx: mpsc::Sender<LogTxDoKompresjiPliku>,
) -> Result<(), tokio::io::Error> {
    println!("zaczynam pakować");
    let start_czas = Instant::now();

    let wynik = async {
        let (in_path, out_path, poz_kompresji, strukturalnie, nazwa_pliku) = (
            zestaw_danych.ścieżka_in,
            zestaw_danych.ścieżka_out,
            zestaw_danych.kompresja,
            zestaw_danych.foldery,
            zestaw_danych.nazwa,
        );

        let wsio_dane =
            zgarnij_pliki(in_path, &strukturalnie, zestaw_danych.filtracja, tx.clone()).await?;
        // let _ = tx.send(ilość_plików).await;
        tworzenie_binarki(wsio_dane, out_path.clone(), nazwa_pliku.clone(), tx.clone()).await?;

        kompresujsuj(
            out_path.clone(),
            nazwa_pliku.clone(),
            poz_kompresji,
            tx.clone(),
        )
        .await?;

        szyfruj_xor(out_path, nazwa_pliku, tx.clone()).await?;
        Ok::<(), tokio::io::Error>(())
    }
    .await;

    match wynik {
        Ok(_) => {
            let _ = tx
                .send(LogTxDoKompresjiPliku::StatusKompresjaPlikówZakonczono { czas: przelicz_czas(start_czas) })
                .await;
            Ok(())
        }
        Err(e) => {
            let _ = tx
                .send(LogTxDoKompresjiPliku::StatusKompresjaPlikówBłąd(
                    e.to_string(),
                ))
                .await;
            Err(e)
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     // Używamy kanałów z futures, bo takie przyjmuje Twoja funkcja
//     use iced::futures::channel::mpsc;
//     use iced::futures::StreamExt; // Potrzebne dla metody .next()
//
//     #[tokio::test]
//     async fn test_ogarniania_eksportu_podstawowy() {
//         // 1. Setup
//         let zestaw = DaneDoKompresjaPlików {
//             ścieżka_in: PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/for_compression/"), // Testujemy na własnym folderze src
//             ścieżka_out: PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/compressed/"),
//             kompresjaa: OptKompresjaPlikówPoziomKompresjiZstd::Brak,
//             nazwa: "test_output".to_string(),
//             foldery: StrukturaFolderów::Tak,
//             filtracja: OptKompresjaPlikówFiltracjaPlików::Wszystkie,
//         };
//         let sciezka = PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/for_compression/");
//         println!("Czy widzę folder wejściowy? {}", sciezka.exists());
//         assert!(sciezka.exists(), "FOLDER NIE ISTNIEJE Z PERSPEKTYWY TESTU: {:?}", sciezka);
//         let sciezka2 = PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/compressed/");
//         println!("Czy widzę folder wyjściowy? {}", sciezka2.exists());
//         assert!(sciezka2.exists(), "FOLDER NIE ISTNIEJE Z PERSPEKTYWY TESTU: {:?}", sciezka2);
//         // Tworzymy kanał z biblioteki futures (ten sam co w Iced)
//         let (tx, mut rx) = mpsc::channel::<Progress>(100);
//
//         // 2. Odpalamy funkcję w tle
//         let handle = tokio::spawn(async move {
//             ogarnianie_eksportu(zestaw, tx).await
//         });
//
//         // 3. Odbieramy wiadomosci z kanału (Stream API)
//         // .next() zwraca Option<Progress>
//         // Odbieraj wszystko, co leci kanałem, aż do zamknięcia lub sukcesu
//         let mut czy_zakonczono = false;
//         while let Some(msg) = rx.next().await {
//             match msg {
//                 Progress::ZnalezionoPliki { pliki, .. } => {
//                     println!("Sukces: Znaleziono {} plików", pliki);
//                 },
//                 Progress::Proces { etap, procent } => {
//                     println!("Postęp: {:?} - {}%", etap, procent);
//                 },
//                 Progress::Zakonczono { czas, .. } => {
//                     println!("Eksport zakończony w czasie: {}", czas);
//                     czy_zakonczono = true;
//                     break; // Wychodzimy z pętli, bo to koniec
//                 },
//                 Progress::Błąd(e) => {
//                     panic!("Wystąpił błąd w trakcie procesu: {}", e);
//                 },
//                 _ => {} // Resztę (np. pakowanie) ignorujemy lub dopisujemy
//             }
//         }
//
//         assert!(czy_zakonczono, "Test zakończył się bez otrzymania komunikatu Zakonczono!");
//
//         // Na końcu sprawdzamy wynik samej funkcji (handle)
//         let wynik = handle.await.unwrap();
//         assert!(wynik.is_ok());
//     }
// }
