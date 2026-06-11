use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write, /* Cursor */};
use iced_core::Color;

const DOMYSLNY_NAGLOWEK: &str = "
    ## Configuration file for Asset Bundler by Patryk Jerzak\n\n";

pub fn plik_z_ustawieniami_wczytaj(znacznik: bool) -> Result<Option<HashMap<String,String>>, std::io::Error> {
    let mut sciezka_pliku = dirs::document_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Brak folderu Dokumenty"))?;
    sciezka_pliku.push("Asset Bundler");
    if !sciezka_pliku.exists() {
        fs::create_dir_all(&sciezka_pliku)?;
    }
    sciezka_pliku.push("asset_bundler_settings.cfg");
    let mut ustawienia = HashMap::new();
    if !sciezka_pliku.exists() | znacznik {
        let tekst = DOMYSLNY_NAGLOWEK;
        let mut output_file = File::create(&sciezka_pliku)?;
        output_file.write_all(tekst.as_bytes())?;

        return Ok(None);

    }
    let mut buf = String::new();
    let mut plik_otwarty = File::open(&sciezka_pliku)?;
    plik_otwarty.read_to_string(&mut buf)?;
            for linia in buf.lines() {
        let linia = linia.trim();

        if linia.is_empty() || linia.starts_with("##") {
            continue;
        }
        if let Some((klucz, wartosc)) = linia.split_once('=') {
            let klucz_clean = klucz.trim().to_string();
            let wartosc_clean = wartosc.trim().to_string();

            if !klucz_clean.is_empty() {
                ustawienia.insert(klucz_clean, wartosc_clean);
            }
        }

    }
    let ghghg = if ustawienia.is_empty() { None } else { Some(ustawienia) };
    Ok(ghghg)
}

// fn match_ustawienia(mapa: HashMap<String, String>, ustawienia_wejsciowe: UstawieniaDoPliku) -> UstawieniaDoPliku {
//     let mut ustawienia = ustawienia_wejsciowe;
//     mapa.into_iter().for_each(|(k, v)| {
//         match k{
//
//             String { .. } => {}
//         }
//     });
//     ustawienia
// }

pub fn plik_z_ustawieniami_popraw(klucz: String, val: String) -> Result<(), std::io::Error> {
    let mut sciezka_pliku = dirs::document_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Brak folderu Dokumenty"))?;
    sciezka_pliku.push("Asset Bundler");
    sciezka_pliku.push("asset_bundler_settings.cfg");

    // println!("val:{},klucz:{}", val, klucz);

    // Jeśli plik z jakiegoś powodu nie istnieje, to nowy z domyślnym nagłówkiem
    if !sciezka_pliku.exists() {
        let tekst = DOMYSLNY_NAGLOWEK;
        let mut output_file = File::create(&sciezka_pliku)?;
        output_file.write_all(tekst.as_bytes())?;
    }

    let mut buf = String::new();
    let mut plik_otwarty = File::open(&sciezka_pliku)?;
    plik_otwarty.read_to_string(&mut buf)?;

    let mut nowy_tekst = String::new();
    let mut znaleziono_klucz = false;

    for linia in buf.lines() {
        if !linia.trim().starts_with("##") && !linia.trim().is_empty()
            && let Some((k, _)) = linia.split_once('=')
            && k.trim() == klucz {
                nowy_tekst.push_str(&format!("{} = {}\n", klucz, val));
                znaleziono_klucz = true;
                continue;


        }
        nowy_tekst.push_str(linia);
        nowy_tekst.push('\n');
    }

    if !znaleziono_klucz {
        nowy_tekst.push_str(&format!("{} = {}\n", klucz, val));
    }

    let mut plik_zapis = File::create(&sciezka_pliku)?;
    plik_zapis.write_all(nowy_tekst.as_bytes())?;

    Ok(())
}
#[allow(dead_code)]
pub fn key_to_color(kolor: String) -> Color{
    let ghhh: Vec<u8> = kolor
        .split(',')
        .map(|ff| {
            ff.trim().parse::<u8>().expect("Err przy liczbie")
        })
        .collect();
    Color::from_rgb8(ghhh[0], ghhh[1], ghhh[2] )
}

pub fn hex_to_color(kolor:String) -> Color {
    let hex = kolor.trim().trim_start_matches('#');
    println!("{}",hex);
    if hex.len() != 6 {
        return Color::BLACK;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

    Color::from_rgb8(r, g, b)
}