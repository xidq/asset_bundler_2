use enumy::send::wyslij_status;
use enumy::statusy::LogTxBinPak;
use iced::futures::channel::mpsc;
use pass::BAŁDZOTAJNEHASŁO;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn szyfruj_xor(
    ścieżka: PathBuf,
    nazwa: String,
    mut tx: mpsc::Sender<LogTxBinPak>,
) -> Result<(), tokio::io::Error> {

    let sciezka_in = ścieżka.join(format!("{}.jrz", nazwa)); // Plik po kompresji
    let sciezka_out = ścieżka.join(format!("{}.jrzs", nazwa)); // Plik zaszyfrowany (finalny)

    let mut plik_in = File::open(&sciezka_in).await?;
    let metadata = plik_in.metadata().await?;
    let calkowity_rozmiar = metadata.len();

    let mut plik_out = File::create(&sciezka_out).await?;

    let klucz = BAŁDZOTAJNEHASŁO.as_bytes();
    let mut przeczytano_razem = 0u64;
    let mut bufor = vec![0u8; 128 * 1024]; // 128KB bufor

    while let Ok(n) = plik_in.read(&mut bufor).await {
        if n == 0 {
            break;
        }

        // Aplikujemy XOR na tym konkretnym kawałku (buforze)
        // Musimy wiedzieć, na którym bajcie całego pliku jesteśmy,
        // aby zachować ciągłość klucza XOR (przeczytano_razem + i)
        for (i, bajt) in bufor[..n].iter_mut().enumerate() {
            let pozycja_w_pliku = przeczytano_razem + i as u64;
            *bajt ^= klucz[pozycja_w_pliku as usize % klucz.len()];
        }

        plik_out.write_all(&bufor[..n]).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        // Aktualizacja postępu
        przeczytano_razem += n as u64;

        wyslij_status(&mut tx,Some(LogTxBinPak::Szyfrowanie {
            aktualny: przeczytano_razem as u32,
            suma: Some(calkowity_rozmiar as u32 +2),
        })).await;

        
    }

    wyslij_status(&mut tx,Some(LogTxBinPak::Szyfrowanie {
        aktualny: przeczytano_razem as u32 + 1,
        suma: Some(calkowity_rozmiar as u32 +2),
    })).await;

    plik_out.flush().await?;

    tokio::fs::remove_file(sciezka_in).await?;
    wyslij_status(&mut tx,Some(LogTxBinPak::Szyfrowanie {
        aktualny: przeczytano_razem as u32 + 2,
        suma: Some(calkowity_rozmiar as u32 +2),
    })).await;

    Ok(())
}
