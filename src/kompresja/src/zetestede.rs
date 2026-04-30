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



    let sciezka_temp = ścieżka_pliku.join(format!("{}.jrz_tmp", nazwa_pliku));
    let sciezka_final = ścieżka_pliku.join(format!("{}.jrz", nazwa_pliku));



    let mut plik_in = File::open(&sciezka_temp).await?;

    let metadata = plik_in.metadata().await?;
    let calkowity_rozmiar = metadata.len();


    let poziom = match poziom_kompresji {
        OptKompresjaPlikówPoziomKompresjiZstd::Brak => 0,
        OptKompresjaPlikówPoziomKompresjiZstd::Standard => 3,
        OptKompresjaPlikówPoziomKompresjiZstd::Duża => 10,
    };


    let plik_out = std::fs::File::create(&sciezka_final)?;
    println!("tworzony plik jrz (bez temp)");
    let mut encoder = Encoder::new(plik_out, poziom)?;


    let mut bufor = vec![0u8; 128 * 1024]; // 128KB bufor
    let mut przeczytano_razem = 0u64;
    let mut ostatni_procent = 0u8;

    while let Ok(n) = plik_in.read(&mut bufor).await {
        if n == 0 {
            break;
        } 

        // Zapis do kompresora
        encoder.write_all(&bufor[..n])?;

        // Obliczanie postępu
        przeczytano_razem += n as u64;
        let procent = ((przeczytano_razem as f64 / calkowity_rozmiar as f64) * 100.0) as u8;
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        if procent > ostatni_procent {
            if procent > 100{println!("procent w kompresji zstd ma powyżej 100")}
            ostatni_procent = procent;
            let _ = tx
                .send(
                    LogTxDoKompresjiPliku::StatusKompresjaPlikówProcesKompresji {
                        procent: Some(procent),
                    },
                )
                .await;
        }
    }


    if let Err(e) = encoder.finish() {
        let _ = tx
            .send(LogTxDoKompresjiPliku::StatusKompresjaPlikówBłąd(e.to_string()))
            .await;
        return Err(e);
    }



    if let Err(e) = tokio::fs::remove_file(sciezka_temp).await
    {
        let _ = tx
            .send(LogTxDoKompresjiPliku::StatusKompresjaPlikówBłąd(e.to_string()))
            .await;
        return Err(e);
    };



    Ok(())
}
