use enumy::log_file_gen::generuj_plik_logow;
use enumy::opcje::OptKompresjaPlikówPoziomKompresjiZstd;
use enumy::send::wyslij_status;
use enumy::statusy::LogTxBinPak;
use iced::futures::channel::mpsc;
use std::io::Write;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use zstd::*;

pub async fn kompresujsuj(
    ścieżka_pliku: PathBuf,
    nazwa_pliku: String,
    poziom_kompresji: OptKompresjaPlikówPoziomKompresjiZstd,
    mut tx: mpsc::Sender<LogTxBinPak>,
) -> Result<(), tokio::io::Error> {

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
    let mut encoder = Encoder::new(plik_out, poziom)?;

    let mut bufor = vec![0u8; 128 * 1024]; // 128KB bufor
    let mut przeczytano_razem = 0u64;

    while let Ok(n) = plik_in.read(&mut bufor).await {
        if n == 0 {
            break;
        } 


        encoder.write_all(&bufor[..n])?;


        przeczytano_razem += n as u64;
        
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;

        wyslij_status(&mut tx,Some(LogTxBinPak::Kompresja{ aktualny: przeczytano_razem as u32, suma: Some(calkowity_rozmiar as u32) })).await;
        
    }

    if let Err(e) = encoder.finish() {
        let msg = format!("[Binary packing: compression: encoding] {}",e);
        generuj_plik_logow(msg.clone());
        return Err(e);

    }

    if let Err(e) = tokio::fs::remove_file(sciezka_temp).await
    {
        let msg = format!("[Binary packing: compression: file removal] {}",e);
        generuj_plik_logow(msg.clone());
        return Err(e);
    };

    Ok(())
}
