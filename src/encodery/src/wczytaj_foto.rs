use image::ImageDecoder;
use std::io::Read;
use std::path::PathBuf;
use image::DynamicImage;
use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};

pub fn wczytaj_zdjęcie(
    ścieżka: PathBuf,
) -> Result<(DynamicImage, String), std::io::Error> {
    let bajty = std::fs::read(&ścieżka)?;
    let nazwa = ścieżka
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("nieznany")
        .to_string();
    let rozszerzenie = ścieżka
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 1. Najpierw przygotowujemy surowe bajty (rozpakowane lub nie)
    let dane_obrazu = match rozszerzenie.as_str() {
        "avif" => {
            let lib_heif = LibHeif::new();
            let ctx = HeifContext::read_from_bytes(&bajty)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            let handle = ctx.primary_image_handle()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

            // Sprawdzamy czy obraz ma alphę
            let has_alpha = handle.has_alpha_channel();

            // Dekodujemy do Interleaved RGBA lub RGB.
            // Uwaga: libheif-rs przy RgbChroma::Rgba/Rgb zawsze daje 8 bitów.
            // Jeśli chcesz 10/12 bit, musiałbyś dekodować do Planar, co jest trudniejsze.
            let chroma = if has_alpha { RgbChroma::Rgba } else { RgbChroma::Rgb };
            let bytes_per_pixel = if has_alpha { 4 } else { 3 };

            let image = lib_heif.decode(
                &handle,
                ColorSpace::Rgb(chroma),
                None,
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

            let width = image.width() as usize;
            let height = image.height() as usize;

            let planes = image.planes();
            let interleaved = planes.interleaved.ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "Brak danych interleaved")
            })?;

            let data = interleaved.data;
            let stride = interleaved.stride as usize; // KONWERSJA NA USIZE

            let mut clean_vec = Vec::with_capacity(width * height * bytes_per_pixel);

            for y in 0..height {
                let start = y * stride;
                let end = start + (width * bytes_per_pixel);

                // Teraz start i end to usize, więc indeksowanie zadziała
                clean_vec.extend_from_slice(&data[start..end]);
            }

            clean_vec
        }
        "zst" => {
            let mut decoder = zstd::stream::read::Decoder::new(&bajty[..])?;
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            rozpakowane
        }
        "bz2" => {
            let mut decoder = bzip2::read::BzDecoder::new(&bajty[..]);
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            rozpakowane
        }
        "xz" => {
            let mut decoder = xz2::read::XzDecoder::new(&bajty[..]);
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            rozpakowane
        }
        _ => bajty,
    };

    // 2. Tworzymy Reader, który automatycznie rozpozna format (JPG, PNG, itp.)
    // let cursor = std::io::Cursor::new(&dane_obrazu);
    // let reader = image::ImageReader::new(cursor).with_guessed_format()?;
    // 2. Poprawione rozpoznawanie formatu
    let cursor = std::io::Cursor::new(&dane_obrazu);
    let mut reader = image::ImageReader::new(cursor).with_guessed_format()?;

    // Jeśli automatyczne rozpoznanie po bajtach zawiodło (częste dla TGA)
    if reader.format().is_none() {
        // Sprawdzamy czy to nie był skompresowany TGA lub czy oryginał to TGA
        // Możemy spróbować wymusić format TGA jeśli rozszerzenie na to wskazuje
        if rozszerzenie == "tga"  {
            reader.set_format(image::ImageFormat::Tga);
        }
    }


    // 3. Dobieramy się do dekodera, żeby wyciągnąć EXIF
    let mut decoder = reader.into_decoder().map_err(std::io::Error::other)?;
    // let profil_icc = decoder.icc_profile().map_err(std::io::Error::other)?;
    // Tutaj wyciągamy EXIF (metoda z traitu ImageDecoder, który wrzuciłeś)
    // let exif = decoder.exif_metadata().ok().flatten();

    // 4. Dekodujemy sam obraz
    let img = DynamicImage::from_decoder(decoder).map_err(std::io::Error::other)?;

    Ok((img, nazwa))
}