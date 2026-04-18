// // #[cfg(test)]
// // mod tests {
// //     use std::path::PathBuf;
// //     use super::*;
// //     // Używamy kanałów z futures, bo takie przyjmuje Twoja funkcja
// //     use iced::futures::channel::mpsc;
// //     use iced::futures::StreamExt;
// //     use crate::io::import_for_compression::{ogarnianie_eksportu, DaneDoKompresji, PoziomKompresji, Progress, StrukturaFolderów};
// //     // Potrzebne dla metody .next()
// //
// //     #[tokio::test]
// //     async fn test_ogarniania_eksportu_podstawowy() {
// //         // 1. Setup
// //         let zestaw = DaneDoKompresji {
// //             ścieżka_in: PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/for_compression/"), // Testujemy na własnym folderze src
// //             ścieżka_out: PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/compressed/"),
// //             kompresja: PoziomKompresji::Brak,
// //             nazwa: "test_output".to_string(),
// //             foldery: StrukturaFolderów::Tak,
// //         };
// //         let sciezka = PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/for_compression/");
// //         println!("Czy widzę folder wejściowy? {}", sciezka.exists());
// //         assert!(sciezka.exists(), "FOLDER NIE ISTNIEJE Z PERSPEKTYWY TESTU: {:?}", sciezka);
// //         let sciezka2 = PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/compressed/");
// //         println!("Czy widzę folder wyjściowy? {}", sciezka2.exists());
// //         assert!(sciezka2.exists(), "FOLDER NIE ISTNIEJE Z PERSPEKTYWY TESTU: {:?}", sciezka2);
// //         // Tworzymy kanał z biblioteki futures (ten sam co w Iced)
// //         let (tx, mut rx) = mpsc::channel::<Progress>(100);
// //
// //         // 2. Odpalamy funkcję w tle
// //         let handle = tokio::spawn(async move {
// //             ogarnianie_eksportu(zestaw, tx).await
// //         });
// //
// //         // 3. Odbieramy wiadomości z kanału (Stream API)
// //         // .next() zwraca Option<Progress>
// //         // Odbieraj wszystko, co leci kanałem, aż do zamknięcia lub sukcesu
// //         let mut czy_zakonczono = false;
// //         while let Some(msg) = rx.next().await {
// //             match msg {
// //                 Progress::ZnalezionoPliki { pliki, .. } => {
// //                     println!("Sukces: Znaleziono {} plików", pliki);
// //                 },
// //                 Progress::Proces { etap, procent } => {
// //                     println!("Postęp: {:?} - {}%", etap, procent);
// //                 },
// //                 Progress::Zakonczono { czas, .. } => {
// //                     println!("Eksport zakończony w czasie: {}", czas);
// //                     czy_zakonczono = true;
// //                     break; // Wychodzimy z pętli, bo to koniec
// //                 },
// //                 Progress::Błąd(e) => {
// //                     panic!("Wystąpił błąd w trakcie procesu: {}", e);
// //                 },
// //                 _ => {} // Resztę (np. pakowanie) ignorujemy lub dopisujemy
// //             }
// //         }
// //
// //         assert!(czy_zakonczono, "Test zakończył się bez otrzymania komunikatu Zakonczono!");
// //
// //         // Na końcu sprawdzamy wynik samej funkcji (handle)
// //         let wynik = handle.await.unwrap();
// //         assert!(wynik.is_ok());
// //     }
// // }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashMap;
//     use std::path::PathBuf;
//     use tokio::fs;
//     use iced::futures::channel::mpsc;
//     use iced::futures::StreamExt;
//     use crate::io::enums_structs_io::PoziomKompresji;
//     use crate::io::import_for_compression::{ogarnianie_eksportu, DaneDoKompresji, FiltracjaPlików, Progress, StrukturaFolderów};
//     use crate::io::export_with_compression::{ogarnianie_dekompresji, DaneDoDekompresji, ProgressDe,};
//
//     // Pomocnicza funkcja do zbierania skrótów plików (rozmiar + bity), by porównać czy są identyczne
//     async fn pobierz_mape_plikow(path: &PathBuf) -> HashMap<PathBuf, Vec<u8>> {
//         let mut mapa = HashMap::new();
//         let mut stack = vec![path.clone()];
//
//         while let Some(current_dir) = stack.pop() {
//             let mut entries = fs::read_dir(current_dir).await.unwrap();
//             while let Some(entry) = entries.next_entry().await.unwrap() {
//                 let p = entry.path();
//                 if p.is_dir() {
//                     stack.push(p);
//                 } else {
//                     let content = fs::read(&p).await.unwrap();
//                     // Kluczem jest ścieżka relatywna, żeby móc porównać dwa różne foldery
//                     let rel_path = p.strip_prefix(path).unwrap().to_path_buf();
//                     mapa.insert(rel_path, content);
//                 }
//             }
//         }
//         mapa
//     }
//         #[tokio::test]
//         async fn test_ogarniania_eksportu_podstawowy() {
//             // 1. Setup
//             let zestaw = DaneDoKompresji {
//                 ścieżka_in: PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/for_compression/"), // Testujemy na własnym folderze src
//                 ścieżka_out: PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/compressed/"),
//                 kompresja: PoziomKompresji::Brak,
//                 nazwa: "test_output".to_string(),
//                 foldery: StrukturaFolderów::Tak,
//                 filtracja: FiltracjaPlików::Wszystkie,
//                 filtracja_opcje: vec![],
//                 filtracja_state: Default::default(),
//                 kompresja_opcje: vec![],
//                 kompresja_state: Default::default(),
//             };
//             let sciezka = PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/for_compression/");
//             println!("Czy widzę folder wejściowy? {}", sciezka.exists());
//             assert!(sciezka.exists(), "FOLDER NIE ISTNIEJE Z PERSPEKTYWY TESTU: {:?}", sciezka);
//             let sciezka2 = PathBuf::from("/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/compressed/");
//             println!("Czy widzę folder wyjściowy? {}", sciezka2.exists());
//             assert!(sciezka2.exists(), "FOLDER NIE ISTNIEJE Z PERSPEKTYWY TESTU: {:?}", sciezka2);
//             // Tworzymy kanał z biblioteki futures (ten sam co w Iced)
//             let (tx, mut rx) = mpsc::channel::<Progress>(100);
//
//             // 2. Odpalamy funkcję w tle
//             let handle = tokio::spawn(async move {
//                 ogarnianie_eksportu(zestaw, tx).await
//             });
//
//             // 3. Odbieramy wiadomości z kanału (Stream API)
//             // .next() zwraca Option<Progress>
//             // Odbieraj wszystko, co leci kanałem, aż do zamknięcia lub sukcesu
//             let mut czy_zakonczono = false;
//             while let Some(msg) = rx.next().await {
//                 match msg {
//                     Progress::ZnalezionoPliki { pliki, .. } => {
//                         println!("Sukces: Znaleziono {:?} plików", pliki);
//                     },
//                     Progress::Pakowanie { etap, aktualny, suma, status } => {
//                         println!("Postęp: {:?} - {:?}/{:?} ", etap, aktualny, suma);
//                     },
//                     Progress::Zakonczono { czas, .. } => {
//                         println!("Eksport zakończony w czasie: {}", czas);
//                         czy_zakonczono = true;
//                         break; // Wychodzimy z pętli, bo to koniec
//                     },
//                     Progress::Błąd(e) => {
//                         panic!("Wystąpił błąd w trakcie procesu: {}", e);
//                     },
//                     _ => {} // Resztę (np. pakowanie) ignorujemy lub dopisujemy
//                 }
//             }
//
//             assert!(czy_zakonczono, "Test zakończył się bez otrzymania komunikatu Zakonczono!");
//
//             // Na końcu sprawdzamy wynik samej funkcji (handle)
//             let wynik = handle.await.unwrap();
//             assert!(wynik.is_ok());
//         }
//     #[tokio::test]
//     async fn test_pelnego_cyklu_bit_po_bicie() {
//         // 1. Definicja ścieżek
//         let root = "/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/";
//         let folder_oryginalny = PathBuf::from(format!("{}for_compression/", root));
//         let folder_skompresowany = PathBuf::from(format!("{}compressed/", root));
//         let folder_wypakowany = PathBuf::from(format!("{}decompressed_final/", root));
//
//         // Czyszczenie folderu docelowego przed testem
//         if folder_wypakowany.exists() {
//             fs::remove_dir_all(&folder_wypakowany).await.unwrap();
//         }
//         fs::create_dir_all(&folder_wypakowany).await.unwrap();
//
//         // 2. EKSPORT (Pakowanie)
//         let zestaw_eksport = DaneDoKompresji {
//             ścieżka_in: folder_oryginalny.clone(),
//             ścieżka_out: folder_skompresowany.clone(),
//             kompresja: PoziomKompresji::Brak,
//             nazwa: "cykl_testowy".to_string(),
//             foldery: StrukturaFolderów::Tak,
//             filtracja: FiltracjaPlików::Wszystkie,
//             filtracja_opcje: vec![],
//             filtracja_state: Default::default(),
//             kompresja_opcje: vec![],
//             kompresja_state: Default::default(),
//         };
//         let (tx_e, _rx_e) = mpsc::channel::<Progress>(100);
//         ogarnianie_eksportu(zestaw_eksport, tx_e).await.expect("Eksport zawiódł");
//
//         // 3. IMPORT (Rozpakowywanie)
//         let zestaw_import = DaneDoDekompresji {
//             ścieżka_pliku: folder_skompresowany.join("cykl_testowy.jrzs"),
//             ścieżka_docelowa: folder_wypakowany.clone(),
//         };
//         let (tx_i, mut rx_i) = mpsc::channel::<ProgressDe>(100);
//
//         // Odpalamy dekompresję i czekamy na zakończenie komunikacji
//         let import_handle = tokio::spawn(async move {
//             ogarnianie_dekompresji(zestaw_import, tx_i).await
//         });
//
//         while let Some(msg) = rx_i.next().await {
//             if let ProgressDe::ZakonczonoDe { .. } = msg { break; }
//             if let ProgressDe::BłądDe(e) = msg { panic!("Błąd importu: {}", e); }
//         }
//         import_handle.await.unwrap().expect("Import zawiódł");
//
//         // 4. PORÓWNANIE (Weryfikacja bitowa)
//         let pliki_oryginalne = pobierz_mape_plikow(&folder_oryginalny).await;
//         let pliki_odzyskane = pobierz_mape_plikow(&folder_wypakowany).await;
//
//         assert_eq!(pliki_oryginalne.len(), pliki_odzyskane.len(), "Ilość plików się nie zgadza!");
//
//         for (sciezka, zawartosc_orig) in pliki_oryginalne {
//             let zawartosc_nowa = pliki_odzyskane.get(&sciezka)
//                 .expect(&format!("Brakuje pliku: {:?}", sciezka));
//
//             assert_eq!(&zawartosc_orig, zawartosc_nowa, "BITY SIĘ NIE ZGADZAJĄ w pliku: {:?}", sciezka);
//         }
//         println!("SUKCES: Wszystkie pliki są identyczne bit po bicie!");
//     }
//
//     #[tokio::test]
//     async fn test_ogarniania_importu_podstawowy() {
//         let root = "/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/";
//         let zestaw = DaneDoDekompresji {
//             ścieżka_pliku: PathBuf::from(format!("{}compressed/test_output.jrzs", root)),
//             ścieżka_docelowa: PathBuf::from(format!("{}decompressed_quick/", root)),
//         };
//
//         if !zestaw.ścieżka_docelowa.exists() {
//             fs::create_dir_all(&zestaw.ścieżka_docelowa).await.unwrap();
//         }
//
//         let (tx, mut rx) = mpsc::channel::<ProgressDe>(100);
//
//         let handle = tokio::spawn(async move {
//             ogarnianie_dekompresji(zestaw, tx).await
//         });
//
//         let mut sukces = false;
//         while let Some(msg) = rx.next().await {
//             match msg {
//                 ProgressDe::ZnalezionoPlikDe { etap, .. } => println!("Etap: {:?} - Znaleziono bazowy plik", etap),
//                 ProgressDe::Dekompresja { pamięć, .. } => println!("Dekompresja: {:?} bajtów", pamięć),
//                 ProgressDe::RozpakowywanieDe { aktualny, suma, .. } => {
//                     println!("Rozpakowywanie: {:?}/{:?}", aktualny, suma);
//                 },
//                 ProgressDe::ZakonczonoDe { czas, .. } => {
//                     println!("Import zakończony w: {}", czas);
//                     sukces = true;
//                 },
//                 ProgressDe::BłądDe(e) => panic!("Błąd: {}", e),
//                 _ => {}
//             }
//         }
//
//         assert!(sukces);
//         handle.await.unwrap().expect("Funkcja zwróciła Err");
//     }
//     #[tokio::test]
//     async fn test_weryfikacji_miedzy_folderami() {
//         // 1. Definicja ścieżek (korzystamy z tych samych co w Twoich testach 1 i 3)
//         let root = "/home/xidq/RustroverProjects/asset_bundler_2/src/testfiles/";
//         let folder_pierwotny = PathBuf::from(format!("{}for_compression/", root));
//         let folder_koncowy = PathBuf::from(format!("{}decompressed_quick/", root));
//
//         // Sprawdzenie czy foldery w ogóle istnieją przed porównaniem
//         assert!(folder_pierwotny.exists(), "Błąd: Folder pierwotny nie istnieje: {:?}", folder_pierwotny);
//         assert!(folder_koncowy.exists(), "Błąd: Folder końcowy nie istnieje. Czy test 3 został uruchomiony?: {:?}", folder_koncowy);
//
//         println!("Rozpoczynam bitowe porównywanie folderów...");
//         println!("Źródło: {:?}", folder_pierwotny);
//         println!("Cel:    {:?}", folder_koncowy);
//
//         // 2. Pobieramy mapy plików (relatywna ścieżka -> zawartość bajtowa)
//         let pliki_orig = pobierz_mape_plikow(&folder_pierwotny).await;
//         let pliki_wynik = pobierz_mape_plikow(&folder_koncowy).await;
//
//         // 3. Weryfikacja ilości plików
//         println!("Liczba plików w oryginale: {}", pliki_orig.len());
//         println!("Liczba plików wypakowanych: {}", pliki_wynik.len());
//
//         assert_eq!(
//             pliki_orig.len(),
//             pliki_wynik.len(),
//             "NIEZGODNOŚĆ: Liczba plików w folderze końcowym jest inna niż w oryginalnym!"
//         );
//
//         // 4. Szczegółowe porównanie zawartości każdego pliku
//         let mut licznik_ok = 0;
//
//         for (sciezka, bajty_orig) in pliki_orig {
//             // Sprawdzamy czy plik o takiej samej ścieżce relatywnej istnieje w celu
//             let bajty_wynik = pliki_wynik.get(&sciezka).expect(&format!(
//                 "BŁĄD: Plik {:?} istnieje w oryginale, ale nie został odnaleziony w folderze końcowym!",
//                 sciezka
//             ));
//
//             // Porównanie zawartości
//             assert_eq!(
//                 &bajty_orig,
//                 bajty_wynik,
//                 "BŁĄD KRYTYCZNY: Zawartość pliku {:?} uległa zmianie (bity się nie zgadzają)!",
//                 sciezka
//             );
//
//             licznik_ok += 1;
//             if licznik_ok % 10 == 0 || pliki_wynik.len() < 10 {
//                 println!("Zweryfikowano poprawnie: {:?} ({} bajtów)", sciezka, bajty_orig.len());
//             }
//         }
//
//         println!("--- WYNIK WERYFIKACJI ---");
//         println!("Sukces! Wszystkie {} pliki są identyczne bit po bicie.", licznik_ok);
//         println!("Struktura folderów została zachowana.");
//     }
// }