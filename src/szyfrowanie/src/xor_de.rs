use enumy::statusy::LogTxBinUnpak;
use iced::futures::channel::mpsc;
use iced::futures::SinkExt;
use pass::BAŁDZOTAJNEHASŁO;
use std::path::PathBuf;
use std::time::Instant;

// Pamiętaj o zdefiniowaniu klucza gdzieś w stałych

pub async fn deszyfruj_xor(
    ścieżka_docelowa: PathBuf,
    nazwa_pliku: String,
    mut tx: mpsc::Sender<LogTxBinUnpak>,
) -> Result<(), tokio::io::Error> {


    println!("Rozpoczęcie deszyfrowania XOR dla: {}", nazwa_pliku);

    // 1. Składamy ścieżki na podstawie argumentów (prosto i przejrzyście)
    let ścieżka_in = ścieżka_docelowa.join(format!("{}_temp_coded", nazwa_pliku));
    let ścieżka_out = ścieżka_docelowa.join(format!("{}_temp", nazwa_pliku));

    // 2. Otwarcie plików
    println!("ścieżka przed otwarciem: \n {:?}", &ścieżka_in);
    let mut plik_in = tokio::fs::File::open(&ścieżka_in).await?;
    let metadata = plik_in.metadata().await?;
    let calkowity_rozmiar = metadata.len();
    println!("ścieżka w xor de : \n {:?}", &ścieżka_out);
    let mut plik_out = tokio::fs::File::create(&ścieżka_out).await?;

    let klucz = BAŁDZOTAJNEHASŁO.as_bytes();
    let mut przeczytano_razem = 0u64;
    let mut bufor = vec![0u8; 128 * 1024];
    let mut ostatni_stan = Instant::now();

    // 3. Pętla XOR
    while let Ok(n) = tokio::io::AsyncReadExt::read(&mut plik_in, &mut bufor).await {
        if n == 0 {
            break;
        }

        for (i, bajt) in bufor[..n].iter_mut().enumerate() {
            let pozycja_w_pliku = przeczytano_razem + i as u64;
            *bajt ^= klucz[pozycja_w_pliku as usize % klucz.len()];
        }

        tokio::io::AsyncWriteExt::write_all(&mut plik_out, &bufor[..n]).await?;
        przeczytano_razem += n as u64;


        if ostatni_stan.elapsed().as_millis() >= 250  {
            ostatni_stan = Instant::now();
            let _ = tx
                .send(
                    LogTxBinUnpak::Deszyfracja {
                        current: przeczytano_razem as u32,
                        max: Some(calkowity_rozmiar as u32 + 3),
                    },
                )
                .await;
        }
    }

    let _ = tx
        .send(
            LogTxBinUnpak::Deszyfracja {
                current: przeczytano_razem as u32 + 1,
                max: Some(calkowity_rozmiar as u32 + 3),
            },
        )
        .await;



    tokio::io::AsyncWriteExt::flush(&mut plik_out).await?;


    drop(plik_in);
    drop(plik_out);
    let _ = tx
        .send(
            LogTxBinUnpak::Deszyfracja {
                current: przeczytano_razem as u32 + 2,
                max: Some(calkowity_rozmiar as u32 + 3),
            },
        )
        .await;
    
    
    tokio::fs::remove_file(ścieżka_in.clone()).await?;
    let _ = tx
        .send(
            LogTxBinUnpak::Deszyfracja {
                current: przeczytano_razem as u32 + 3,
                max: Some(calkowity_rozmiar as u32 + 3),
            },
        )
        .await;

    
    Ok(())
}
