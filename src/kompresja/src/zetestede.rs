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

/// ZSTD compression module for files.
/// All u need is:
/// - path to file folder
/// - file name (.jrz_temp)
/// - compression level (none = Brak, standard = Standard, max = Duża)
/// - futures mpsc Sender
/// 
/// # Errors
/// Can throw error if something in arguments isn't right (not valid data)
/// Shouldn't be a problem here, coz reasons. (yeah, it's checked)
pub async fn kompresujsuj(
    ścieżka_pliku: PathBuf,
    nazwa_pliku: String,
    poziom_kompresji: OptKompresjaPlikówPoziomKompresjiZstd,
    mut tx: mpsc::Sender<LogTxBinPak>,
) -> Result<(), tokio::io::Error> {
// walidacja wejścia
// ------------------------------------------------------------------------------------------
    if nazwa_pliku.trim().is_empty() {
        let msg = "[Binary packing: validation] Nazwa pliku jest pusta!".to_string();
        generuj_plik_logow(msg.clone());
        return Err(tokio::io::Error::new(tokio::io::ErrorKind::InvalidInput, msg));
    }

    if !ścieżka_pliku.is_dir() {
        let msg = format!("[Binary packing: validation] Ścieżka nie jest katalogiem: {:?}", ścieżka_pliku);
        generuj_plik_logow(msg.clone());
        return Err(tokio::io::Error::new(tokio::io::ErrorKind::InvalidInput, msg));
    }
// ------------------------------------------------------------------------------------------

    let sciezka_temp = ścieżka_pliku.join(format!("{}.jrz_tmp", nazwa_pliku));
    let sciezka_final = ścieżka_pliku.join(format!("{}.jrz", nazwa_pliku));

    let mut plik_in = File::open(&sciezka_temp).await?;

    let metadata = plik_in.metadata().await?;
    let calkowity_rozmiar = metadata.len();
    
    let plik_out = std::fs::File::create(&sciezka_final)?;
    let mut encoder = Encoder::new(plik_out, poziom_kompresji as i32)?;

    let mut bufor = vec![0u8; 128 * 1024]; // 128KB bufor
    let mut przeczytano_razem = 0u64;

    // while let Ok(n) = plik_in.read(&mut bufor).await {
    //     if n == 0 {
    //         break;
    //     }
    //
    //     encoder.write_all(&bufor[..n])?;
    //
    //     przeczytano_razem += n as u64;
    //
    //     tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    //
    //     wyslij_status(&mut tx,Some(LogTxBinPak::Kompresja{ aktualny: przeczytano_razem, suma: Some(calkowity_rozmiar) })).await;
    //
    // }

    // regular loop coz w/ while let even when disc will be unplugged broken file will be.
    loop {
        let n = plik_in.read(&mut bufor).await?;
        if n == 0 {
            break;
        }
        encoder.write_all(&bufor[..n])?;
        przeczytano_razem += n as u64;

        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;

        wyslij_status(
            &mut tx,
            Some(LogTxBinPak::Kompresja {
                aktualny: przeczytano_razem,
                suma: Some(calkowity_rozmiar)
            })
        ).await;
    }

    if let Err(e) = encoder.finish() {
        let msg = format!("[Binary packing: compression: encoding] {}",e);
        generuj_plik_logow(msg.clone());
        return Err(e);

    }

    if let Err(e) = tokio::fs::remove_file(sciezka_temp).await {
        let msg = format!("[Binary packing: compression: file removal] {}",e);
        generuj_plik_logow(msg.clone());
        return Err(e);
    };

    Ok(())
}
