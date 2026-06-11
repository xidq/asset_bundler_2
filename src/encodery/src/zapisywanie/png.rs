use crate::halper::{konwersja_float_na_mniejsze, ogarnij_icc, usun_kanal_alpha, zaszumianie};
use crate::send::wyslij_status;
use crate::zapisywanie::generic::{get_higher_tier_copy, InneDane};
use enumy::opcje::OptIstniejePlik;
use enumy::przetwarzanie::{DaneDoPrzetwarzania, PrzetwarzaniePng};
use enumy::rozszerzenia::bdepth::BdepthPng;
use enumy::rozszerzenia::kolor::ColorProfilePhoto;
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use lcms2::PixelFormat;
use std::fs::{create_dir_all, File};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn png_match<T>(
    dane: PrzetwarzaniePng,
    dane2: InneDane<BdepthPng>,
    // wymiar: u32,
    // bit_depth: BdepthPng,
    // nazwa_wariantu: String,
    // filtr: FilterType,
    metryka_operacji: Option<u32>,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<T>,
) -> Result<(), tokio::io::Error>
where T: Logi,
{
    let (fotu, icc) = match &dane.kolor {
        ColorProfilePhoto::Exr(xxx) => {
            let obraz = dane.bufor().clone();
            konwersja_float_na_mniejsze(obraz, xxx.clone())
        },
        _ => (dane.bufor().clone(), dane.kolor.clone()),
    };
    let obrazek =
        if dane2.wymiar == 0 {
            fotu
        } else {
            fotu
                .resize(
                    dane2.wymiar,
                    dane2.wymiar,
                    dane2.filtr,
                )
        };

    let (final_img, nazwa_bd, profil, png_color, png_bdepth) = match dane2.bdepth {
        BdepthPng::Luma8 => (usun_kanal_alpha(obrazek, dane.alpha), "_l8b", PixelFormat::GRAY_8, png::ColorType::Grayscale, png::BitDepth::Eight),
        BdepthPng::Luma8Alpha => (obrazek, "_l8bt", PixelFormat::GRAYA_8, png::ColorType::GrayscaleAlpha, png::BitDepth::Eight),
        BdepthPng::Rgb8 => (usun_kanal_alpha(obrazek, dane.alpha), "_8b", PixelFormat::RGB_8, png::ColorType::Rgb, png::BitDepth::Eight),
        BdepthPng::Rgb8Alpha => (obrazek, "_8bt",  PixelFormat::RGBA_8, png::ColorType::Rgba, png::BitDepth::Eight),
        BdepthPng::Rgb16 => (usun_kanal_alpha(obrazek, dane.alpha), "_16b", PixelFormat::RGB_16, png::ColorType::Rgb, png::BitDepth::Sixteen),
        BdepthPng::Rgb16Alpha => (obrazek, "_16bt", PixelFormat::RGBA_16, png::ColorType::Rgba, png::BitDepth::Sixteen),
        BdepthPng::Luma16 => (usun_kanal_alpha(obrazek, dane.alpha), "_l16b", PixelFormat::GRAY_16, png::ColorType::Grayscale, png::BitDepth::Sixteen),
        BdepthPng::Luma16Alpha => (obrazek, "_l16bt", PixelFormat::GRAYA_16, png::ColorType::GrayscaleAlpha, png::BitDepth::Sixteen),
    };

    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);
    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;
    
    let final_final_final_v3_xd = match dane.zaszumienie {
        Some(x) => zaszumianie(x, final_img),
        None => final_img,
    };
    let mut plikkk = ogarnij_icc(
        icc,
        &final_final_final_v3_xd,
        profil
    );

    // 4. Budowanie nazwy
    let finalna_nazwa = format!("{}{}{}.png", dane.nazwa, dane2.nazwa_wariantu, nazwa_bd);
    let mut ścieżka_pliku = dane.sciezka_wyjsciowa.to_path_buf();
    if !ścieżka_pliku.exists() {
        create_dir_all(ścieżka_pliku.clone())?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    if ścieżka_pliku.exists() {
        match dane2.zastepowanie{
            OptIstniejePlik::Zamień => {}
            OptIstniejePlik::Zostaw => {
                let mut oopr = obecna_operacja.lock().await;
                *oopr += 1;
                let obecnie = *oopr;
                drop(oopr);

                wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;
                return Ok(())
            }
            OptIstniejePlik::ZmieńNazwę => {ścieżka_pliku = get_higher_tier_copy(ścieżka_pliku)}
        }
    };

    let mut bufor_png = Vec::new();

    // 5. Zapis z kompresją
    let f = File::create(&ścieżka_pliku)?;
    let _w = &mut std::io::BufWriter::new(f);

    let (width, height) = final_final_final_v3_xd.to_rgb8().dimensions();
    let mut encoder = png::Encoder::new(&mut bufor_png, width, height);
    encoder.set_color(png_color);
    encoder.set_depth(png_bdepth);
    // Mapowanie u8 kompresji (0-9) na poziomy PNG (Best, Fast, Default)
    let speed = match dane.kompresja {
        1 => png::Compression::Fastest,
        2 => png::Compression::Fast,
        3 => png::Compression::Balanced,
        4 => png::Compression::High,
        _ => png::Compression::NoCompression,
    };

    encoder.set_compression(speed);
    encoder.set_filter(png::Filter::Adaptive);

    let mut writer = encoder.write_header().map_err(std::io::Error::other)?;

    if png_bdepth == png::BitDepth::Sixteen {
        // Zamieniamy bajty miejscami w parach (2 bajty na kanał), aby uzyskać Big Endian
        for chunk in plikkk.chunks_exact_mut(2) {
            chunk.swap(0, 1);
        }
    }



    writer.write_image_data(&plikkk).map_err(std::io::Error::other)?;
    drop(writer);

    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);
    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;



    // --- RĘCZNE WSTRZYKIWANIE EXIF DO WEKTORA BAJTÓW ---
    if let Some(ref exif_bytes) = dane.exif &&
        !exif_bytes.is_empty() {
            // Każdy plik PNG zaczyna się od stałego 8-bajtowego nagłówka: [137, 80, 78, 71, 13, 10, 26, 10]
            // Zaraz za nim leci pierwszy chunk, czyli IHDR (który ma zawsze 25 bajtów: 4b długość + 4b nazwa + 13b dane + 4b CRC).
            // Idealne miejsce na nasz EXIF to pozycja tuż po chunku IHDR, czyli dokładnie na 33. bajcie pliku.
            if bufor_png.len() >= 33 {
                let mut chunk_exif = Vec::new();

                // 1. Długość danych EXIF (4 bajty, Big Endian)
                let dlugosc = exif_bytes.len() as u32;
                chunk_exif.extend_from_slice(&dlugosc.to_be_bytes());

                // 2. Nazwa chunku (4 bajty)
                let nazwa = b"eXIf";
                chunk_exif.extend_from_slice(nazwa);

                // 3. Surowe dane EXIF
                chunk_exif.extend_from_slice(exif_bytes);

                // 4. Suma kontrolna CRC32 (liczona z nazwy i danych)
                // Używamy algorytmu CRC32 standardowo zaszytego w crate `png` lub `zlib`
                // Jeśli masz w projekcie crate `crc32fast`, możesz użyć `crc32fast::hash`.
                // Tutaj użyjemy prostego wyliczenia, żeby nie dodawać zależności (zakładam, że masz dostęp do crc):
                let mut hasher = crc32fast::Hasher::new();
                hasher.update(nazwa);
                hasher.update(exif_bytes);
                let crc = hasher.finalize();
                chunk_exif.extend_from_slice(&crc.to_be_bytes());

                // Wstrzykujemy cały przygotowany chunk na 33. pozycję w wektorze pliku PNG
                let pozycja_wstrzykniecia = 33;
                bufor_png.splice(pozycja_wstrzykniecia..pozycja_wstrzykniecia, chunk_exif);
            }

    }
    // --------------------------------------------------

    // Zapisujemy ostateczny, zmodyfikowany bufor z pamięci na dysk
    std::fs::write(&ścieżka_pliku, bufor_png)?;


    // let encoder = image::codecs::png::PngEncoder::new_with_quality(
    //     f,
    //     speed,
    //     image::codecs::png::FilterType::Adaptive,
    // );
    //
    // final_final_final_v3_xd
    //     .write_with_encoder(encoder)
    //     .map_err(std::io::Error::other)?;

    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);
    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;

    Ok(())
}