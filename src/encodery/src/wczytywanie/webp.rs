use crate::DaneDoWczytywania;
use webp::Decoder;
use enumy::rozszerzenia::kolor::ColorProfilePhoto;

/// # Decoding webp
/// ;)
pub fn webp(bajty: &[u8]) -> Result<DaneDoWczytywania, std::io::Error> {

    let decoder = Decoder::new(bajty);
    // let webp_img = decoder.decode()
    //     .ok_or_else(|| std::io::Error::other("Błąd dekodowania WebP"))?;
    let obraz = match decoder.decode() {
        Some(img) => img.to_image(),
        None => {
            // Jeśli zawiedzie, prawdopodobnie mamy do czynienia z animacją (VP8X)
            // Musimy skorzystać z WebPAnimDecoder z biblioteki libwebp-sys (pod spodem)
            // Albo użyć ogólnego crate 'image', który ma wsparcie dla klatek WebP.

            // Alternatywa: użycie image::load_from_memory_with_format
            // To najbezpieczniejsza metoda na "pierwszą klatkę" bez pisania własnego parsera klatek
            image::load_from_memory_with_format(bajty, image::ImageFormat::WebP)
                .map_err(|e| std::io::Error::other(format!("Błąd dekodowania klatki WebP: {}", e)))?
        }
    };
    // let obraz = webp_img.to_image();

    // parsowanie RIFF "ręcznie" -- metadane
    let mut exif_out = None;
    let mut typ_koloru = ColorProfilePhoto::None;

    if let Ok(meta) = dej_dane(bajty) {
        exif_out = meta.exif;
        if let Some(icc) = meta.icc {
            typ_koloru = ColorProfilePhoto::ICC(icc);
        }
    }

    Ok(DaneDoWczytywania {
        dane: obraz,
        exif: exif_out,
        kolor: typ_koloru,
    })
}

struct WebPMetadata {
    exif: Option<Vec<u8>>,
    icc: Option<Vec<u8>>,
}

fn dej_dane(data: &[u8]) -> Result<WebPMetadata, std::io::Error> {
    let mut metadata = WebPMetadata { exif: None, icc: None };

    // WebP zaczyna się od: "RIFF" (4b) + size (4b) + "WEBP" (4b)
    if data.len() < 12 || &data[0..4] != b"RIFF" || &data[8..12] != b"WEBP" {
        return Err(std::io::Error::other("To nie jest poprawny kontener WebP/RIFF"));
    }

    let mut pos = 12;
    while pos + 8 < data.len() {
        let chunk_id = &data[pos..pos + 4];
        let chunk_len = u32::from_le_bytes([data[pos+4], data[pos+5], data[pos+6], data[pos+7]]) as usize;
        pos += 8;

        if pos + chunk_len > data.len() { break; }

        match chunk_id {
            b"EXIF" => metadata.exif = Some(data[pos..pos + chunk_len].to_vec()),
            b"ICCP" => metadata.icc = Some(data[pos..pos + chunk_len].to_vec()),
            _ => {}
        }

        pos += chunk_len;
        // Chunk musi być wyrównany do parzystej liczby bajtów
        if pos % 2 != 0 { pos += 1; }
    }

    Ok(metadata)
}