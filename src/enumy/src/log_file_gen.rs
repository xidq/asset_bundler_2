use chrono::Local;
use std::fs::File;
use std::io::Write;
use std::fs;
pub fn generuj_plik_logow(wiadomosc: String){
    let mut sciezka_pliku = dirs::document_dir().unwrap();
    sciezka_pliku.push("Asset Bundler");

    if !sciezka_pliku.exists() {
        _ = fs::create_dir_all(&sciezka_pliku);
    }
    sciezka_pliku.push("asset_bundler.log");

    let plik = File::options()
        .append(true)
        .create(true)
        .open(&sciezka_pliku);

    if let Ok(mut otwarty_plik) = plik {
        let formatowana_wiadomosc = format!("{} ->\n{}\n------------------------------", Local::now().format("%d/%m/%Y %H:%M:%S"), wiadomosc);

        _ = otwarty_plik.write_all(formatowana_wiadomosc.as_bytes());
    }
}