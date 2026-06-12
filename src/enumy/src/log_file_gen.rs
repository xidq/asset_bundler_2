use chrono::Local;
use std::fs::File;
use std::io::Write;
use std::fs;
/// # Generating log file
/// so...
///
/// We're here...
///
/// If smth wrong then there should be generated log text file in docs dir.
/// Or in same dir. Or in temp dir.
///
pub fn generuj_plik_logow(wiadomosc: String){
    let mut sciezka_pliku = dirs::document_dir()
        .unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_else(|_| std::env::temp_dir())
        });

    sciezka_pliku.push("Asset Bundler");

    // BEZPIECZNE TWORZENIE KATALOGU
    // Jeśli nie możemy stworzyć folderu (np. brak uprawnień zapisu w Dokumentach),
    // natychmiast uciekamy do folderu tymczasowego systemu.
    if !sciezka_pliku.exists() {
        match fs::create_dir_all(&sciezka_pliku){
            Ok(_) => {}
            Err(_) => {
                sciezka_pliku = std::env::temp_dir().join("Asset Bundler");
                // Jeśli nawet w tempie nie możemy stworzyć folderu, ignorujemy to i próbujemy iść dalej
                let _ = fs::create_dir_all(&sciezka_pliku);
            }
        };
    }

    sciezka_pliku.push("asset_bundler.log");

    // let plik = File::options()
    //     .append(true)
    //     .create(true)
    //     .open(&sciezka_pliku);
    //
    let formatowana_wiadomosc = format!("{} ->\n{}\n------------------------------\n", Local::now().format("%d/%m/%Y %H:%M:%S"), wiadomosc);

    match File::options().append(true).create(true).open(&sciezka_pliku) {
        Ok(mut otwarty_plik) => {
            // Próba zapisu. Jeśli dysk jest pełny, uszkodzony lub zablokowany,
            // .is_err() wyłapie problem, a program przeżyje.
            if otwarty_plik.write_all(formatowana_wiadomosc.as_bytes()).is_err() {
                // Ostateczny ratunek: rzut na standardowe wyjście błędów (konsola)
                eprintln!("BŁĄD ZAPISU LOGU DO PLIKU: {}", formatowana_wiadomosc);
            }
        }
        Err(e) => {
            // Jeśli całkowicie odcięło nas od zapisu na dysku (brak jakichkolwiek uprawnień)
            eprintln!(
                "NIE MOŻNA OTWORZYĆ PLIKU LOGÓW ({}): {}\nWiadomość: {}",
                sciezka_pliku.display(),
                e,
                formatowana_wiadomosc
            );
        }
    }
}