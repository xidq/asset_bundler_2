use iced::futures::channel::mpsc;
use std::io::Write;
use std::path::PathBuf;
// use iced::futures::channel::mpsc;
use enumy::opcje::OptKompresjaPlikówPoziomKompresjiZstd;
use enumy::statusy::LogTxDoKompresjiPliku;
use iced::futures::SinkExt;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use zstd::*;

pub async fn kompresujsuj(
    ścieżka_pliku: PathBuf,
    nazwa_pliku: String,
    poziom_kompresji: OptKompresjaPlikówPoziomKompresjiZstd,
    mut tx: mpsc::Sender<LogTxDoKompresjiPliku>, // Dodajemy kanał tutaj
) -> Result<(), tokio::io::Error> {
    let _ = tx
        .send(LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesKompresji { procent: None })
        .await;

    println!("Rozpoczęcie fn kompresji");
    // 1. Ścieżki: .jrz_temp jako wejście, .jrz jako wyjście
    let sciezka_temp = ścieżka_pliku.join(format!("{}.jrz_tmp", nazwa_pliku));
    let sciezka_final = ścieżka_pliku.join(format!("{}.jrz", nazwa_pliku));
    // DEBUG: Sprawdźmy fizycznie, czy plik tam jest sekundę po zakończeniu poprzedniej funkcji
    println!("DEBUG KOMPRESJA: Szukam pliku: {:?}", sciezka_temp);
    println!(
        "DEBUG KOMPRESJA: Czy plik istnieje? {}",
        sciezka_temp.exists()
    );

    // 2. Otwieramy plik temp do odczytu
    let mut plik_in = File::open(&sciezka_temp).await?;
    println!("plik otwarty do odczytu w kompresji");
    let metadata = plik_in.metadata().await?;
    let calkowity_rozmiar = metadata.len();

    // 3. Ustawienie poziomu ZSTD
    let poziom = match poziom_kompresji {
        OptKompresjaPlikówPoziomKompresjiZstd::Brak => 0,
        OptKompresjaPlikówPoziomKompresjiZstd::Standard => 3,
        OptKompresjaPlikówPoziomKompresjiZstd::Duża => 10,
    };

    // Encoder zstd (tworzy plik .jrz)
    let plik_out = std::fs::File::create(&sciezka_final)?;
    println!("tworzony plik jrz (bez temp)");
    let mut encoder = Encoder::new(plik_out, poziom)?;

    // 4. Pętla przetwarzania (Kompresja)
    let mut bufor = vec![0u8; 128 * 1024]; // 128KB bufor
    let mut przeczytano_razem = 0u64;
    let mut ostatni_procent = 0u8;

    while let Ok(n) = plik_in.read(&mut bufor).await {
        if n == 0 {
            break;
        } // Koniec danych

        // Zapis do kompresora
        encoder.write_all(&bufor[..n])?;

        // Obliczanie postępu
        przeczytano_razem += n as u64;
        let procent = ((przeczytano_razem as f64 / calkowity_rozmiar as f64) * 100.0) as u8;
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        if procent > ostatni_procent {
            ostatni_procent = procent;
            // WYSYŁKA DO UI (używamy Twojego Progress::Proces)
            let _ = tx
                .send(
                    LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesKompresji {
                        procent: Some(procent),
                    },
                )
                .await;
        }
    }

    // Zamknięcie streama zstd (ważne!)
    encoder.finish()?;
    let _ = tx
        .send(LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesKompresji { procent: None })
        .await;

    // 5. Usuwamy tymczasowy plik .jrz_temp
    // Po tej operacji na dysku zostaje tylko gotowy plik .jrz
    tokio::fs::remove_file(sciezka_temp).await?;
    let _ = tx
        .send(LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesKompresji { procent: None })
        .await;

    // NIE wysyłamy tutaj Zakonczono, bo potem pewnie leci szyfrowanie w ogarnianie_eksportu
    Ok(())
}
