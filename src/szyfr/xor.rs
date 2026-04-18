use crate::no_sync::bardzo_tajne_haslo::BAŁDZOTAJNEHASŁO;
use std::path::PathBuf;
use iced::futures::channel::mpsc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use iced::futures::SinkExt;
use crate::io::enums_structs_io::{Progress, KolejnośćDziałań, ProcesStatus};

// Pamiętaj o zdefiniowaniu klucza gdzieś w stałych

pub async fn szyfruj_xor(
    ścieżka: PathBuf,
    nazwa: String,
    mut tx: mpsc::Sender<Progress>
) -> Result<(), tokio::io::Error> {
    let _ = tx.send(Progress::ProcesSzyfrowania {
        etap: KolejnośćDziałań::Szyfrowanie,
        procent:None,
        status: ProcesStatus::Rozpoczęte,
    }).await;
    println!("Rozpoczęcie fn szyfru");

    // 1. Przygotowanie ścieżek
    let sciezka_in = ścieżka.join(format!("{}.jrz", nazwa));     // Plik po kompresji
    let sciezka_out = ścieżka.join(format!("{}.jrzs", nazwa));   // Plik zaszyfrowany (finalny)

    // 2. Otwarcie plików i sprawdzenie rozmiaru
    let mut plik_in = File::open(&sciezka_in).await?;
    let metadata = plik_in.metadata().await?;
    let calkowity_rozmiar = metadata.len();

    let mut plik_out = File::create(&sciezka_out).await?;

    let klucz = BAŁDZOTAJNEHASŁO.as_bytes();
    let mut przeczytano_razem = 0u64;
    let mut bufor = vec![0u8; 128 * 1024]; // 128KB bufor
    let mut ostatni_procent = 0u8;

    // 3. Pętla szyfrowania
    while let Ok(n) = plik_in.read(&mut bufor).await {
        if n == 0 { break; } // Koniec pliku

        // Aplikujemy XOR na tym konkretnym kawałku (buforze)
        // Musimy wiedzieć, na którym bajcie całego pliku jesteśmy, 
        // aby zachować ciągłość klucza XOR (przeczytano_razem + i)
        for (i, bajt) in bufor[..n].iter_mut().enumerate() {
            let pozycja_w_pliku = przeczytano_razem + i as u64;
            *bajt ^= klucz[pozycja_w_pliku as usize % klucz.len()];
        }

        // Zapisujemy zaszyfrowany kawałek
        plik_out.write_all(&bufor[..n]).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        // Aktualizacja postępu
        przeczytano_razem += n as u64;
        let procent = ((przeczytano_razem as f64 / calkowity_rozmiar as f64) * 100.0) as u8;

        // Wysyłka do UI
        if procent > ostatni_procent {
            ostatni_procent = procent;

            let _ = tx.send(Progress::ProcesSzyfrowania {
                etap: KolejnośćDziałań::Szyfrowanie,
                procent:Some(procent),
                status: ProcesStatus::Wtrakcie,
            }).await;
        }
    }

    let _ = tx.send(Progress::ProcesSzyfrowania {
        etap: KolejnośćDziałań::Szyfrowanie,
        procent:None,
        status: ProcesStatus::IO,
    }).await;

    // 4. Flush i zamknięcie
    plik_out.flush().await?;

    // 5. Usuwanie pliku .jrz (pośredniego)
    tokio::fs::remove_file(sciezka_in).await?;
    let _ = tx.send(Progress::ProcesSzyfrowania {
        etap: KolejnośćDziałań::Szyfrowanie,
        procent:None,
        status: ProcesStatus::Zakończone,
    }).await;

    Ok(())
}