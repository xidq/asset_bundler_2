use image::DynamicImage;
use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};
use std::io::Read;
use std::path::PathBuf;
/// # Read image (obsolete)
/// get image from path to memory
///
/// try using wczytaj_pliki(PathBuf) instead
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


    let (dane_obrazu,depth,w,h) = match rozszerzenie.as_str() {
        "avif" => {
            // let lib_heif = LibHeif::new();
            // let ctx = HeifContext::read_from_bytes(&bajty)
            //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            // let handle = ctx.primary_image_handle()
            //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            //
            //
            // let has_alpha = handle.has_alpha_channel();
            //
            // // Dekodujemy do Interleaved RGBA lub RGB.
            // // Uwaga: libheif-rs przy RgbChroma::Rgba/Rgb zawsze daje 8 bitów.
            // // Jeśli chcesz 10/12 bit, musiałbyś dekodować do Planar, co jest trudniejsze.
            // let chroma = if has_alpha { RgbChroma::Rgba } else { RgbChroma::Rgb };
            // let bytes_per_pixel = if has_alpha { 4 } else { 3 };
            //
            // let image = lib_heif.decode(
            //     &handle,
            //     ColorSpace::Rgb(chroma),
            //     None,
            // ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            //
            // let width = image.width() as usize;
            // let height = image.height() as usize;
            //
            // let planes = image.planes();
            // let interleaved = planes.interleaved.ok_or_else(|| {
            //     std::io::Error::new(std::io::ErrorKind::Other, "Brak danych interleaved")
            // })?;
            //
            // let data = interleaved.data;
            // let stride = interleaved.stride as usize;
            //
            // let mut clean_vec = Vec::with_capacity(width * height * bytes_per_pixel);
            //
            // for y in 0..height {
            //     let start = y * stride;
            //     let end = start + (width * bytes_per_pixel);
            //
            //     // Teraz start i end to usize, więc indeksowanie zadziała
            //     clean_vec.extend_from_slice(&data[start..end]);
            // }
            //
            // clean_vec
            let lib_heif = LibHeif::new();
            let ctx = HeifContext::read_from_bytes(&bajty)
                .map_err(std::io::Error::other)?;
            let handle = ctx.primary_image_handle()
                .map_err(std::io::Error::other)?;

            let has_alpha = handle.has_alpha_channel();
            let bit_depth = handle.luma_bits_per_pixel(); // Sprawdzamy bity (8, 10, 12)
            // let width = handle.width() as usize;
            // let height = handle.height() as usize;

            // Decydujemy o formacie dekodowania
            // Jeśli bity > 8, używamy trybu HDR (16-bit na kanał)
            let (chroma, bytes_per_channel) = if bit_depth > 8 {
                if has_alpha { (RgbChroma::HdrRgbaBe, 2) } else { (RgbChroma::HdrRgbBe, 2) }
            } else {
                if has_alpha { (RgbChroma::Rgba, 1) } else { (RgbChroma::Rgb, 1) }
            };

            let image = lib_heif.decode(
                &handle,
                ColorSpace::Rgb(chroma),
                None,
            ).map_err(std::io::Error::other)?;

            let width = image.width() as usize;
            let height = image.height() as usize;
            let channels = if has_alpha { 4 } else { 3 };

            let planes = image.planes();
            let interleaved = planes.interleaved.ok_or_else(|| {
                std::io::Error::other("Brak danych interleaved")
            })?;

            let data = interleaved.data; // To jest &[u8]
            let stride = interleaved.stride as usize;

            // clean_vec będzie zawierać surowe bajty.
            // Jeśli to HDR, każde 2 bajty tworzą jeden kanał (u16, Little Endian).
            let mut clean_vec = Vec::with_capacity(width * height * channels * bytes_per_channel);

            for y in 0..height {
                let line_start = y * stride;
                let line_end = line_start + (width * channels * bytes_per_channel);

                // Kopiujemy całą linię uwzględniając stride i szerokość danych
                clean_vec.extend_from_slice(&data[line_start..line_end]);
            }

            (clean_vec,bit_depth,width,height)
        }
        "zst" => {
            let mut decoder = zstd::stream::read::Decoder::new(&bajty[..])?;
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            (rozpakowane, 8,0,0)
        }
        "bz2" => {
            let mut decoder = bzip2::read::BzDecoder::new(&bajty[..]);
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            (rozpakowane, 8,0,0)
        }
        "xz" => {
            let mut decoder = xz2::read::XzDecoder::new(&bajty[..]);
            let mut rozpakowane = Vec::new();
            decoder.read_to_end(&mut rozpakowane)?;
            (rozpakowane, 8,0,0)
        }
        _ => (bajty, 8,0,0),
    };


    // let cursor = std::io::Cursor::new(&dane_obrazu);
    // let mut reader = image::ImageReader::new(cursor).with_guessed_format()?;
    //
    // // Jeśli automatyczne rozpoznanie po bajtach zawiodło (częste dla TGA)
    // if reader.format().is_none() {
    //     if rozszerzenie == "tga"  {
    //         reader.set_format(image::ImageFormat::Tga);
    //     }
    // }
    //
    //
    //
    // let mut decoder = reader.into_decoder().map_err(std::io::Error::other)?;
    //
    //
    //
    // let img = DynamicImage::from_decoder(decoder).map_err(std::io::Error::other)?;
    let img: DynamicImage = if rozszerzenie == "avif" {
        // Tu używasz zdekodowanego 'clean_vec' i 'bit_depth' z libheif
        if depth > 8 {
            // Obraz 10/12 bit promujemy do 16-bitowego DynamicImage
            let data_u16: Vec<u16> = dane_obrazu.chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();

            let buffer = image::ImageBuffer::<image::Rgba<u16>, _>::from_raw(w as u32, h as u32, data_u16)
                .ok_or_else(|| std::io::Error::other("Błąd bufora AVIF 16-bit"))?;

            image::DynamicImage::ImageRgba16(buffer)
        } else {
            // Standardowe 8 bit
            let buffer = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(w as u32, h as u32, dane_obrazu)
                .ok_or_else(|| std::io::Error::other("Błąd bufora AVIF 8-bit"))?;

            image::DynamicImage::ImageRgba8(buffer)
        }
    } else {
        // dotychczasowa logika dla reszty świata (JPG, PNG, TGA)
        let cursor = std::io::Cursor::new(&dane_obrazu);
        let mut reader = image::ImageReader::new(cursor).with_guessed_format()?;

        if reader.format().is_none() && rozszerzenie == "tga" {
            reader.set_format(image::ImageFormat::Tga);
        }

        reader.decode().map_err(std::io::Error::other)?
    };

    Ok((img, nazwa))
}