use std::path::{Path, PathBuf};
use std::time::Instant;
use iced::futures::channel::mpsc;
use iced::futures::SinkExt;
use iced::window::open;
use tokio::*;
pub(crate) use crate::io::enums_structs_io::{DaneDoKompresji, FiltracjaPlików, KolejnośćDziałań, ProcesStatus, Progress, StrukturaFolderów};
use crate::io::enums_structs_io::KolejnośćDziałań::Qniec;
// use crate::io::import_for_compression::KolejnośćDziałań::Qniec;



async fn zgarnij_pliki(
    ścieżka: PathBuf,
    opcja: &StrukturaFolderów,
    filter: FiltracjaPlików,
    mut tx: mpsc::Sender<Progress> // Dodajemy kanał do raportowania "na bieżąco"
) -> Result<(Vec<(String, Vec<u8>)>, i32), tokio::io::Error> {
    let mut lista_plików = Vec::new();
    let mut licznik = 0;
    let mut foldery_do_przejrzenia = vec![ścieżka.clone()];
    let _ = tx.send(Progress::ZnalezionoPliki {
        etap: KolejnośćDziałań::ZbieraniePlików,
        pliki: None,
        status: ProcesStatus::Rozpoczęte,
    }).await;

    while let Some(aktualny_folder) = foldery_do_przejrzenia.pop() {
        let mut wejścia = tokio::fs::read_dir(aktualny_folder).await?;

        while let Some(entry) = wejścia.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                foldery_do_przejrzenia.push(path);
            } else if path.is_file() {
                // --- LOGIKA FILTROWANIA ---
                if !czy_plik_pasuje(&path, &filter) {
                    continue; // Pomijamy plik, jeśli nie pasuje do filtra
                }

                let nazwa = match opcja {
                    StrukturaFolderów::Tak => path.strip_prefix(&ścieżka).unwrap().to_string_lossy().into_owned(),
                    StrukturaFolderów::Nie => path.file_name().unwrap().to_string_lossy().into_owned(),
                };

                let tresc = tokio::fs::read(&path).await?;
                lista_plików.push((nazwa, tresc));
                licznik += 1;

                let _ = tx.send(Progress::ZnalezionoPliki {
                    etap: KolejnośćDziałań::ZbieraniePlików,
                    pliki: Some(licznik),
                    status: ProcesStatus::Wtrakcie,
                }).await;
            }
        }
    }
    let _ = tx.send(Progress::ZnalezionoPliki {
        etap: KolejnośćDziałań::ZbieraniePlików,
        pliki: None,
        status: ProcesStatus::Zakończone,
    }).await;
    Ok((lista_plików, licznik))
}
fn czy_plik_pasuje(path: &Path, filtr: &FiltracjaPlików) -> bool {
    match filtr {
        FiltracjaPlików::Wszystkie => true,
        _ => {
            // Pobieramy rozszerzenie i zamieniamy na małe litery (żeby .JPG i .jpg działały tak samo)
            let ext = path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            match filtr {
                FiltracjaPlików::Graficzne => matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg"),
                FiltracjaPlików::Audio => matches!(ext.as_str(), "mp3" | "wav" | "ogg" | "flac" | "m4a"),
                FiltracjaPlików::Tekstowe => matches!(ext.as_str(), "txt" | "md" | "json" | "cfg" | "ini" | "log"),
                FiltracjaPlików::Pdf => ext == "pdf",
                FiltracjaPlików::Wszystkie => true, // Obsłużone wyżej, ale kompilator wymaga kompletu
            }
        }
    }
}

async fn tworzenie_binarki(
    pliki: Vec<(String, Vec<u8>)>,
    ścieżka_wyjściowa: PathBuf,
    nazwa_pliku:String,
    mut tx: mpsc::Sender<Progress> // Dodajemy kanał tutaj
) -> Result<(), tokio::io::Error> {
    let _ = tx.send(Progress::Pakowanie {
        etap: KolejnośćDziałań::Binarka,
        aktualny: None,
        suma: None,
        status: ProcesStatus::Rozpoczęte,
    }).await;
    println!("Rozpoczęcie fn 2");
    let mut blobloblob = Vec::new();
    let suma = pliki.len() as i32;
    // 1. Ilość plików na start
    blobloblob.extend_from_slice(&(pliki.len() as u32).to_le_bytes());

    let mut licznik = 0;
    #[allow(clippy::explicit_counter_loop)]
    for (i, (nazwa, dane)) in pliki.into_iter().enumerate() {
        let n_bytes = nazwa.as_bytes();

        // Pakowanie metadanych i danych
        blobloblob.extend_from_slice(&(n_bytes.len() as u32).to_le_bytes());
        blobloblob.extend_from_slice(n_bytes);
        blobloblob.extend_from_slice(&(dane.len() as u64).to_le_bytes());
        blobloblob.extend_from_slice(&dane);

        // --- TUTAJ LICZENIE I WYSYŁKA ---
        licznik += 1;
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        // Wysyłamy aktualny numer pliku do Iced
        let _ = tx.send(Progress::Pakowanie {
            etap: KolejnośćDziałań::Binarka,
            aktualny: Some((i + 1) as i32),
            suma: Some(suma),
            status: ProcesStatus::Wtrakcie,
        }).await;
    }

    let _ = tx.send(Progress::Pakowanie {
        etap: KolejnośćDziałań::Binarka,
        aktualny: None,
        suma: None,
        status: ProcesStatus::IO,
    }).await;

    let ścieżka_temp = ścieżka_wyjściowa.join(format!("{}.jrz_tmp",nazwa_pliku));
    // tokio::fs::create_dir_all(&ścieżka_wyjściowa).await?;
    // println!("fn2 wyjściowa ścieżka pliku to: \n {:?}", ścieżka_temp);
    // tokio::fs::write(ścieżka_temp, blobloblob).await?;

    // DEBUG: Sprawdźmy czy folder nadrzędny istnieje
    if let Some(parent) = ścieżka_temp.parent() {
        println!("DEBUG: Czy folder nadrzędny {:?} istnieje? {}", parent, parent.exists());
        if !parent.exists() {
            println!("DEBUG: Próbuję stworzyć folder: {:?}", parent);
            tokio::fs::create_dir_all(parent).await?;
            println!("DEBUG: Po stworzeniu, czy istnieje? {}", parent.exists());
        }
    }



    println!("DEBUG: Próba zapisu {} bajtów do {:?}", blobloblob.len(), ścieżka_temp);

    // ZAPIS
    match tokio::fs::write(&ścieżka_temp, blobloblob).await {
        Ok(_) => println!("DEBUG: Zapis zakończony sukcesem!"),
        Err(e) => {
            println!("DEBUG: KATASTROFA przy zapisie: {}", e);
            return Err(e);
        }
    }
    let _ = tx.send(Progress::Pakowanie {
        etap: KolejnośćDziałań::Binarka,
        aktualny: None,
        suma: None,
        status: ProcesStatus::Zakończone,
    }).await;
    Ok(())
}




pub(crate) async fn ogarnianie_eksportu(zestaw_danych: DaneDoKompresji, mut tx: mpsc::Sender<Progress>,) -> Result<(), tokio::io::Error>{
    let start_czas = Instant::now();

    let wynik = async {
        let (in_path, out_path, poz_kompresji, strukturalnie, nazwa_pliku) = (zestaw_danych.ścieżka_in, zestaw_danych.ścieżka_out, zestaw_danych.kompresja, zestaw_danych.foldery, zestaw_danych.nazwa);


        let (wsio_dane, ilość_plików) = zgarnij_pliki(in_path, &strukturalnie, zestaw_danych.filtracja, tx.clone()).await?;
        // let _ = tx.send(ilość_plików).await;
        tworzenie_binarki(wsio_dane, out_path.clone(), nazwa_pliku.clone(), tx.clone()).await?;

        crate::kompresja::zetestede::kompresujsuj(out_path.clone(), nazwa_pliku.clone(), poz_kompresji, tx.clone()).await?;

        crate::szyfr::xor::szyfruj_xor(out_path, nazwa_pliku, tx.clone()).await?;
        Ok::<(), tokio::io::Error>(())
    }.await;

    match wynik {
        Ok(_) => {
            let trwanie = start_czas.elapsed(); // Zwraca strukturę Duration

            // Formatujemy czas na ładny napis, np. "1.23s" lub "45ms"
            // Możesz użyć prostego formatowania:
            let czas_napis = format!("{:.2?}", trwanie);
            let _ = tx.send(Progress::Zakonczono{ etap:Qniec, czas:czas_napis }).await;
            Ok(())
        },
        Err(e) => {
            // Jeśli cokolwiek powyżej sypnie błędem (przez znak zapytania),
            // wysyłamy opis błędu do UI zamiast po prostu "padać".
            let _ = tx.send(Progress::Błąd(e.to_string())).await;
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
//         let zestaw = DaneDoKompresji {
//             ścieżka_in: PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/for_compression/"), // Testujemy na własnym folderze src
//             ścieżka_out: PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/compressed/"),
//             kompresja: PoziomKompresji::Brak,
//             nazwa: "test_output".to_string(),
//             foldery: StrukturaFolderów::Tak,
//             filtracja: FiltracjaPlików::Wszystkie,
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
//         // 3. Odbieramy wiadomości z kanału (Stream API)
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