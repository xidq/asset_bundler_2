use enumy::send::wyslij_status;
use enumy::statusy::LogTxBinUnpak;
use iced::futures::channel::mpsc;
use std::path::PathBuf;
use tokio::time::Instant;

pub async fn dekompresujsuj(
    ścieżka_pliku: PathBuf,
    nazwa_pliku: String,
    mut tx: mpsc::Sender<LogTxBinUnpak>,
) -> Result<(), tokio::io::Error> {

    wyslij_status(&mut tx, Some(LogTxBinUnpak::Dekompresja { pamięć: 0 })).await;

    let sciezka_in = ścieżka_pliku.join(format!("{}_temp", nazwa_pliku));
    let sciezka_out = ścieżka_pliku.join(format!("{}_temp_clean", nazwa_pliku));

    let plik_in = std::fs::File::open(&sciezka_in)?;

    let plik_out = std::fs::File::create(&sciezka_out)?;
    let decoder = zstd::stream::read::Decoder::new(plik_in)?;

    let mut bufor = vec![0u8; 128 * 1024]; // 128KB
    let mut przeczytano_rozpakowanych = 0u64;

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

        przeczytano_rozpakowanych += n as u64;

        if czasss.elapsed().as_millis() >= 250 {
            czasss = Instant::now();
            wyslij_status(&mut tx, Some(LogTxBinUnpak::Dekompresja { pamięć: przeczytano_rozpakowanych })).await;
        }
    }
// just bcoz I can ;)
// If u know, u know
    wyslij_status(&mut tx, Some(LogTxBinUnpak::Dekompresja { pamięć: przeczytano_rozpakowanych })).await;

    // Sprzątanie
    drop(decoder_sync);
    drop(plik_out_sync);

    tokio::fs::remove_file(sciezka_in).await?;

    Ok(())
}
