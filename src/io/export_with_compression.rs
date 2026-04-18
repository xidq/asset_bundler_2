use std::path::PathBuf;
use std::time::Instant;
use iced::futures::channel::mpsc;
use iced::futures::SinkExt;

use tokio::*;
use tokio::io::AsyncReadExt;
pub(crate) use crate::io::enums_structs_io::{DaneDoDekompresji, KolejnośćDziałańDe, ProcesStatus, ProgressDe};
// pub(crate) use crate::io::import_for_compression::{KolejnośćDziałań, Progress};
use crate::kompresja::dekompresjazstd::dekompresujsuj;
use crate::szyfr::xor_de::deszyfruj_xor;




async fn sprawdzanie_istnienia_pliku(
    ścieżka_pliku: PathBuf,
    ścieżka_docelowa: PathBuf,
    mut tx: mpsc::Sender<ProgressDe>
) ->Result<(), tokio::io::Error>{
    let _ = tx.send(ProgressDe::Deszyfracja {
        etap: KolejnośćDziałańDe::SprawdzaniePliku,
        procent: None,
        status: ProcesStatus::Rozpoczęte
    }).await;

    // 1. Walidacja wejścia
    if !ścieżka_pliku.exists() || !ścieżka_pliku.is_file() {
        let błąd = format!("Nie znaleziono pliku: {:?}", ścieżka_pliku);
        let _ = tx.send(ProgressDe::BłądDe(błąd.clone())).await;
        return Err(tokio::io::Error::new(tokio::io::ErrorKind::NotFound, błąd));
    }

    // 2. Przygotowanie ścieżki tymczasowej (.jrzs -> .jrzs_temp)
    let nazwa_pliku = ścieżka_pliku.file_name()
        .ok_or_else(|| tokio::io::Error::new(tokio::io::ErrorKind::InvalidInput, "Błędna nazwa pliku"))?;

    let mut nowa_nazwa = nazwa_pliku.to_string_lossy().into_owned();
    nowa_nazwa.push_str("_temp_coded"); // Zmiana rozszerzenia/nazwy

    let docelowy_plik_temp = ścieżka_docelowa.join(nowa_nazwa);

    // Upewnij się, że folder docelowy istnieje
    if !ścieżka_docelowa.exists() {
        tokio::fs::create_dir_all(&ścieżka_docelowa).await?;
    }

    // 3. Kopiowanie z postępem
    let mut plik_in = tokio::fs::File::open(&ścieżka_pliku).await?;
    let mut plik_out = tokio::fs::File::create(&docelowy_plik_temp).await?;

    let total_size = plik_in.metadata().await?.len();
    let mut skopiowano = 0u64;
    let mut bufor = vec![0; 64 * 1024]; // Bufor 64KB
    let mut ostatni_procent = 0u8;

    println!("Rozpoczęto kopiowanie do: {:?}", docelowy_plik_temp);

    while let Ok(n) = tokio::io::AsyncReadExt::read(&mut plik_in, &mut bufor).await {
        if n == 0 { break; }
        tokio::io::AsyncWriteExt::write_all(&mut plik_out, &bufor[..n]).await?;

        skopiowano += n as u64;
        let procent = ((skopiowano as f64 / total_size as f64) * 100.0) as u8;

        // Wysyłaj postęp tylko gdy procent się zmieni (żeby nie zapchać kanału)
        if procent > ostatni_procent {
            ostatni_procent = procent;
            let _ = tx.send(ProgressDe::Deszyfracja {
                etap: KolejnośćDziałańDe::SprawdzaniePliku,
                procent: Some(procent),
                status: ProcesStatus::Wtrakcie
            }).await;
        }
    }

    let _ = tx.send(ProgressDe::Deszyfracja {
        etap: KolejnośćDziałańDe::SprawdzaniePliku,
        procent: None,
        status: ProcesStatus::Zakończone
    }).await;

    println!("Kopiowanie zakończone sukcesem.");

    // Zwracamy ścieżkę do nowego pliku, żeby kolejna funkcja wiedziała co deszyfrować
    Ok(())
}

pub async fn wypakuj_pliki(
    ścieżka: PathBuf,
    nazwa_pliku: String,
    mut tx: mpsc::Sender<ProgressDe>
) -> Result<(), tokio::io::Error> {

    let _ = tx.send(ProgressDe::RozpakowywanieDe {
        etap: KolejnośćDziałańDe::Rozpakowywanie,
        aktualny: None,
        suma: None,
        status: ProcesStatus::Rozpoczęte
    }).await;
    println!("Rozpoczęcie rozpakowywania binarki: {}_temp_clean", nazwa_pliku);

    // 1. Ścieżka do surowej binarki (wynik dekompresji)
    let sciezka_binarki = ścieżka.join(format!("{}_temp_clean", nazwa_pliku));

    // 2. Otwieramy plik do czytania przez Tokio
    let mut plik = tokio::fs::File::open(&sciezka_binarki).await?;

    // Bufory pomocnicze na liczby
    let mut buf_u32 = [0u8; 4];
    let mut buf_u64 = [0u8; 8];

    // 3. Czytamy całkowitą ilość plików zapisaną na samym początku (u32)
    plik.read_exact(&mut buf_u32).await?;
    let suma_plikow = u32::from_le_bytes(buf_u32);

    println!("Do wypakowania: {} plików", suma_plikow);

    // 4. Pętla przetwarzająca każdy zapisany plik
    for i in 0..suma_plikow {
        // A. Czytamy długość nazwy (u32)
        plik.read_exact(&mut buf_u32).await?;
        let dlugosc_nazwy = u32::from_le_bytes(buf_u32) as usize;

        // B. Czytamy nazwę (String)
        let mut buf_nazwa = vec![0u8; dlugosc_nazwy];
        plik.read_exact(&mut buf_nazwa).await?;
        let relatywna_sciezka = String::from_utf8_lossy(&buf_nazwa).into_owned();

        // C. Czytamy rozmiar danych (u64)
        plik.read_exact(&mut buf_u64).await?;
        let rozmiar_danych = u64::from_le_bytes(buf_u64);

        // D. Czytamy same dane pliku (Vec<u8>)
        let mut dane_pliku = vec![0u8; rozmiar_danych as usize];
        plik.read_exact(&mut dane_pliku).await?;

        // E. Odtwarzanie ścieżki i zapis na dysk
        let pełna_ścieżka_wyjściowa = ścieżka.join(&relatywna_sciezka);

        // Tworzymy strukturę folderów, jeśli plik był w podkatalogu
        if let Some(parent) = pełna_ścieżka_wyjściowa.parent() &&
            !parent.exists() {
                tokio::fs::create_dir_all(parent).await?;

        }

        // Zapisujemy plik wynikowy
        tokio::fs::write(&pełna_ścieżka_wyjściowa, dane_pliku).await?;

        // 5. Powiadomienie UI - używamy nowego formatu RozpakowywanieDe
        let _ = tx.send(ProgressDe::RozpakowywanieDe {
            etap: KolejnośćDziałańDe::Rozpakowywanie,
            aktualny: Some((i + 1) as i32),
            suma: Some(suma_plikow as i32),
            status: ProcesStatus::Wtrakcie
        }).await;
    }
    let _ = tx.send(ProgressDe::RozpakowywanieDe {
        etap: KolejnośćDziałańDe::Rozpakowywanie,
        aktualny: None,
        suma: None,
        status: ProcesStatus::IO
    }).await;

    // 6. Czyszczenie - zamykamy i usuwamy plik tymczasowy .jrz_temp_clean
    drop(plik);
    tokio::fs::remove_file(sciezka_binarki).await?;

    println!("Wszystkie pliki zostały poprawnie wypakowane do {:?}", ścieżka);
    let _ = tx.send(ProgressDe::RozpakowywanieDe {
        etap: KolejnośćDziałańDe::Rozpakowywanie,
        aktualny: None,
        suma: None,
        status: ProcesStatus::Zakończone
    }).await;
    Ok(())
}

pub async fn ogarnianie_dekompresji(
    dane:DaneDoDekompresji,
    mut tx: mpsc::Sender<ProgressDe>,
)-> Result<(), tokio::io::Error>{
    let start_czas = Instant::now();
    let nazwa_pliku = dane.ścieżka_pliku.file_name().unwrap().to_string_lossy().into_owned();
    // let ścieżka_kastrat = dane.ścieżka_docelowa
    println!("ścieżka docelowa: \n {:?}", &dane.ścieżka_docelowa);
    println!("nazwa pliku: {}", nazwa_pliku);
    let wynik = async {
        sprawdzanie_istnienia_pliku(dane.ścieżka_pliku,dane.ścieżka_docelowa.clone(), tx.clone()).await?;
        deszyfruj_xor(dane.ścieżka_docelowa.clone(), nazwa_pliku.clone(), tx.clone()).await?;
        dekompresujsuj(dane.ścieżka_docelowa.clone(), nazwa_pliku.clone(), tx.clone()).await?;
        wypakuj_pliki(dane.ścieżka_docelowa, nazwa_pliku, tx.clone()).await?;
        Ok::<(), tokio::io::Error>(())
    }.await;
    match wynik {
        Ok(_) => {
            let trwanie = start_czas.elapsed(); // Zwraca strukturę Duration

            // Formatujemy czas na ładny napis, np. "1.23s" lub "45ms"
            // Możesz użyć prostego formatowania:
            let czas_napis = format!("{:.2?}", trwanie);
            let _ = tx.send(ProgressDe::ZakonczonoDe{ etap:KolejnośćDziałańDe::Qniec, czas:czas_napis }).await;
            Ok(())
        },
        Err(e) => {
            // Jeśli cokolwiek powyżej sypnie błędem (przez znak zapytania),
            // wysyłamy opis błędu do UI zamiast po prostu "padać".
            let _ = tx.send(ProgressDe::BłądDe(e.to_string())).await;
            Err(e)
        }
    }
}