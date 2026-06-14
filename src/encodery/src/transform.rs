use image::DynamicImage;
use lcms2::{Profile, Transform, Intent, PixelFormat, CIExyY, CIExyYTRIPLE, ToneCurve};
use libheif_rs::{ColorPrimaries, TransferCharacteristics};
use enumy::rozszerzenia::kolor::ColorNclx;
/// Convert color
pub fn konwertuj_przestrzen(
    img: &DynamicImage,
    icc_wejscie: Option<&Vec<u8>>,
    pixel_format_wyjsciowy: PixelFormat,
) -> Result<Vec<u8>, std::io::Error> {

    // let profil = match icc_wejscie {
    //     None => {Profile::new_srgb()}
    //     Some(xxx) => {Profile::new_icc(xxx)
    //         .map_err(|_| std::io::Error::other("Błąd parsera profilu wejściowego ICC"))?}
    // };
    let wp_d65 = lcms2::CIExyY { x: 0.3127, y: 0.3290, Y: 1.0 };
    let curve_22 = lcms2::ToneCurve::new(2.2);
    // 1. Profil wejściowy z pliku (np. AdobeRGB)
    let p_src = match icc_wejscie {
        Some(bajty) => Profile::new_icc(bajty)
            .map_err(|_| std::io::Error::other("Błąd parsera profilu wejściowego ICC"))?,
        None => {
            // BRAK PROFILU: Sprawdzamy, czy wejściowy plik to szarość czy RGB
            match img {
                DynamicImage::ImageLuma8(_) | DynamicImage::ImageLumaA8(_) |
                DynamicImage::ImageLuma16(_) | DynamicImage::ImageLumaA16(_) => {
                    // Jeśli wejście to szarość, profil źródłowy TEŻ musi być szarością
                    Profile::new_gray(&wp_d65, &curve_22)
                        .map_err(|_| std::io::Error::other("LCMS2: Błąd profilu Gray dla źródła"))?
                },
                _ => Profile::new_srgb(), // W przeciwnym wypadku standardowe sRGB
            }
        }
    };

    // 2. Profil docelowy (np. sRGB)
    let p_dst = match pixel_format_wyjsciowy {
        PixelFormat::GRAY_8 | PixelFormat::GRAY_16 | PixelFormat::GRAYA_8 | PixelFormat::GRAYA_16 => Profile::new_gray(&wp_d65, &curve_22).unwrap(),
        _ => Profile::new_srgb(),
    };

    // DYNAMICZNE mapowanie formatu z DynamicImage na LCMS2
    // To zapobiega niszczeniu danych przy Luma8 czy Rgb16
    let f_src = match img {
        DynamicImage::ImageLuma8(_) => PixelFormat::GRAY_8,
        DynamicImage::ImageLumaA8(_) => PixelFormat::GRAYA_8,
        DynamicImage::ImageRgb8(_) => PixelFormat::RGB_8,
        DynamicImage::ImageRgba8(_) => PixelFormat::RGBA_8,

        // Obsługa 16-bitów (żeby nie ubić jakości w Rgb16/Rgba16)
        DynamicImage::ImageLuma16(_) => PixelFormat::GRAY_16,
        DynamicImage::ImageLumaA16(_) => PixelFormat::GRAYA_16,
        DynamicImage::ImageRgb16(_) => PixelFormat::RGB_16,
        DynamicImage::ImageRgba16(_) => PixelFormat::RGBA_16,

        _ => return Err(std::io::Error::other("Nienatywny lub nieobsługiwany format w DynamicImage")),
    };

    let zrodlo_ma_alpha = matches!(f_src,
        lcms2::PixelFormat::RGBA_8 | lcms2::PixelFormat::RGBA_16 |
        lcms2::PixelFormat::GRAYA_8 | lcms2::PixelFormat::GRAYA_16);

    let cel_wymaga_alpha = matches!(pixel_format_wyjsciowy,
        lcms2::PixelFormat::RGBA_8 | lcms2::PixelFormat::RGBA_16 |
        lcms2::PixelFormat::GRAYA_8 | lcms2::PixelFormat::GRAYA_16);


    // let flagi = lcms2::Flags::COPY_ALPHA;
    // 4. Inicjalizacja transformacji LCMS2
    // let tr = Transform::new(&p_src, f_src, &p_dst, pixel_format_wyjsciowy, Intent::Perceptual)
    //     .map_err(|_| std::io::Error::other("LCMS2: Nie można powiązać formatów lub profili"))?;
    let tr = if zrodlo_ma_alpha && cel_wymaga_alpha {
        // Jeśli oba mają Alphę, jawnie odpalamy wersję z flagą COPY_ALPHA
        Transform::new_flags(
            &p_src,
            f_src,
            &p_dst,
            pixel_format_wyjsciowy,
            Intent::Perceptual,
            lcms2::Flags::COPY_ALPHA
        )
    } else {
        // Jeśli źródło nie ma Alphy (lub cel jej nie chce), używamy standardowego konstruktora.
        // Pod maską lcms2 przekaże do biblioteki C wartość 0 jako flagi.
        Transform::new(
            &p_src,
            f_src,
            &p_dst,
            pixel_format_wyjsciowy,
            Intent::Perceptual
        )
    }.map_err(|e| std::io::Error::other(format!("LCMS2: Nie można powiązać formatów lub profili: {}", e)))?;

    // 5. Przygotowanie bufora pod format wyjściowy (np. CMYK8)
    let szerokosc = img.width() as usize;
    let wysokosc = img.height() as usize;
    let bajty_na_piksel = pixel_format_wyjsciowy.bytes_per_pixel();

    let mut bufor_wyjsciowy = vec![0u8; szerokosc * wysokosc * bajty_na_piksel];

    // 6. Transformacja pikseli
    tr.transform_pixels(img.as_bytes(), &mut bufor_wyjsciowy);

    Ok(bufor_wyjsciowy)
}


const WP_D65: CIExyY = CIExyY { x: 0.3127, y: 0.3290, Y: 1.0 };
const WP_C: CIExyY   = CIExyY { x: 0.3101, y: 0.3162, Y: 1.0 };

/// Convert 'nclx' profile
pub fn profil_z_nclx(nclx: &ColorNclx) -> Result<Profile, std::io::Error> {

    //  Mapowanie Primaries (Współrzędne kolorów) oraz Punktu Bieli
    let (bialy_i_kolory, punkt_bieli) = match nclx.primaries {

        // BT.709_5 / sRGB
        ColorPrimaries::ITU_R_BT_709_5 | ColorPrimaries::Unspecified => {
            let triple = CIExyYTRIPLE {
                Red:   CIExyY { x: 0.640, y: 0.330, Y: 1.0 },
                Green: CIExyY { x: 0.300, y: 0.600, Y: 1.0 },
                Blue:  CIExyY { x: 0.150, y: 0.060, Y: 1.0 },
            };
            (triple, WP_D65)
        },

        // BT.2020 / BT.2100 (Szeroki gamut, HDR, 4K/8K UltraHD)
        ColorPrimaries::ITU_R_BT_2020_2_and_2100_0 => {
            let triple = CIExyYTRIPLE {
                Red:   CIExyY { x: 0.708, y: 0.292, Y: 1.0 },
                Green: CIExyY { x: 0.170, y: 0.797, Y: 1.0 },
                Blue:  CIExyY { x: 0.131, y: 0.046, Y: 1.0 },
            };
            (triple, WP_D65)
        },

        // DCI-P3 / Display P3 (Apple, nowoczesne smartfony, kina)
        // W libheif SMPTE_EG_432_1 to zazwyczaj profil DCI-P3 z punktem bieli D65 (Display P3)
        ColorPrimaries::SMPTE_EG_432_1 => {
            let triple = CIExyYTRIPLE {
                Red:   CIExyY { x: 0.680, y: 0.320, Y: 1.0 },
                Green: CIExyY { x: 0.265, y: 0.690, Y: 1.0 },
                Blue:  CIExyY { x: 0.150, y: 0.060, Y: 1.0 },
            };
            (triple, WP_D65)
        },

        // BT.601 / PAL (Stara telewizja SD w Europie)
        ColorPrimaries::ITU_R_BT_470_6_System_B_G | ColorPrimaries::ITU_R_BT_601_6 => {
            let triple = CIExyYTRIPLE {
                Red:   CIExyY { x: 0.640, y: 0.330, Y: 1.0 },
                Green: CIExyY { x: 0.290, y: 0.600, Y: 1.0 },
                Blue:  CIExyY { x: 0.150, y: 0.060, Y: 1.0 },
            };
            (triple, WP_D65)
        },

        // BT.601 / NTSC (Stara telewizja SD w USA/Japonii)
        ColorPrimaries::ITU_R_BT_470_6_System_M => {
            let triple = CIExyYTRIPLE {
                Red:   CIExyY { x: 0.670, y: 0.330, Y: 1.0 },
                Green: CIExyY { x: 0.210, y: 0.710, Y: 1.0 },
                Blue:  CIExyY { x: 0.140, y: 0.080, Y: 1.0 },
            };
            (triple, WP_C) // System M używał Illuminant C jako punktu bieli
        },

        // Cała reszta egzotyki leci jako fallback do standardowego sRGB, żeby nie wywalić paniki
        _ => {
            let triple = CIExyYTRIPLE {
                Red:   CIExyY { x: 0.640, y: 0.330, Y: 1.0 },
                Green: CIExyY { x: 0.300, y: 0.600, Y: 1.0 },
                Blue:  CIExyY { x: 0.150, y: 0.060, Y: 1.0 },
            };
            (triple, WP_D65)
        }
    };

    // Mapowanie Transfer Characteristics (Krzywa Gamma)
    // lcms2::ToneCurve przyjmuje wartość gamy (np. 2.2).
    // Jeśli potrzebujesz idealnego sRGB, lcms2 potrafi budować krzywe parametryczne,
    // ale czysta wartość 2.2 lub 2.4 dla wideo w zupełności wystarcza, by kolory nie były wyprane.
    let gamma_value = match nclx.transfer {
        // Liniowy podgląd danych
        TransferCharacteristics::Linear => 1.0,

        // Klasyczny standard sRGB (oficjalnie to krzywa składana, ale czyste 2.2 daje świetny wynik)
        TransferCharacteristics::IEC_61966_2_1 | TransferCharacteristics::Unspecified => 2.2,

        // Starsze standardy wideo (np. PAL) używały często wyższej gammy wyjściowej
        TransferCharacteristics::ITU_R_BT_470_6_System_B_G => 2.8,
        TransferCharacteristics::ITU_R_BT_470_6_System_M => 2.2,

        // Standardy telewizyjne i wideo SD/HD/UHD
        // Oficjalny wykładnik matematyczny to ok. 1.95 ze względu na liniowy segment przy czerni,
        // ale w rzeczywistości ekrany renderują to bliżej 2.2/2.4. Użycie 1.95 zapobiega podbiciu cieni.
        TransferCharacteristics::ITU_R_BT_709_5 |
        TransferCharacteristics::ITU_R_BT_601_6 |
        TransferCharacteristics::ITU_R_BT_2020_2_10bit |
        TransferCharacteristics::ITU_R_BT_2020_2_12bit |
        TransferCharacteristics::ITU_R_BT_1361 |
        TransferCharacteristics::IEC_61966_2_4 |
        TransferCharacteristics::SMPTE_240M => 1.95,

        // Profile HDR (PQ oraz HLG) lub kinowe (XYZ - ST_428_1)
        // Little CMS nie obsłuży ich poprawnie za pomocą zwykłego wykładnika potęgowego.
        // Dajemy tutaj bezpieczny fallback sRGB (2.2), żeby obraz nie wyszedł kompletnie czarny lub zmasakrowany.
        TransferCharacteristics::ITU_R_BT_2100_0_PQ |
        TransferCharacteristics::ITU_R_BT_2100_0_HLG |
        TransferCharacteristics::SMPTE_ST_428_1 |
        TransferCharacteristics::Logarithmic100 |
        TransferCharacteristics::Logarithmic100Sqrt10 |
        TransferCharacteristics::Unknown => 2.2,
    };

    let gamma_curve = ToneCurve::new(gamma_value);

    // profil w locie
    // Podajemy 3 takie same krzywe (osobno dla kanału R, G, B)
    let profil = Profile::new_rgb(
        &punkt_bieli,
        &bialy_i_kolory,
        &[&gamma_curve.clone(), &gamma_curve.clone(), &gamma_curve]
    ).map_err(|_| std::io::Error::other("Nie udało się wygenerować profilu ICC z danych NCLX"))?;

    Ok(profil)
}