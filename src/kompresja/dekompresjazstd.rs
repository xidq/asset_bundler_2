use std::io::Write;
use std::path::PathBuf;
use iced::futures::channel::mpsc;
// use iced::futures::channel::mpsc;
use iced::futures::SinkExt;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use zstd::*;
use crate::io::enums_structs_io::ProcesStatus;
use crate::io::export_with_compression::{KolejnośćDziałańDe, ProgressDe};

pub async fn dekompresujsuj(
    ścieżka_pliku: PathBuf,
    nazwa_pliku: String,
    mut tx: mpsc::Sender<ProgressDe>
) -> Result<(), tokio::io::Error> {

    let _ = tx.send(ProgressDe::Dekompresja {
        etap: KolejnośćDziałańDe::Dekompresja,
        pamięć: None,
        status: ProcesStatus::Rozpoczęte
    }).await;
    println!("Rozpoczęcie fn dekompresji");

    // 1. Ścieżki: .jrz_temp (wejście po XOR) -> .jrz_temp_clean (surowa binarka)
    let sciezka_in = ścieżka_pliku.join(format!("{}_temp", nazwa_pliku));
    let sciezka_out = ścieżka_pliku.join(format!("{}_temp_clean", nazwa_pliku));

    // 2. Otwieramy plik skompresowany
    println!("sciezka_in: \n{:?}", sciezka_in);
    println!("sciezka_out: \n{:?}", sciezka_out);
    let mut plik_in = File::open(&sciezka_in).await?;
    let metadata = plik_in.metadata().await?;
    let rozmiar_skompresowany = metadata.len();

    // 3. Decoder ZSTD
    // Używamy std::fs::File dla wyjścia, bo większość bibliotek zstd operuje na sync I/O
    let plik_out = std::fs::File::create(&sciezka_out)?;
    let mut decoder = Decoder::new(plik_in.into_std().await)?;
    println!("przeszło decoder");

    // 4. Pętla przetwarzania (Dekompresja)
    let mut bufor = vec![0u8; 128 * 1024]; // 128KB
    let mut przeczytano_skompresowanych = 0u64;
    let mut ostatni_procent = 0u8;

    // Uwaga: W przypadku dekompresji czytamy z DECODERA, a on sam dba o czytanie z pliku
    // Aby jednak mieć procenty, musimy wiedzieć ile bajtów "surowych" weszło do systemu.
    // ZSTD Decoder implementuje Read, więc czytamy przez niego:

    use std::io::Read; // Importujemy sync Read dla decodera

    // Ponieważ decoder blokuje, a jesteśmy w async, najlepiej zrobić to w pętli
    // lub użyć read_to_end, ale przy dużych plikach (100MB+) pętla z buforem jest lepsza.

    let mut decoder_sync = decoder;
    let mut plik_out_sync = plik_out;

    // Tutaj mały "trick": Zstd Decoder nie raportuje łatwo postępu wewnątrz.
    // Najdokładniej byłoby opakować plik wejściowy w strukturę liczącą bajty,
    // ale najprościej dla Twojego UX będzie aktualizować procenty co kawałek bufora.
println!("wejście do loop");
    loop {
        let n = decoder_sync.read(&mut bufor)?;
        if n == 0 { break; }

        use std::io::Write;
        plik_out_sync.write_all(&bufor[..n])?;

        // Przy dekompresji trudno o idealny procent z samego czytania decodera,
        // dlatego często wysyła się po prostu etap "Dekompresja..."
        // ALBO liczy się postęp na podstawie wewnętrznego licznika decodera jeśli biblioteka pozwala.
        // Jeśli biblioteka zstd (zstd-rs) nie daje dostępu do licznika, robimy skokowy progres:

        // Opcja: szacunkowy progres (np. co 1MB)
        przeczytano_skompresowanych += n as u64;

        // --- ZMIANA TUTAJ ---
        // Wysyłamy realną ilość przetworzonych bajtów (pamięć)
        let _ = tx.send(ProgressDe::Dekompresja {
            etap: KolejnośćDziałańDe::Dekompresja,
            pamięć: Some(przeczytano_skompresowanych),
            status: ProcesStatus::Wtrakcie
        }).await;
    }

    let _ = tx.send(ProgressDe::Dekompresja {
        etap: KolejnośćDziałańDe::Dekompresja,
        pamięć: None,
        status: ProcesStatus::IO
    }).await;

    // 5. Sprzątanie
    drop(decoder_sync);
    drop(plik_out_sync);

    // Usuwamy plik .jrz_temp (ten po XOR), zostaje czysta binarka .jrz_temp_clean
    tokio::fs::remove_file(sciezka_in).await?;

    let _ = tx.send(ProgressDe::Dekompresja {
        etap: KolejnośćDziałańDe::Dekompresja,
        pamięć: None,
        status: ProcesStatus::Zakończone
    }).await;

    println!("Dekompresja {} zakończona.", nazwa_pliku);
    Ok(())
}