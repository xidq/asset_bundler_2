use crate::no_sync::bardzo_tajne_haslo::BAŁDZOTAJNEHASŁO;
use std::path::PathBuf;
use iced::futures::channel::mpsc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use iced::futures::SinkExt;
use crate::io::enums_structs_io::ProcesStatus;
use crate::io::export_with_compression::{ProgressDe, KolejnośćDziałańDe};

// Pamiętaj o zdefiniowaniu klucza gdzieś w stałych

pub async fn deszyfruj_xor(
    ścieżka_docelowa: PathBuf,
    nazwa_pliku: String,
    mut tx: mpsc::Sender<ProgressDe>
) -> Result<(), tokio::io::Error> {

    let _ = tx.send(ProgressDe::Deszyfracja {
        etap: KolejnośćDziałańDe::Deszyfrowanie,
        procent: None,
        status: ProcesStatus::Rozpoczęte
    }).await;


    println!("Rozpoczęcie deszyfrowania XOR dla: {}", nazwa_pliku);

    // 1. Składamy ścieżki na podstawie argumentów (prosto i przejrzyście)
    let ścieżka_in = ścieżka_docelowa.join(format!("{}_temp_coded", nazwa_pliku));
    let ścieżka_out = ścieżka_docelowa.join(format!("{}_temp", nazwa_pliku));

    // 2. Otwarcie plików
    println!("ścieżka przed otwarciem: \n {:?}",&ścieżka_in);
    let mut plik_in = tokio::fs::File::open(&ścieżka_in).await?;
    let metadata = plik_in.metadata().await?;
    let calkowity_rozmiar = metadata.len();
println!("ścieżka w xor de : \n {:?}", &ścieżka_out);
    let mut plik_out = tokio::fs::File::create(&ścieżka_out).await?;

    let klucz = BAŁDZOTAJNEHASŁO.as_bytes();
    let mut przeczytano_razem = 0u64;
    let mut bufor = vec![0u8; 128 * 1024];
    let mut ostatni_procent = 0u8;

    // 3. Pętla XOR
    while let Ok(n) = tokio::io::AsyncReadExt::read(&mut plik_in, &mut bufor).await {
        if n == 0 { break; }

        for (i, bajt) in bufor[..n].iter_mut().enumerate() {
            let pozycja_w_pliku = przeczytano_razem + i as u64;
            *bajt ^= klucz[pozycja_w_pliku as usize % klucz.len()];
        }

        tokio::io::AsyncWriteExt::write_all(&mut plik_out, &bufor[..n]).await?;
        przeczytano_razem += n as u64;

        let procent = ((przeczytano_razem as f64 / calkowity_rozmiar as f64) * 100.0) as u8;
        if procent > ostatni_procent {
            ostatni_procent = procent;
            let _ = tx.send(ProgressDe::Deszyfracja {
                etap: KolejnośćDziałańDe::Deszyfrowanie,
                procent: Some(procent),
                status: ProcesStatus::Wtrakcie
            }).await;
        }
    }


    let _ = tx.send(ProgressDe::Deszyfracja {
        etap: KolejnośćDziałańDe::Deszyfrowanie,
        procent: None,
        status: ProcesStatus::IO
    }).await;


    // 4. Finalizacja
    tokio::io::AsyncWriteExt::flush(&mut plik_out).await?;

    // Zamykamy uchwyty zanim usuniemy plik
    drop(plik_in);
    drop(plik_out);
    println!("ścieżka in istnienie: {}: \n {:?}",&ścieżka_in.exists(),&ścieżka_in);
    println!("ścieżka out istnienie: {}: \n {:?}",&ścieżka_out.exists(),&ścieżka_out);

    // 5. Usuwamy wejście (.jrzs_temp), zostaje tylko .jrz_temp
    tokio::fs::remove_file(ścieżka_in.clone()).await?;
    println!("ścieżka in istnienie: {}: \n {:?}",&ścieżka_in.exists(),&ścieżka_in);
    println!("ścieżka out istnienie: {}: \n {:?}",&ścieżka_out.exists(),&ścieżka_out);

    println!("Deszyfrowanie {} zakończone.", nazwa_pliku);

    let _ = tx.send(ProgressDe::Deszyfracja {
        etap: KolejnośćDziałańDe::Deszyfrowanie,
        procent: None,
        status: ProcesStatus::Zakończone
    }).await;
    Ok(())
}