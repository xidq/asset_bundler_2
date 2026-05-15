use std::io::Cursor;
use image::ImageReader;
use enumy::rozszerzenia::kolor::ColorProfilePhoto;
use crate::wczytywanie::strukty::DaneDoWczytywania;

pub fn unknown(bajty: &[u8]) -> Result<DaneDoWczytywania, std::io::Error> {
    let cursor = Cursor::new(bajty);

    // 1. Próba zgadnięcia formatu na podstawie bajtów (magiczne liczby)
    let reader = ImageReader::new(cursor)
        .with_guessed_format()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // 2. Jeśli format nie został rozpoznany, wywalamy błąd
    if reader.format().is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Nie rozpoznano formatu pliku (brak znanej sygnatury)"
        ));
    }

    // 3. Dekodowanie do DynamicImage
    let obraz = reader.decode()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Dla nieznanych formatów trudno o generyczne wyciąganie ICC/EXIF bez matchowania,
    // więc zwracamy None.
    Ok(DaneDoWczytywania {
        dane: obraz,
        exif: None,
        kolor: ColorProfilePhoto::None,
    })
}