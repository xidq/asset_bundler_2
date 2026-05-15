pub mod dds_export;
pub mod dds_halper;
pub mod dds_import;
pub mod dds_wczytywanie_zdjec;
pub mod zapisy;
mod strukt;
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::dds_import::export_dds_array_to_jpg;
//     use crate::dds_wczytywanie_zdjec::{dds_ogarnij_ze_zdjec_do_paczki};
//     use enumy::dane_do_przetwarzania::DaneDoPakowaniaDds;
//     use enumy::opcje::{OptFormatDds, OptFormatyKoloruObrazOgólny, OptKompresjaDds, OptRozszerzeniaPlikówZdjęciowych};
//     use enumy::statusy::{LogTxDoPakowanieDds, LogTxDoRozpakowanieDds};
//     use futures::StreamExt;
//     use futures::channel::mpsc;
//     use std::path::PathBuf;
//     const ŚCIEŻKAZPLIKAMI: &str = "../../src/Test/dds_test/pliki_org";
//     const ŚCIEŻKAWYJŚCIOWA: &str = "../../src/Test/dds_test/pliki_spakowane";
//     const ŚCIEŻKADEKOMPILOWANIA: &str = "../../src/Test/dds_test/pliki_rozpakowane";
//     const NAZWAPLIKU: &str = "umcyumcy";
//     #[tokio::test]
//     async fn testowanie_czy_zapis_wielu() {
//         println!("ścieżka wejściowa: \n{}", ŚCIEŻKAZPLIKAMI);
//         println!("ścieżka wejściowa: \n{}", ŚCIEŻKAZPLIKAMI);
//         let ustawienia: DaneDoPakowaniaDds = DaneDoPakowaniaDds {
//             ścieżka_wejściowa: PathBuf::from(ŚCIEŻKAZPLIKAMI),
//             ścieżka_wyjściowa: PathBuf::from(ŚCIEŻKAWYJŚCIOWA),
//             nazwa: NAZWAPLIKU.to_string(),
//             format: OptFormatDds::DxgiFormatBc7Unorm,
//             kompresja: OptKompresjaDds::Fast,
//         };
//         let (tx, mut rx) = mpsc::channel::<LogTxDoPakowanieDds>(100);
// 
//         let handle = tokio::spawn(async move { dds_ogarnij_ze_zdjec_do_paczki(ustawienia, tx).await });
// 
//         let mut odebrano_start = false;
//         let mut odebrano_koniec = false;
// 
//         while let Some(postep) = rx.next().await {
//             match postep {
//                 LogTxDoPakowanieDds::StatusPakowanieDdsStart => {
//                     odebrano_start = true;
//                     println!("Start")
//                 }
//                 LogTxDoPakowanieDds::StatusPakowanieDdsWtrakcie(p) => println!("Postęp: {}%", p),
//                 LogTxDoPakowanieDds::StatusPakowanieDdsKoniec(s) => {
//                     odebrano_koniec = true;
//                     println!("Koniec: {}", s);
//                 }
//                 LogTxDoPakowanieDds::StatusPakowanieDdsBłąd(e) => panic!("Błąd: {}", e),
//             }
//         }
// 
//         let _ = handle.await.expect("Task failed");
// 
//         assert!(odebrano_start, "Nie otrzymano sygnału Start");
//         assert!(odebrano_koniec, "Nie otrzymano sygnału Koniec");
//     }
//     #[tokio::test]
//     async fn rozpakowanko() {
//         let (tx, mut rx) = mpsc::channel::<LogTxDoRozpakowanieDds>(100);
//         let ścieżka_z_plikiem = PathBuf::from(format!("{}/{}.dds", ŚCIEŻKAWYJŚCIOWA, NAZWAPLIKU));
//         let handle = tokio::spawn(async move {
//             export_dds_array_to_jpg(
//                 ścieżka_z_plikiem,
//                 PathBuf::from(ŚCIEŻKADEKOMPILOWANIA),
//                 OptRozszerzeniaPlikówZdjęciowych::Jpg {
//                     jakosc: 90,
//                     progresywny: false,
//                     bit_depth: vec![OptFormatyKoloruObrazOgólny::B8],
//                 },
//                 tx.clone(),
//             )
//             .await
//         });
//         while let Some(postep) = rx.next().await {
//             match postep {
//                 LogTxDoRozpakowanieDds::StatusRozpakowanieDdsStart => {
//                     println!("Start")
//                 }
//                 LogTxDoRozpakowanieDds::StatusRozpakowanieDdsWtrakcie(p) => {
//                     println!("Postęp: {}%", p)
//                 }
//                 LogTxDoRozpakowanieDds::StatusRozpakowanieDdsKoniec(s) => {
//                     println!("Koniec: {}", s);
//                 }
//                 LogTxDoRozpakowanieDds::StatusRozpakowanieDdsBłąd(e) => panic!("Błąd: {}", e),
//             }
//         }
// 
//         let _ = handle.await.expect("Task failed");
//     }
// }
