use enumy::send::wyslij_status;
use enumy::statusy::LogTxBinUnpak;
use iced::futures::channel::mpsc;
use pass::BAŁDZOTAJNEHASŁO;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Instant;
/// # Decoding binary file
/// So, here we've output file path ('ścieżka_docelowa'), file name ('nazwa_pliku) and ofc futures mpsc...
/// And then save copy with different extension name and delete original file etc...
pub async fn deszyfruj_xor(
    ścieżka_docelowa: PathBuf,
    nazwa_pliku: String,
    mut tx: mpsc::Sender<LogTxBinUnpak>,
) -> Result<(), tokio::io::Error> {

    let ścieżka_in = ścieżka_docelowa.join(format!("{}_temp_coded", nazwa_pliku));
    let ścieżka_out = ścieżka_docelowa.join(format!("{}_temp", nazwa_pliku));

    let mut plik_in = std::fs::File::open(&ścieżka_in)?;
    let metadata = plik_in.metadata()?;
    let calkowity_rozmiar = metadata.len();
    let mut plik_out = std::fs::File::create(&ścieżka_out)?;

    let klucz = BAŁDZOTAJNEHASŁO.as_bytes();
    let mut przeczytano_razem = 0u64;
    let mut bufor = vec![0u8; 128 * 1024];
    let mut ostatni_stan = Instant::now();

    loop {
        // Jeśli odczyt padnie, `?` natychmiast wyrzuci błąd wyżej
        let n = plik_in.read(&mut bufor)?;
        if n == 0 {
            break; // Koniec pliku - sukces
        }

        // pozyca starowa liczona w kluczu RAZ na całą paczkę (128KB)
        let mut klucz_idx = (przeczytano_razem % klucz.len() as u64) as usize;

        for bajt in &mut bufor[..n] {
            *bajt ^= klucz[klucz_idx];

            // zwykła inkrementacja
            klucz_idx += 1;
            if klucz_idx >= klucz.len() {
                klucz_idx = 0;
            }
        }

        plik_out.write_all(&bufor[..n])?;
        przeczytano_razem += n as u64;
        
        if ostatni_stan.elapsed().as_millis() >= 250 {
            ostatni_stan = Instant::now();
            wyslij_status(&mut tx, Some(LogTxBinUnpak::Deszyfracja {
                current: przeczytano_razem, 
                max: Some(calkowity_rozmiar + 3),
            })).await;
        }
    }

    wyslij_status(&mut tx, Some(LogTxBinUnpak::Deszyfracja {
        current: przeczytano_razem +1,
        max: Some(calkowity_rozmiar + 3),
    })).await;
    
    plik_out.flush()?;
    
    drop(plik_in);
    drop(plik_out);
    
    wyslij_status(&mut tx, Some(LogTxBinUnpak::Deszyfracja {
        current: przeczytano_razem +2,
        max: Some(calkowity_rozmiar + 3),
    })).await;
    
    std::fs::remove_file(ścieżka_in.clone())?;
    
    wyslij_status(&mut tx, Some(LogTxBinUnpak::Deszyfracja {
        current: przeczytano_razem +3,
        max: Some(calkowity_rozmiar + 3),
    })).await;
    
    Ok(())
}
