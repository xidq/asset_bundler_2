// use little_exif::rational::uR64;
use crate::foty::edycja_jpg::edycja_jpg;
use crate::foty::edycja_png::edycja_png;
use crate::foty::wczytanie_zdjec::wczytaj_zdjęcie;
use crate::io::enums_structs_io::Progress;
use futures::channel::mpsc;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use futures::executor::block_on;
use futures::SinkExt;
use image::{DynamicImage, GenericImage, GenericImageView};
use rand::{Rng, RngExt};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::Mutex;
use walkdir::WalkDir;
use crate::foty::edycja_tga::edycja_tga;
#[allow(dead_code)]
#[derive(Clone)]
pub struct EdycjaZdjęć{
    pub(crate) ścieżka_wejściowa: PathBuf,
    pub(crate) ścieżka_wyjściowa: PathBuf,
    pub(crate) opcje_rozdzielczości: Vec<Rozdzielczości>,
    // dane_exif: DaneExif,
    pub(crate) noising: Option<u8>,
    pub(crate) rozszerzenia: Vec<Rozszerzenia>,
    pub(crate) inter: Interpolacja,
    pub(crate) alfa_rgb:(u16,u16,u16)
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PostepMieleniaZdjec{
    Start,
    Rozpoczęto(u32, u8),
    FiltrowaniePlików(u32),
    PominiętePliki { sciezka: String, powod: String },
    Koniec(String),
    Błąd(String),
}

// #[derive(Debug, Clone)]
// pub struct RozszerzeniaIopcje{
//     pub(crate) rozszerzenie: Rozszerzenia,
//     pub(crate) bit_depth: Vec<Obraz>,
// }
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Interpolacja{
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}
#[allow(dead_code)]
#[derive(Clone,  Debug)]
pub enum Rozszerzenia{
    Jpg{ jakosc: u8, progresywny: bool , bit_depth: Vec<Obraz> },
    Png{ kompresja: u8,  bit_depth: Vec<Obraz> },
    Webp{ jakosc: u8, lossless: bool, bit_depth: Vec<Obraz> },
    Tga{ bit_depth:Vec<ObrazTGA>}
}
#[allow(dead_code)]
#[derive(Debug,Copy, Clone,PartialEq)]
pub(crate) enum Obraz{
    L8,
    L8a,
    B8,
    B8a,
    L16,
    L16a,
    B16,
    B16a,
    B32,
    B32a,
}
#[allow(dead_code)]
#[derive(Debug,Copy, Clone,PartialEq)]
pub(crate) enum ObrazTGA{
    Szary8,
    HighColor16, //alpga 1 bit (on/off)
    TrueColor24,
    TrueColorA32
}
// #[derive(Clone)]
// pub enum CoRobimyZExif{
//     Usuwamy,
//     Zostawiamy,
//     Zmieniamy
// }
#[allow(dead_code)]
#[derive(Clone)]
pub enum Zaszumianie{
    Tak(u8),
    Nie,
}
#[allow(dead_code)]
#[derive(Debug, Clone,PartialEq)]
pub enum Rozdzielczości{
    R16,
    R32,
    R64,
    R128,
    R256,
    R512,
    R1k,
    R2k,
    R4k,
    R6k,
    R8k,
    R16k,
    Oryginalna,
}


pub async fn ogarnianie_foto(zestaw_danych: EdycjaZdjęć, mut tx: mpsc::Sender<PostepMieleniaZdjec>) ->Result<(), tokio::io::Error>{
    let start_czas = Instant::now();
    let obecna_operacja:Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let procent_progress:Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
    let wsio_dane = Arc::new(zestaw_danych);

    let wynik = async {


        let ścieżki_do_zdjęć = if !wsio_dane.ścieżka_wejściowa.is_file() {
            // println!("to jest na foldery");
            wez_sprawdz_sciezki(wsio_dane.ścieżka_wejściowa.clone(), &mut tx)
        } else {
            // println!("to jest na pliki");
            zgarnij_dane_z_pliku(wsio_dane.ścieżka_wejściowa.clone(), &mut tx)
        };


        let ile_rozdzielczosci = wsio_dane.opcje_rozdzielczości.len() as u32;

        let mut suma_wariantow_bit_depth = 0u32;

        for rozszerzenie in &wsio_dane.rozszerzenia {
            match rozszerzenie {
                Rozszerzenia::Jpg { bit_depth, .. } => {
                    // Jeśli JPG ma zaznaczone L8 i B8, to są 2 warianty
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                Rozszerzenia::Png { bit_depth, .. } => {
                    // Jeśli PNG ma zaznaczone B8, B16, L16, to są 3 warianty
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
                Rozszerzenia::Webp { .. } => {
                    // Webp u Ciebie nie ma bit_depth w enumie, więc liczymy jako 1
                    suma_wariantow_bit_depth += 1;
                }
                Rozszerzenia::Tga {bit_depth, ..} => {
                    suma_wariantow_bit_depth += bit_depth.len() as u32;
                }
            }
        }


        let total_operacji = ścieżki_do_zdjęć.len() as u32 * ile_rozdzielczosci * suma_wariantow_bit_depth;
        let metryka_operacji = total_operacji;

        let _ = tx.send(PostepMieleniaZdjec::FiltrowaniePlików(ścieżki_do_zdjęć.len() as u32)).await;

        let tx_dla_rayona = tx.clone();

            // for p in ścieżki_do_zdjęć{
            //     let (bufor, nazwa) = wczytaj_zdjęcie(p.0)?;
        tokio::task::spawn_blocking(move || {
            ścieżki_do_zdjęć.par_iter().try_for_each(|p| {
                let (bufor, nazwa) = wczytaj_zdjęcie(p.0.clone())?;
                for r in &wsio_dane.rozszerzenia{

                    block_on(async {
                        let tx_zadanie = tx_dla_rayona.clone();

                        match &r {
                            Rozszerzenia::Jpg { jakosc, progresywny, bit_depth } => {
                                edycja_jpg(
                                    bufor.clone(),
                                    &wsio_dane.opcje_rozdzielczości,
                                    &wsio_dane.ścieżka_wyjściowa,
                                    &p.2, //ścieżka dopełniająca
                                    &wsio_dane.inter,
                                    &nazwa,
                                    jakosc,
                                    progresywny,
                                    bit_depth,
                                    &wsio_dane.alfa_rgb,
                                    wsio_dane.noising,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    procent_progress.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            Rozszerzenia::Png { kompresja, bit_depth } => {
                                edycja_png(
                                    bufor.clone(),
                                    &wsio_dane.opcje_rozdzielczości,
                                    &wsio_dane.ścieżka_wyjściowa,
                                    &p.2,
                                    &wsio_dane.inter,
                                    &nazwa,
                                    kompresja,
                                    &wsio_dane.alfa_rgb,
                                    bit_depth,
                                    wsio_dane.noising,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    procent_progress.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                            Rozszerzenia::Webp { jakosc , lossless, bit_depth} => {
                                // placeholder
                            }
                            Rozszerzenia::Tga { bit_depth } => {
                                edycja_tga(
                                    bufor.clone(),
                                    &wsio_dane.opcje_rozdzielczości,
                                    &wsio_dane.ścieżka_wyjściowa,
                                    &p.2,
                                    &wsio_dane.inter,
                                    &nazwa,
                                    &wsio_dane.alfa_rgb,
                                    bit_depth,
                                    wsio_dane.noising,
                                    metryka_operacji,
                                    obecna_operacja.clone(),
                                    procent_progress.clone(),
                                    tx_zadanie,
                                ).await?;
                            }
                        }
                        Ok::<(), tokio::io::Error>(())
                    })?;
                }
                Ok::<(), tokio::io::Error>(())
            })
        }).await.map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::Other, e.to_string()))?
    }.await;


    match wynik {
        Ok(_) => {
            let trwanie = start_czas.elapsed(); // Zwraca strukturę Duration

            // Formatujemy czas na ładny napis, np. "1.23s" lub "45ms"
            // Możesz użyć prostego formatowania:
            let czas_napis = format!("{:.2?}", trwanie);
            let _ = tx.send(PostepMieleniaZdjec::Koniec(czas_napis)).await;
            Ok(())
        },
        Err(e) => {
            // Jeśli cokolwiek powyżej sypnie błędem (przez znak zapytania),
            // wysyłamy opis błędu do UI zamiast po prostu "padać".
            let _ = tx.send(PostepMieleniaZdjec::Błąd(e.to_string())).await;
            Err(e)
        }
    }


}


fn wez_sprawdz_sciezki(sciezka: PathBuf, tx: &mut mpsc::Sender<PostepMieleniaZdjec>) -> Vec<(PathBuf, String, String)> {
    let rozszerzenia = ["jpg", "jpeg", "png", "webp"];
    let mut przetworzone_pliki: u32 = 0;
    let mut do_wyjscia = Vec::new();

    // WalkDir jako iterator
    for entry in WalkDir::new(&sciezka).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

            if rozszerzenia.contains(&ext.to_lowercase().as_str()) {
                let pełna_ścieżka = path.to_path_buf();

                // BRAMKARZ - sprawdzamy czy ścieżka jest git
                // Jeśli nie, funkcja czy_sciezka_jest_git sama wyśle raport przez tx
                if let Some(sprawdzona_ścieżka) = czy_sciezka_jest_git(pełna_ścieżka, tx) {

                    let nazwa_pliku = sprawdzona_ścieżka
                        .file_stem()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "nieznany".to_string());

                    let ścieżka_dopełniająca = sprawdzona_ścieżka
                        .strip_prefix(&sciezka)
                        .ok()
                        .and_then(|p| p.parent())
                        .map(|p| p.to_string_lossy().into_owned())
                        .unwrap_or_default();

                    // NALICZANIE
                    przetworzone_pliki += 1;
                    let _ = tx.try_send(PostepMieleniaZdjec::FiltrowaniePlików(przetworzone_pliki));

                    do_wyjscia.push((sprawdzona_ścieżka, nazwa_pliku, ścieżka_dopełniająca));
                }
            }
        }
    }

    do_wyjscia
}

fn zgarnij_dane_z_pliku(ścieżka:PathBuf, tx: &mut mpsc::Sender<PostepMieleniaZdjec>) -> Vec<(PathBuf, String, String)>{
    // println!("jestem w zgarnij dane z pliku!!!!!");
    let mut przetworzone_pliki:u32 = 0;
    let mut do_wyjscia = Vec::new();
    let rozszerzenia = ["jpg", "jpeg", "png", "webp"];

    // Używamy WalkDir, żeby ogarnąć foldery i podfoldery
    for entry in WalkDir::new(&ścieżka).into_iter().flatten() {
        let path = entry.path();

        if path.is_file() {
            // 1. Sprawdzamy rozszerzenie
            let ext = path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            if rozszerzenia.contains(&ext.as_str()) {
                // 2. Wyciągamy ścieżkę do folderu (bez nazwy pliku)
                // parent() zwraca ścieżkę o jeden poziom wyżej
                let sciezka_bez_pliku = path.parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(PathBuf::new);

                // 3. Wyciągamy nazwę pliku bez rozszerzenia (file_stem)
                let nazwa_pliku = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                przetworzone_pliki +=1;
                let _ = tx.try_send(PostepMieleniaZdjec::FiltrowaniePlików
                    (przetworzone_pliki)
                );
                // println!("[zgarnij_dane_z_pliku]ścieżka: {:?}\nnazwa pliku:{:?}\nścieżka dopełniająca: {}",ścieżka.clone(), nazwa_pliku,String::from(""));
                do_wyjscia.push((ścieżka.clone(), nazwa_pliku,String::from("")));
            }
        }
    }


    do_wyjscia
}

pub fn zaszumianie(noising: u8, mut bufor:DynamicImage) -> DynamicImage{
    // let mut xoxo = bufor.clone();
    let mut rng = rand::rng();
    let (w,h) = bufor.dimensions();
    let n_factor = noising as f64 / 100.0;
    let bit_depth = bufor.color().bits_per_pixel() / bufor.color().channel_count() as u16;

    let max_val: f64 = match bit_depth {
        8 => 255.0,
        16 => 65535.0,
        32 => 1.0, // Dla obrazów HDR (f32)
        _ => 255.0,
    };

    // 2. Obliczamy zakres szumu na podstawie % (noising)
    // Jeśli noising = 100, to max_delta = max_val
    let max_delta = (max_val * (noising as f64 / 100.0));


    match bufor {
        // --- OBSŁUGA 8-BIT ---
        DynamicImage::ImageRgba8(mut img) => {
            let max_val = 255.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                for i in 0..3 { // Tylko R, G, B
                    let v = pixel.0[i] as f64;
                    let delta = rng.random_range(-max_delta..=max_delta);
                    pixel.0[i] = (v + delta).clamp(0.0, max_val) as u8;
                }
            }
            DynamicImage::ImageRgba8(img)
        }

        // --- OBSŁUGA 16-BIT ---
        DynamicImage::ImageRgba16(mut img) => {
            let max_val = 65535.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                for i in 0..3 {
                    let v = pixel.0[i] as f64;
                    let delta = rng.random_range(-max_delta..=max_delta);
                    pixel.0[i] = (v + delta).clamp(0.0, max_val) as u16;
                }
            }
            DynamicImage::ImageRgba16(img)
        }

        // --- OBSŁUGA 32-BIT (F32) ---
        DynamicImage::ImageRgba32F(mut img) => {
            let max_val = 1.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                for i in 0..3 {
                    let v = pixel.0[i] as f64;
                    let delta = rng.random_range(-max_delta..=max_delta);
                    pixel.0[i] = (v + delta).clamp(0.0, max_val) as f32;
                }
            }
            DynamicImage::ImageRgba32F(img)
        }

        // Jeśli wpadnie format bez Alfy (RGB), traktujemy go tak samo
        DynamicImage::ImageRgb8(mut img) => {
            let max_val = 255.0;
            let max_delta = max_val * n_factor;
            for pixel in img.pixels_mut() {
                for i in 0..3 {
                    let v = pixel.0[i] as f64;
                    let delta = rng.random_range(-max_delta..=max_delta);
                    pixel.0[i] = (v + delta).clamp(0.0, max_val) as u8;
                }
            }
            DynamicImage::ImageRgb8(img)
        }

        _ => bufor, // Reszta formatów bez zmian
    }
}
// fn losuj_i_wyrownaj(v: f64, rng: &mut impl Rng, max_delta: f64, max_val: f64) -> f64 {
//
//     let delta: f64 = rng.random_range(-max_delta..=max_delta);
//     let val = v + delta;
//
//     // Clamp zależny od bit-depth (max_val)
//     val.clamp(0.0, max_val)
//
// }
// fn wyczysc_sciezke(s: String) -> String {
//     s.chars().map(|c| {
//         match c {
//             // Zamień japońskie nawiasy, ukośniki i inne dziwadła na bezpieczny znak
//             '｢' | '｣' | '／' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
//             // Opcjonalnie: jeśli chcesz być super bezpieczny, usuń znaki spoza ASCII
//             _ if !c.is_ascii() && c.is_control() => '_',
//             _ => c,
//         }
//     }).collect()
// }

fn czy_sciezka_jest_git(
    pelna: PathBuf,
    tx: &mut mpsc::Sender<PostepMieleniaZdjec>
) -> Option<PathBuf> {
    let s = pelna.to_string_lossy();
    let zakazane_znaki = ['／', '\0', '｢', '｣', '\\', '*', '?', '"', '<', '>', '|'];

    // Szukamy pierwszego wystąpienia zakazanego znaku
    // if let Some(znaleziony_znak) = s.chars().find(|c| zakazane_znaki.contains(c)) {
    //     let _ = tx.try_send(PostepMieleniaZdjec::PominiętePliki {
    //         sciezka: s.to_string(),
    //         powod: format!("Niedozwolony znak ({}) w ścieżce", znaleziony_znak)
    //     });
    //     return None;
    // }

    // 2. Limit bajtowy dla Linuxa (255 bajtów na folder/plik)
    for komponent in pelna.components() {
        if komponent.as_os_str().as_encoded_bytes().len() > 250 {
            let _ = tx.try_send(PostepMieleniaZdjec::PominiętePliki {
                sciezka: s.to_string(),
                powod: format!("Człon ścieżki przekracza limit 250 bajtów, jest {} bajtów",komponent.as_os_str().as_encoded_bytes().len())
            });
            return None;
        }
    }

    // Jak wszystko przeszło, zwracamy ścieżkę z powrotem
    Some(pelna)
}