use image::DynamicImage;
use std::path::PathBuf;


pub(crate) fn wczytaj_zdjęcie(ścieżka: PathBuf) -> Result<(DynamicImage,String), std::io::Error> {
    // 1. Czytamy bajty (std::io::Error)
    // println!("ścieżka przy wczutuwaniu pliku: {:?}",ścieżka);
    let bajty = std::fs::read(&ścieżka)?;
    // println!(" bajty? {:?}", bajty);
    let nazwa = ścieżka
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("nieznany")
        .to_string();
    // println!("pojedynczy plik nazwa: {}", nazwa);
    // 2. Dekodujemy (image::ImageError)
    // Dzięki Box<dyn Error>, oba typy błędów zostaną automatycznie skonwertowane
    let img = image::load_from_memory(&bajty)
        .map_err(std::io::Error::other)?;
    // println!("jest po wczytaniu pojedynczego pliku");
    Ok((img,nazwa))
}