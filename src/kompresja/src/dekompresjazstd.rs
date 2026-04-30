use iced::futures::channel::mpsc;
use std::path::PathBuf;
use iced::futures::SinkExt;
use tokio::fs::File;
use tokio::time::Instant;
use zstd::*;
use enumy::statusy::LogTxDoDekompresjiPliku;

pub async fn dekompresujsuj(
    ścieżka_pliku: PathBuf,
    nazwa_pliku: String,
    mut tx: mpsc::Sender<LogTxDoDekompresjiPliku>,
) -> Result<(), tokio::io::Error> {
    let _ = tx
        .send(LogTxDoDekompresjiPliku::StatusDekompresjaPlikówDekompresja { pamięć: 0 })
        .await;
    println!("Rozpoczęcie fn dekompresji");


    let sciezka_in = ścieżka_pliku.join(format!("{}_temp", nazwa_pliku));
    let sciezka_out = ścieżka_pliku.join(format!("{}_temp_clean", nazwa_pliku));


    println!("sciezka_in: \n{:?}", sciezka_in);
    println!("sciezka_out: \n{:?}", sciezka_out);
    let plik_in = File::open(&sciezka_in).await?;
    // let metadata = plik_in.metadata().await?;
    // let rozmiar_skompresowany = metadata.len();


    let plik_out = std::fs::File::create(&sciezka_out)?;
    let decoder = Decoder::new(plik_in.into_std().await)?;
    println!("przeszło decoder");


    let mut bufor = vec![0u8; 128 * 1024]; // 128KB
    let mut przeczytano_skompresowanych = 0u64;



    use std::io::Read; // Importujemy sync Read dla decodera



    let mut decoder_sync = decoder;
    let mut plik_out_sync = plik_out;
    let mut czasss = Instant::now();


    loop {
        let n = decoder_sync.read(&mut bufor)?;
        if n == 0 {
            break;
        }

        use std::io::Write;
        plik_out_sync.write_all(&bufor[..n])?;


        przeczytano_skompresowanych += n as u64;

        if czasss.elapsed().as_millis() >= 250 {
            czasss = Instant::now();
            let _ = tx
                .send(
                    LogTxDoDekompresjiPliku::StatusDekompresjaPlikówDekompresja {
                        pamięć: przeczytano_skompresowanych,
                    },
                )
                .await;
        }
    }

    let _ = tx
        .send(LogTxDoDekompresjiPliku::StatusDekompresjaPlikówDekompresja { pamięć: przeczytano_skompresowanych })
        .await;

    // 5. Sprzątanie
    drop(decoder_sync);
    drop(plik_out_sync);


    tokio::fs::remove_file(sciezka_in).await?;


    Ok(())
}
