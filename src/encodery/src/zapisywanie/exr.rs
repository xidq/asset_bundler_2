use std::sync::Arc;
use exr::compression::Compression;
use exr::math::Vec2;
use exr::prelude::attribute::Chromaticities;
use futures::channel::mpsc::Sender;
use image::DynamicImage;
use image::imageops::FilterType;
use tokio::sync::Mutex;
use enumy::przetwarzanie::{DaneDoPrzetwarzania, PrzetwarzanieExr, PrzetwarzanieJpg};
use enumy::rozszerzenia::bdepth::{BdepthExr, BdepthJpg};
use enumy::rozszerzenia::kolor::{ColorProfilePhoto, PrzestrzeńExr};
use enumy::statusy::Logi;
use crate::halper::{konwersja_float_na_mniejsze, konwersja_mniejsze_na_float, usun_kanal_alpha};
use exr::prelude::*;
use enumy::rozszerzenia::kompresje::ForExrKompresja;

pub async fn exr_match<T>(
    dane: PrzetwarzanieExr, //będzie dostosowane do exr... gdzie będą kompresje, profile itd.
    wymiar: u32,
    bit_depth: BdepthExr, //tutaj też będą do exr rzeczy czyli na ten moment możesz używać BdepthExr::f16Half, BdepthExr::f32, BdepthExr::f32Half itp itd
    nazwa_wariantu: String,
    filtr: FilterType,
    metryka_operacji: Option<u32>,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<T>,
) -> std::result::Result<(), tokio::io::Error>
where T: Logi,
{
    // konwersja_mniejsze_na_float
    let (ghgh, icc) = match &dane.kolor {
        ColorProfilePhoto::Exr(_) => {
            (dane.bufor().clone(), dane.kolor.clone())
        },
        _ => konwersja_mniejsze_na_float(dane.bufor().clone(), dane.kolor.clone()),
    };

    let fotu = if wymiar == 0 {
        usun_kanal_alpha(ghgh, dane.alpha)
    } else {
        usun_kanal_alpha(ghgh, dane.alpha)
            .resize(
                wymiar,
                wymiar,
                filtr,
            )
    };


    // 2. Uzyskaj bufor f32 RGBA
    let rgba32f = match fotu {
        DynamicImage::ImageRgba32F(buf) => buf,
        other => other.to_rgba32f(),  // konwersja w razie potrzeby
    };
    let (width, height) = rgba32f.dimensions();
    let pixels: Vec<f32> = rgba32f.into_raw();

    // 3. Określenie ścieżki zapisu (możliwe, że w `dane` jest pole `sciezka`)
    // let output_path = dane.sciezka_wyjsciowa.join(&nazwa_wariantu).with_extension("exr");

    // 4. Parametry kompresji z obiektu `dane` (przyjmuję, że istnieje pole `kompresja: CompressionExr`)
    let encoding = match &dane.kompresja {
        ForExrKompresja::Brak            => Encoding {
            compression: Compression::Uncompressed,
            blocks: Blocks::ScanLines,
            line_order: LineOrder::Increasing,
        },
        ForExrKompresja::Rle             => Encoding {
            compression: Compression::RLE,
            blocks: Blocks::ScanLines,
            line_order: LineOrder::Increasing,
        },
        ForExrKompresja::Zip1            => Encoding {
            compression: Compression::ZIP1,
            blocks: Blocks::ScanLines,
            line_order: LineOrder::Increasing,
        },
        ForExrKompresja::Zip16           => Encoding {
            compression: Compression::ZIP16,
            blocks: Blocks::ScanLines,
            line_order: LineOrder::Increasing,
        },
        ForExrKompresja::Piz             => Encoding {
            compression: Compression::PIZ,
            blocks: Blocks::Tiles(Vec2(32, 32)), // PIZ preferuje kafle, domyślnie 32x32
            line_order: LineOrder::Increasing,
        },
        ForExrKompresja::Pxr24           => Encoding {
            compression: Compression::PXR24,
            blocks: Blocks::ScanLines,
            line_order: LineOrder::Increasing,
        },
        ForExrKompresja::B44             => Encoding {
            compression: Compression::B44,
            blocks: Blocks::ScanLines,
            line_order: LineOrder::Increasing,
        },
        ForExrKompresja::B44a            => Encoding {
            compression: Compression::B44A,
            blocks: Blocks::ScanLines,
            line_order: LineOrder::Increasing,
        },
        ForExrKompresja::Dwaa(level)     => Encoding {
            compression: Compression::DWAA(*level),
            blocks: Blocks::ScanLines,         // DWAA operuje na liniach
            line_order: LineOrder::Increasing,
        },
        ForExrKompresja::Dwab(level)     => Encoding {
            compression: Compression::DWAB(*level),
            blocks: Blocks::Tiles(Vec2(256, 256)), // DWAB używa bloków 256 linii → mapujemy na duże kafle
            line_order: LineOrder::Increasing,
        },
    };

    // 5. Wybór typu próbek (f16 / f32) na podstawie `bit_depth`
    let use_f16 = match bit_depth {
        BdepthExr::F16Half => true,   // odpowiedniki BdepthExr
        BdepthExr::F32 => false,
        BdepthExr::F32Half => true,   // jeśli to half w kontenerze 32-bit
        _ => false,                   // domyślnie f32
    };

    // 6. Informacja o chromatycznościach z profilu
    let chromaticities = if let ColorProfilePhoto::Exr(ref przestrzen) = icc {
        match przestrzen {
            PrzestrzeńExr::LinearCustom(primaries) => Some(Chromaticities {
                red:   Vec2(primaries[0], primaries[1]),
                green: Vec2(primaries[2], primaries[3]),
                blue:  Vec2(primaries[4], primaries[5]),
                white: Vec2(primaries[6], primaries[7]),
            }),
            _ => None,
        }
    } else {
        None
    };


    let num_pixels = (width * height) as usize;

    let channels = if use_f16 {
        // Konwersja do half-float
        let mut r = vec![f16::ZERO; num_pixels];
        let mut g = vec![f16::ZERO; num_pixels];
        let mut b = vec![f16::ZERO; num_pixels];
        let mut a = vec![f16::ONE; num_pixels];

        for i in 0..num_pixels {
            let base = i * 4;
            r[i] = f16::from_f32(pixels[base]);
            g[i] = f16::from_f32(pixels[base + 1]);
            b[i] = f16::from_f32(pixels[base + 2]);
            a[i] = f16::from_f32(pixels[base + 3]);
        }

        AnyChannels::sort(
            SmallVec::from([
            AnyChannel::new("R", FlatSamples::F16(r)),
            AnyChannel::new("G", FlatSamples::F16(g)),
            AnyChannel::new("B", FlatSamples::F16(b)),
            AnyChannel::new("A", FlatSamples::F16(a)),
        ]))
    } else {
        let mut r = vec![0.0f32; num_pixels];
        let mut g = vec![0.0f32; num_pixels];
        let mut b = vec![0.0f32; num_pixels];
        let mut a = vec![1.0f32; num_pixels];

        for i in 0..num_pixels {
            let base = i * 4;
            r[i] = pixels[base];
            g[i] = pixels[base + 1];
            b[i] = pixels[base + 2];
            a[i] = pixels[base + 3];
        }

        AnyChannels::sort(SmallVec::from([
            AnyChannel::new("R", FlatSamples::F32(r)),
            AnyChannel::new("G", FlatSamples::F32(g)),
            AnyChannel::new("B", FlatSamples::F32(b)),
            AnyChannel::new("A", FlatSamples::F32(a)),
        ]))
    };

    // ---------- 6. Warstwa i obraz ----------
    let layer = Layer::new(
        Vec2(width as usize, height as usize),
        LayerAttributes::default(),
        encoding,
        channels,
    );

    let mut image_attributes = ImageAttributes::new(IntegerBounds::from_dimensions(Vec2(width as usize, height as usize)));
    image_attributes.chromaticities = chromaticities;

    let image = Image::new(image_attributes, layer);

    // ---------- 7. Ścieżka zapisu ----------
    let output_path = dane.sciezka_wyjsciowa.join(&nazwa_wariantu).with_extension("exr");

    // ---------- 8. Zapis ----------
    image.write()
        .to_file(&output_path)
        .map_err(|e| tokio::io::Error::other(format!("Błąd zapisu EXR: {}", e)))?;
        // .map_err(|e| tokio::io::Error::new(tokio::io::ErrorKind::Other, format!("Błąd zapisu EXR: {e}")))?;



    Ok(())
}
