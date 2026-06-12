use enumy::dane_do_przetwarzania::DaneBinUnpak;
use enumy::fn_ogolne_przeliczeniowe::przelicz_czas;
use enumy::log_file_gen::generuj_plik_logow;
use enumy::send::wyslij_status;
use enumy::statusy::LogTxBinUnpak;
use iced::futures::channel::mpsc;
use kompresja::dekompresjazstd::dekompresujsuj;
use std::path::PathBuf;
use std::time::Instant;
use szyfrowanie::xor_de::deszyfruj_xor;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
/// # Checking if file is ok
///
/// mentioned file is *.jrz
///
/// and copying to destination folder in chunks
///
/// There's change in extension to *.jrz_temp_coded bcoz it's still coded tho
async fn sprawdzanie_istnienia_pliku(
    ścieżka_pliku: PathBuf,
    ścieżka_docelowa: PathBuf,
    mut tx: mpsc::Sender<LogTxBinUnpak>,
) -> Result<(), tokio::io::Error> {

// Walidacja wejścia
// ------------------------------------------------------------------------------------------
    if !ścieżka_pliku.is_file() {
        let msg = format!("[Binary unpacking checking] File not found: {:?}", ścieżka_pliku);
        generuj_plik_logow(msg.clone());
        wyslij_status(&mut tx,Some(LogTxBinUnpak::Błąd("Nie znaleziono pliku".to_string()))).await;
        return Err(tokio::io::Error::new(tokio::io::ErrorKind::NotFound, format!("File not found: {:?}",ścieżka_pliku)));
    }

    let nazwa_pliku = ścieżka_pliku.file_name().ok_or_else(|| {
        let msg = format!("[Binary unpacking checking] Wrong file name: {:?}", ścieżka_pliku.file_name());
        generuj_plik_logow(msg.clone());
        tokio::io::Error::new(tokio::io::ErrorKind::InvalidInput, msg)
    })?;
// ------------------------------------------------------------------------------------------

    let mut nowa_nazwa = nazwa_pliku.to_string_lossy().into_owned();
    nowa_nazwa.push_str("_temp_coded"); // Zmiana rozszerzenia_plików_zdjęciowych/nazwy

    let docelowy_plik_temp = ścieżka_docelowa.join(nowa_nazwa);

    // Upewnij się, że folder docelowy istnieje
    if !ścieżka_docelowa.exists() {
        tokio::fs::create_dir_all(&ścieżka_docelowa).await?;
    }

    // Kopiowanie z postępem
    let mut plik_in = tokio::fs::File::open(&ścieżka_pliku).await?;
    let mut plik_out = tokio::fs::File::create(&docelowy_plik_temp).await?;

    let total_size = plik_in.metadata().await?.len();
    let mut skopiowano = 0u64;
    let mut bufor = vec![0; 64 * 1024]; // Bufor 64KB
    let mut ostatni_stan = Instant::now();

//     while let Ok(n) = tokio::io::AsyncReadExt::read(&mut plik_in, &mut bufor).await {
//         if n == 0 {
//             break;
//         }
//         tokio::io::AsyncWriteExt::write_all(&mut plik_out, &bufor[..n]).await?;
//
//         skopiowano += n as u64;
//
//         if ostatni_stan.elapsed().as_millis() >= 250  {
//             ostatni_stan = Instant::now();
//             wyslij_status(&mut tx,Some(LogTxBinUnpak::Zbieranie{
//                 current: skopiowano,
//                 max: Some(total_size + 1),
//             })).await;
//         }
//     }
    loop {
        let n = tokio::io::AsyncReadExt::read(&mut plik_in, &mut bufor).await?;

        if n == 0 {
            break; // Koniec pliku - sukces
        }

        tokio::io::AsyncWriteExt::write_all(&mut plik_out, &bufor[..n]).await?;

        skopiowano += n as u64;

        // doing that way coz.. ya know... There's no sense of pushing msgs too often...
        // that's why u can see +1 in max so i cana confirm if everything went smooth even when loop
        // weren't in time window
        if ostatni_stan.elapsed().as_millis() >= 250  {
            ostatni_stan = Instant::now();
            wyslij_status(&mut tx, Some(LogTxBinUnpak::Zbieranie {
                current: skopiowano,
                max: Some(total_size + 1),
            })).await;
        }
    }

// +1 coz to be sure ;)
    wyslij_status(&mut tx,Some(LogTxBinUnpak::Zbieranie{
        current: skopiowano +1,
        max: Some(total_size + 1),
    })).await;

    Ok(())
}

/// # File extraction
/// We need path to decoded file folder ('ścieżka') and file name without extension ('nazwa_pliku'),
/// and ofc futures mpsc tx...
///
///
pub async fn wypakuj_pliki(
    ścieżka: PathBuf,
    nazwa_pliku: String,
    mut tx: mpsc::Sender<LogTxBinUnpak>,
) -> Result<(), tokio::io::Error> {
    
    let sciezka_binarki = ścieżka.join(format!("{}_temp_clean", nazwa_pliku));
    let mut plik = File::open(&sciezka_binarki).await?;

    let mut buf_u32 = [0u8; 4];
    let mut buf_u64 = [0u8; 8];
    let mut ostatni_stan = Instant::now();
    let mut liczydło = 0;

    plik.read_exact(&mut buf_u32).await?;
    let suma_plikow = u32::from_le_bytes(buf_u32);

    // [Optymalizacja 4] Jeden bufor na nazwy plików, wielokrotnego użytku
    let mut buf_nazwa = Vec::with_capacity(128);

    for i in 0..suma_plikow {
        // A. Czytamy długość nazwy
        plik.read_exact(&mut buf_u32).await?;
        let dlugosc_nazwy = u32::from_le_bytes(buf_u32) as usize;

        // B. Czytamy nazwę korzystając z tego samego bufora
        buf_nazwa.resize(dlugosc_nazwy, 0);
        plik.read_exact(&mut buf_nazwa).await?;
        let relatywna_sciezka = String::from_utf8_lossy(&buf_nazwa);

        // C. Czytamy rozmiar danych
        plik.read_exact(&mut buf_u64).await?;
        let rozmiar_danych = u64::from_le_bytes(buf_u64);

        // E. Odtwarzanie ścieżki i zabezpieczenie
        let pełna_ścieżka_wyjściowa = ścieżka.join(relatywna_sciezka.as_ref());

        // [Optymalizacja 2] Zabezpieczenie przed atakiem Zip Slip
        if !pełna_ścieżka_wyjściowa.starts_with(&ścieżka) {
            return Err(tokio::io::Error::new(
                tokio::io::ErrorKind::PermissionDenied,
                "Wanna some adrenaline, huh? Noticed unlawful try to save outside set catalog (Zip Slip)!",
            ));
        }

        if let Some(parent) = pełna_ścieżka_wyjściowa.parent() &&
            !parent.exists() {
                tokio::fs::create_dir_all(parent).await?;

        }

        // STRUMIENIOWANIE (Zero zagrożenia OOM)
        let mut plik_docelowy = File::create(&pełna_ścieżka_wyjściowa).await?;

        // .take(n) ogranicza czytanie z głównego pliku tylko do rozmiaru tego jednego assetu
        let mut adapter_rozmiaru = plik.take(rozmiar_danych);

        // Kopiujemy dane bezpośrednio z pliku do pliku w locie (zużycie RAMu: kilka kilobajtów)
        tokio::io::copy(&mut adapter_rozmiaru, &mut plik_docelowy).await?;

        // Odzyskujemy nasz główny plik spowrotem do dalszego czytania w pętli
        plik = adapter_rozmiaru.into_inner();

        liczydło = i + 1;

        if ostatni_stan.elapsed().as_millis() >= 250 {
            ostatni_stan = Instant::now();
            wyslij_status(&mut tx, Some(LogTxBinUnpak::Rozpakowywanie {
                current: liczydło,
                max: Some(suma_plikow + 2),
            })).await;
        }
    }

    wyslij_status(&mut tx, Some(LogTxBinUnpak::Rozpakowywanie {
        current: liczydło + 1,
        max: Some(suma_plikow + 2),
    })).await;

    drop(plik);
    tokio::fs::remove_file(sciezka_binarki).await?;

    wyslij_status(&mut tx, Some(LogTxBinUnpak::Rozpakowywanie {
        current: liczydło + 2,
        max: Some(suma_plikow + 2),
    })).await;

    Ok(())
}
/// # Main fn for unpacking binary file
/// everything should be smooth etc...
/// So...
///
/// Everything is happening inside 'let wynik'
/// 1. check data & copy
/// 2. decode file
/// 3. decompress file
/// 4. unpack files
pub async fn ogarnianie_dekompresji(
    dane: DaneBinUnpak,
    mut tx: mpsc::Sender<LogTxBinUnpak>,
) -> Result<(), tokio::io::Error> {

    let start_czas = Instant::now();
    let nazwa_pliku = dane
        .ścieżka_pliku
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let wynik = async {
        sprawdzanie_istnienia_pliku(
            dane.ścieżka_pliku,
            dane.ścieżka_docelowa.clone(),
            tx.clone(),
        )
        .await?;
        deszyfruj_xor(
            dane.ścieżka_docelowa.clone(),
            nazwa_pliku.clone(),
            tx.clone(),
        )
        .await?;
        dekompresujsuj(
            dane.ścieżka_docelowa.clone(),
            nazwa_pliku.clone(),
            tx.clone(),
        )
        .await?;
        wypakuj_pliki(dane.ścieżka_docelowa, nazwa_pliku, tx.clone()).await?;
        Ok::<(), tokio::io::Error>(())
    }
    .await;
    match wynik {
        Ok(_) => {
            wyslij_status(&mut tx,Some(LogTxBinUnpak::Finito(przelicz_czas(start_czas)))).await;
            Ok(())
        }
        Err(e) => {
            wyslij_status(&mut tx,Some(LogTxBinUnpak::Błąd(e.to_string()))).await;
            generuj_plik_logow(e.to_string());
            Err(e)
        }
    }
}
