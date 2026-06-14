use enumy::dane_do_przetwarzania::DaneBinPak;
use enumy::fn_ogolne_przeliczeniowe::przelicz_czas;
use enumy::halper_fn::chck_av_spc_n_stuff;
use enumy::opcje::OptKompresjaPlikówFiltracjaPlików;
use enumy::send::wyslij_status;
use enumy::statusy::LogTxBinPak;
use iced::futures::channel::mpsc;
use kompresja::zetestede::kompresujsuj;
use std::path::{Path, PathBuf};
use std::time::Instant;
use enumy::log_file_gen::generuj_plik_logow;
use szyfrowanie::xor_sz::szyfruj_xor;

/// # Gettin' files
/// We're getting file name and data here from files.
/// 
/// Taking input folder('ścieżka') and goin' around to get every 
/// (or almost every, depends on choosed filtration method)
/// file and making Vec out of it.
/// So we can put such vec in loop and so on...
async fn zgarnij_pliki(
    ścieżka: PathBuf,
    opcja: &bool,
    filter: OptKompresjaPlikówFiltracjaPlików,
    mut tx: mpsc::Sender<LogTxBinPak>,
) -> Result<Vec<(String, Vec<u8>)>, tokio::io::Error> {
    
    let mut lista_plików = Vec::new();
    let mut licznik:u32 = 0;
    let mut foldery_do_przejrzenia = vec![ścieżka.clone()];

    wyslij_status(&mut tx,Some(LogTxBinPak::StatusZnaleziono { pliki: 0 })).await;

    while let Some(aktualny_folder) = foldery_do_przejrzenia.pop() {
        let mut wejścia = tokio::fs::read_dir(aktualny_folder).await?;

        while let Some(entry) = wejścia.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                foldery_do_przejrzenia.push(path);
            } else if path.is_file() {
                if !czy_plik_pasuje(&path, &filter) {
                    continue;
                }

                let nazwa = match opcja {
                    true => path
                        .strip_prefix(&ścieżka)
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    false => path.file_name().unwrap().to_string_lossy().into_owned(),
                };

                let tresc = tokio::fs::read(&path).await?;
                lista_plików.push((nazwa, tresc));
                licznik += 1;

                wyslij_status(&mut tx,Some(LogTxBinPak::StatusZnaleziono { pliki: licznik })).await;
                
            }
        }
    }

    Ok(lista_plików)
}

/// # Filtration
/// Fn for filtration, as set in gui.
/// 
fn czy_plik_pasuje(plik: &Path, filtr: &OptKompresjaPlikówFiltracjaPlików) -> bool {

    let ext = plik
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match filtr {
        OptKompresjaPlikówFiltracjaPlików::Graficzne => matches!(
            ext.as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "exr" | "avif"
        ),
        OptKompresjaPlikówFiltracjaPlików::Audio => {
            matches!(ext.as_str(), "mp3" | "wav" | "ogg" | "flac" | "m4a")
        }
        OptKompresjaPlikówFiltracjaPlików::Tekstowe => {
            matches!(ext.as_str(), "txt" | "md" | "json" | "cfg" | "ini" | "log")
        }
        OptKompresjaPlikówFiltracjaPlików::Pdf => ext == "pdf",
        _ => true,
    }
}
/// # Creating binary file
/// 
/// Creating binary file from files collected in other places.
/// 
/// ONE TO RULE 'EM ALL!!!!!
/// 
/// We're getting file names and data from fn zgarnij_pliki.
/// 
/// So:
/// - pliki -> such Vec with names and data
/// - ścieżka_wyjściowa -> Output folder path
/// - nazwa_pliku -> name of desired file
/// - tx -> ofc futures mpsc channel ;)
async fn tworzenie_binarki(
    pliki: Vec<(String, Vec<u8>)>,
    ścieżka_wyjściowa: PathBuf,
    nazwa_pliku: String,
    mut tx: mpsc::Sender<LogTxBinPak>,
) -> Result<(), tokio::io::Error> {
    
    let mut blobloblob = Vec::new();
    let suma = pliki.len() as u32;
    blobloblob.extend_from_slice(&(pliki.len() as u32).to_le_bytes());

    #[allow(clippy::explicit_counter_loop)] //ya, coz clippy sometimes can't see overall idea
    for (i, (nazwa, dane)) in pliki.into_iter().enumerate() {
        let n_bytes = nazwa.as_bytes();

        // Pakowanie metadanych i danych
        blobloblob.extend_from_slice(&(n_bytes.len() as u32).to_le_bytes());
        blobloblob.extend_from_slice(n_bytes);
        blobloblob.extend_from_slice(&(dane.len() as u64).to_le_bytes());
        blobloblob.extend_from_slice(&dane);


        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        wyslij_status(&mut tx,Some(LogTxBinPak::Pakowanie { aktualny: (i + 1) as u32, suma: Some(suma) })).await;

    }

    let ścieżka_temp = ścieżka_wyjściowa.join(format!("{}.jrz_tmp", nazwa_pliku));

    if let Some(parent) = ścieżka_temp.parent() && !parent.exists() {
            tokio::fs::create_dir_all(parent).await?;
    }


    if let Err(e) = tokio::fs::write(&ścieżka_temp, blobloblob).await {
        let msg = format!("[Binary packing: creating] {}",e);
        generuj_plik_logow(msg.clone());
        wyslij_status(&mut tx,Some(LogTxBinPak::Błąd(msg))).await;
    }

    Ok(())
}
/// # Main fn for packing binary file
/// so as u can see, there's some magic...
/// Getting data from gui, matching 'wynik' and stuff...
/// 
/// ah, yeah...
/// Here's async Tokio, but mpsc is from 'futures' ;)
pub async fn ogarnianie_eksportu(
    zestaw_danych: DaneBinPak,
    mut tx: mpsc::Sender<LogTxBinPak>,
) -> Result<(), tokio::io::Error> {
    let start_czas = Instant::now();
    
    

    let wynik = async {
        
        checkin_data(&zestaw_danych)?;
        let (in_path, out_path, poz_kompresji, strukturalnie, nazwa_pliku) = (
            zestaw_danych.ścieżka_in,
            zestaw_danych.ścieżka_out,
            zestaw_danych.kompresja,
            zestaw_danych.foldery,
            zestaw_danych.nazwa,
        );
        chck_av_spc_n_stuff(&in_path)?;
        let wsio_dane =
            zgarnij_pliki(in_path, &strukturalnie, zestaw_danych.filtracja, tx.clone()).await?;
        tworzenie_binarki(wsio_dane, out_path.clone(), nazwa_pliku.clone(), tx.clone()).await?;

        kompresujsuj(
            out_path.clone(),
            nazwa_pliku.clone(),
            poz_kompresji,
            tx.clone(),
        )
        .await?;

        szyfruj_xor(out_path, nazwa_pliku, tx.clone()).await?;
        Ok::<(), tokio::io::Error>(())
    }
    .await;

    match wynik {
        Ok(_) => {
            wyslij_status(&mut tx,Some(LogTxBinPak::Finito(przelicz_czas(start_czas)))).await;
            Ok(())
        }
        Err(e) => {
            let msg = format!("[Binary packing] {}",e);
            generuj_plik_logow(msg.clone());
            wyslij_status(&mut tx,Some(LogTxBinPak::Błąd(e.to_string()))).await;
            Err(e)
        }
    }
}

/// # Check data
/// checking data (file name and input path)
fn checkin_data(data: &DaneBinPak) -> Result<(), std::io::Error> {
    if data.nazwa.is_empty(){ return Err(std::io::Error::other("[Binary packing: checkin_data] File name is empty")); }
    if !data.ścieżka_in.exists(){ return Err(std::io::Error::other("[Binary packing: checkin_data] Input folder doesn't exist ;(")); }
    Ok(())
}