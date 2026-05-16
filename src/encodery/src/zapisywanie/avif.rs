use crate::halper::{konwersja_float_na_mniejsze, ogarnij_icc, usun_kanal_alpha};
use crate::send::wyslij_status;
use enumy::przetwarzanie::{DaneDoPrzetwarzania, PrzetwarzanieAvif};
use enumy::rozszerzenia::bdepth::BdepthAvif;
use enumy::rozszerzenia::kolor::{ColorProfilePhoto, ForAvifChroma};
use enumy::rozszerzenia::kompresje::ForAvifKompresja;
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use image::imageops::FilterType;
use libheif_rs::{Channel, ColorSpace, CompressionFormat, EncoderParameterValue, EncoderQuality, HeifContext, Image, LibHeif, RgbChroma};
use std::fs::create_dir_all;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn avif_match<T>(
    dane: PrzetwarzanieAvif,
    wymiar: u32,
    bit_depth: BdepthAvif,
    nazwa_wariantu: String,
    filtr: FilterType,
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

    let reskalowanie = if wymiar == 0 {
            fotu
    } else {
        fotu
            .resize(
                wymiar,
                wymiar,
                filtr,
            )
    };
    let (heif_img, nazwa_bd) = match bit_depth {
        BdepthAvif::Rgb12Alpha =>
            {

                let przekonwertowane_bajty_u8 = ogarnij_icc(
                    icc.clone(),
                    &reskalowanie,
                    lcms2::PixelFormat::RGBA_16 // <--- Wymuszamy 16-bitów na kanał z LCMS2
                );
                // dbg!("jestem w B10a");
                let res = reskalowanie.to_rgba16();

                // dbg!("ogarnięto Rgba16Image");
                let (w, h) = res.dimensions();

                // let raw_u16 = res.into_raw();
                let raw_u16: Vec<u16> = przekonwertowane_bajty_u8
                    .chunks_exact(2)
                    .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                    .collect();

                //obraz RGBA
                let mut heif_img = Image::new(w, h, ColorSpace::Rgb(RgbChroma::C444))
                    .map_err(std::io::Error::other)?;

                //4 osobne płaszczyzny po 10 bitów każda
                heif_img.create_plane(Channel::R, w, h, 12).expect("Plane R fail");
                heif_img.create_plane(Channel::G, w, h, 12).expect("Plane G fail");
                heif_img.create_plane(Channel::B, w, h, 12).expect("Plane B fail");
                heif_img.create_plane(Channel::Alpha, w, h, 12).expect("Plane A fail");

                {
                    let planes = heif_img.planes_mut();

                    let stride = planes.r.as_ref().unwrap().stride;
                    let data_r = planes.r.unwrap().data;
                    let data_g = planes.g.unwrap().data;
                    let data_b = planes.b.unwrap().data;
                    let data_a = planes.a.unwrap().data;

                    // Przechodzimy przez obraz rząd po rzędzie.
                    // raw_u16.chunks_exact(w * 4) daje nam dokładnie jeden rząd RGBA na raz.
                    for (y, row_src) in raw_u16.chunks_exact(w as usize * 4).enumerate() {
                        let row_offset = y * stride;

                        // Wycinamy dokładny fragment pamięci (slice) dla całego rzędu w danym kanale.
                        // Dzięki temu kompilator usuwa sprawdzanie zakresów w wewnętrznej pętli.
                        let row_r = &mut data_r[row_offset .. row_offset + w as usize * 2];
                        let row_g = &mut data_g[row_offset .. row_offset + w as usize * 2];
                        let row_b = &mut data_b[row_offset .. row_offset + w as usize * 2];
                        let row_a = &mut data_a[row_offset .. row_offset + w as usize * 2];

                        // row_src.chunks_exact(4) iteruje po pikselach: [R, G, B, A]
                        for (x, pixel) in row_src.chunks_exact(4).enumerate() {
                            let dest_idx = x * 2;

                            // Przesunięcie i konwersja (16-bit -> 12-bit)
                            let r10 = pixel[0] >> 4;
                            let g10 = pixel[1] >> 4;
                            let b10 = pixel[2] >> 4;
                            let a10 = pixel[3] >> 4;

                            // Zapis do 2 bajtów jako Little Endian (bez ręcznego przesuwania i maskowania)
                            row_r[dest_idx..dest_idx + 2].copy_from_slice(&r10.to_le_bytes());
                            row_g[dest_idx..dest_idx + 2].copy_from_slice(&g10.to_le_bytes());
                            row_b[dest_idx..dest_idx + 2].copy_from_slice(&b10.to_le_bytes());
                            row_a[dest_idx..dest_idx + 2].copy_from_slice(&a10.to_le_bytes());
                        }
                    }
                    // dbg!("Bufor Planar wypełniony zoptymalizowaną metodą. Przed encode_image");
                }
                (heif_img,"_12a")
            }
        BdepthAvif::Rgb12 =>
            {
                let res = usun_kanal_alpha(reskalowanie, dane.alpha);
                // dbg!("jestem w B10");
                let przekonwertowane_bajty_u8 = ogarnij_icc(
                    icc.clone(),
                    &res,
                    lcms2::PixelFormat::RGB_16 // <--- Wymuszamy 16-bitów na kanał z LCMS2
                );



                // dbg!("ogarnięto Rgb16Image");
                let (w, h) = res.to_rgb16().dimensions();
                // let raw_u16 = res.into_raw();
                let raw_u16: Vec<u16> = przekonwertowane_bajty_u8
                    .chunks_exact(2)
                    .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                    .collect();

                //
                let mut heif_img = Image::new(w, h, ColorSpace::Rgb(RgbChroma::C444))
                    .map_err(std::io::Error::other)?;

                //4 osobne płaszczyzny po 10 bitów każda
                heif_img.create_plane(Channel::R, w, h, 12).expect("Plane R fail");
                heif_img.create_plane(Channel::G, w, h, 12).expect("Plane G fail");
                heif_img.create_plane(Channel::B, w, h, 12).expect("Plane B fail");

                {
                    let planes = heif_img.planes_mut();

                    let stride = planes.r.as_ref().unwrap().stride;
                    let data_r = planes.r.unwrap().data;
                    let data_g = planes.g.unwrap().data;
                    let data_b = planes.b.unwrap().data;

                    // Przechodzimy przez obraz rząd po rzędzie.
                    // raw_u16.chunks_exact(w * 4) daje nam dokładnie jeden rząd RGBA na raz.
                    for (y, row_src) in raw_u16.chunks_exact(w as usize * 3).enumerate() {
                        let row_offset = y * stride;

                        // Wycinamy dokładny fragment pamięci (slice) dla całego rzędu w danym kanale.
                        // Dzięki temu kompilator usuwa sprawdzanie zakresów w wewnętrznej pętli.
                        let row_r = &mut data_r[row_offset .. row_offset + w as usize * 2];
                        let row_g = &mut data_g[row_offset .. row_offset + w as usize * 2];
                        let row_b = &mut data_b[row_offset .. row_offset + w as usize * 2];

                        // row_src.chunks_exact(4) iteruje po pikselach: [R, G, B, A]
                        for (x, pixel) in row_src.chunks_exact(3).enumerate() {
                            let dest_idx = x * 2;

                            // Przesunięcie i konwersja (16-bit -> 10-bit)
                            let r10 = pixel[0] >> 4;
                            let g10 = pixel[1] >> 4;
                            let b10 = pixel[2] >> 4;

                            // Zapis do 2 bajtów jako Little Endian (bez ręcznego przesuwania i maskowania)
                            row_r[dest_idx..dest_idx + 2].copy_from_slice(&r10.to_le_bytes());
                            row_g[dest_idx..dest_idx + 2].copy_from_slice(&g10.to_le_bytes());
                            row_b[dest_idx..dest_idx + 2].copy_from_slice(&b10.to_le_bytes());
                        }
                    }
                    // dbg!("Bufor Planar wypełniony zoptymalizowaną metodą. Przed encode_image");
                }
                (heif_img, "_12")
            }



        BdepthAvif::Rgb10Alpha =>
            {

                let przekonwertowane_bajty_u8 = ogarnij_icc(
                    icc.clone(),
                    &reskalowanie,
                    lcms2::PixelFormat::RGBA_16 // <--- Wymuszamy 16-bitów na kanał z LCMS2
                );
                // dbg!("jestem w B10a");
                let res = reskalowanie.to_rgba16();

                // dbg!("ogarnięto Rgba16Image");
                let (w, h) = res.dimensions();

                // let raw_u16 = res.into_raw();
                let raw_u16: Vec<u16> = przekonwertowane_bajty_u8
                    .chunks_exact(2)
                    .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                    .collect();

                //obraz RGBA
                let mut heif_img = Image::new(w, h, ColorSpace::Rgb(RgbChroma::C444))
                    .map_err(std::io::Error::other)?;

                //4 osobne płaszczyzny po 10 bitów każda
                heif_img.create_plane(Channel::R, w, h, 10).expect("Plane R fail");
                heif_img.create_plane(Channel::G, w, h, 10).expect("Plane G fail");
                heif_img.create_plane(Channel::B, w, h, 10).expect("Plane B fail");
                heif_img.create_plane(Channel::Alpha, w, h, 10).expect("Plane A fail");

                {
                    let planes = heif_img.planes_mut();

                    let stride = planes.r.as_ref().unwrap().stride;
                    let data_r = planes.r.unwrap().data;
                    let data_g = planes.g.unwrap().data;
                    let data_b = planes.b.unwrap().data;
                    let data_a = planes.a.unwrap().data;

                    // Przechodzimy przez obraz rząd po rzędzie.
                    // raw_u16.chunks_exact(w * 4) daje nam dokładnie jeden rząd RGBA na raz.
                    for (y, row_src) in raw_u16.chunks_exact(w as usize * 4).enumerate() {
                        let row_offset = y * stride;

                        // Wycinamy dokładny fragment pamięci (slice) dla całego rzędu w danym kanale.
                        // Dzięki temu kompilator usuwa sprawdzanie zakresów w wewnętrznej pętli.
                        let row_r = &mut data_r[row_offset .. row_offset + w as usize * 2];
                        let row_g = &mut data_g[row_offset .. row_offset + w as usize * 2];
                        let row_b = &mut data_b[row_offset .. row_offset + w as usize * 2];
                        let row_a = &mut data_a[row_offset .. row_offset + w as usize * 2];

                        // row_src.chunks_exact(4) iteruje po pikselach: [R, G, B, A]
                        for (x, pixel) in row_src.chunks_exact(4).enumerate() {
                            let dest_idx = x * 2;

                            // Przesunięcie i konwersja (16-bit -> 10-bit)
                            let r10 = pixel[0] >> 6;
                            let g10 = pixel[1] >> 6;
                            let b10 = pixel[2] >> 6;
                            let a10 = pixel[3] >> 6;

                            // Zapis do 2 bajtów jako Little Endian (bez ręcznego przesuwania i maskowania)
                            row_r[dest_idx..dest_idx + 2].copy_from_slice(&r10.to_le_bytes());
                            row_g[dest_idx..dest_idx + 2].copy_from_slice(&g10.to_le_bytes());
                            row_b[dest_idx..dest_idx + 2].copy_from_slice(&b10.to_le_bytes());
                            row_a[dest_idx..dest_idx + 2].copy_from_slice(&a10.to_le_bytes());
                        }
                    }
                    // dbg!("Bufor Planar wypełniony zoptymalizowaną metodą. Przed encode_image");
                }
                (heif_img,"_10a")
            }
        BdepthAvif::Rgb8Alpha =>
            {
                let przekonwertowane_bajty_u8 = ogarnij_icc(
                    icc.clone(),
                    &reskalowanie,
                    lcms2::PixelFormat::RGBA_8 // <--- Wymuszamy 16-bitów na kanał z LCMS2
                );
                // dbg!("jestem w B8a");
                // let res = reskalowanie.to_rgba8();

                // dbg!("ogarnięto Rgba8Image");
                let (w, h) = reskalowanie.to_rgba8().dimensions();
                // let raw_u8 = res.into_raw();
                let raw_u8: Vec<u8> = przekonwertowane_bajty_u8;

                //
                let mut heif_img = Image::new(w, h, ColorSpace::Rgb(RgbChroma::C444))
                    .map_err(std::io::Error::other)?;

                //4 osobne płaszczyzny po 10 bitów każda
                heif_img.create_plane(Channel::R, w, h, 8).expect("Plane R fail");
                heif_img.create_plane(Channel::G, w, h, 8).expect("Plane G fail");
                heif_img.create_plane(Channel::B, w, h, 8).expect("Plane B fail");
                heif_img.create_plane(Channel::Alpha, w, h, 8).expect("Plane A fail");

                {
                    let planes = heif_img.planes_mut();

                    let stride = planes.r.as_ref().unwrap().stride;
                    let data_r = planes.r.unwrap().data;
                    let data_g = planes.g.unwrap().data;
                    let data_b = planes.b.unwrap().data;
                    let data_a = planes.a.unwrap().data;

                    // Przechodzimy przez obraz rząd po rzędzie
                    for (y, row_src) in raw_u8.chunks_exact(w as usize * 4).enumerate() {
                        let row_offset = y * stride;

                        // Wycinamy plasterki o długości dokładnie 'w' (bo to 1 bajt na piksel)
                        let row_r = &mut data_r[row_offset .. row_offset + w as usize];
                        let row_g = &mut data_g[row_offset .. row_offset + w as usize];
                        let row_b = &mut data_b[row_offset .. row_offset + w as usize];
                        let row_a = &mut data_a[row_offset .. row_offset + w as usize];

                        for (x, pixel) in row_src.chunks_exact(4).enumerate() {
                            // Bezpośrednie kopiowanie bajtu 1:1, bez rzutowania i kombinowania
                            row_r[x] = pixel[0];
                            row_g[x] = pixel[1];
                            row_b[x] = pixel[2];
                            row_a[x] = pixel[3];
                        }
                    }
                    // dbg!("Bufor Planar 8-bit wypełniony zoptymalizowaną metodą. Przed encode_image");
                }
                (heif_img,"_8a")
            }

        BdepthAvif::Rgb10 =>
            {
                let res = usun_kanal_alpha(reskalowanie, dane.alpha);
                // dbg!("jestem w B10");
                let przekonwertowane_bajty_u8 = ogarnij_icc(
                    icc.clone(),
                    &res,
                    lcms2::PixelFormat::RGB_16 // <--- Wymuszamy 16-bitów na kanał z LCMS2
                );


                // dbg!("ogarnięto Rgb16Image");
                let (w, h) = res.to_rgb16().dimensions();
                // let raw_u16 = res.into_raw();
                let raw_u16: Vec<u16> = przekonwertowane_bajty_u8
                    .chunks_exact(2)
                    .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                    .collect();

                //
                let mut heif_img = Image::new(w, h, ColorSpace::Rgb(RgbChroma::C444))
                    .map_err(std::io::Error::other)?;

                //4 osobne płaszczyzny po 10 bitów każda
                heif_img.create_plane(Channel::R, w, h, 10).expect("Plane R fail");
                heif_img.create_plane(Channel::G, w, h, 10).expect("Plane G fail");
                heif_img.create_plane(Channel::B, w, h, 10).expect("Plane B fail");

                {
                    let planes = heif_img.planes_mut();

                    let stride = planes.r.as_ref().unwrap().stride;
                    let data_r = planes.r.unwrap().data;
                    let data_g = planes.g.unwrap().data;
                    let data_b = planes.b.unwrap().data;

                    // Przechodzimy przez obraz rząd po rzędzie.
                    // raw_u16.chunks_exact(w * 4) daje nam dokładnie jeden rząd RGBA na raz.
                    for (y, row_src) in raw_u16.chunks_exact(w as usize * 3).enumerate() {
                        let row_offset = y * stride;

                        // Wycinamy dokładny fragment pamięci (slice) dla całego rzędu w danym kanale.
                        // Dzięki temu kompilator usuwa sprawdzanie zakresów w wewnętrznej pętli.
                        let row_r = &mut data_r[row_offset .. row_offset + w as usize * 2];
                        let row_g = &mut data_g[row_offset .. row_offset + w as usize * 2];
                        let row_b = &mut data_b[row_offset .. row_offset + w as usize * 2];

                        // row_src.chunks_exact(4) iteruje po pikselach: [R, G, B, A]
                        for (x, pixel) in row_src.chunks_exact(3).enumerate() {
                            let dest_idx = x * 2;

                            // Przesunięcie i konwersja (16-bit -> 10-bit)
                            let r10 = pixel[0] >> 6;
                            let g10 = pixel[1] >> 6;
                            let b10 = pixel[2] >> 6;

                            // Zapis do 2 bajtów jako Little Endian (bez ręcznego przesuwania i maskowania)
                            row_r[dest_idx..dest_idx + 2].copy_from_slice(&r10.to_le_bytes());
                            row_g[dest_idx..dest_idx + 2].copy_from_slice(&g10.to_le_bytes());
                            row_b[dest_idx..dest_idx + 2].copy_from_slice(&b10.to_le_bytes());
                        }
                    }
                    // dbg!("Bufor Planar wypełniony zoptymalizowaną metodą. Przed encode_image");
                }
                (heif_img, "_10")
            }
        BdepthAvif::Rgb8 =>
            {
                let res = usun_kanal_alpha(reskalowanie, dane.alpha);
                let przekonwertowane_bajty_u8 = ogarnij_icc(
                    icc.clone(),
                    &res,
                    lcms2::PixelFormat::RGB_8 // <--- Wymuszamy 16-bitów na kanał z LCMS2
                );
                // dbg!("jestem w B8");


                // dbg!("ogarnięto Rgb8Image");
                let (w, h) = res.to_rgb8().dimensions();
                let raw_u8 = przekonwertowane_bajty_u8;

                //
                let mut heif_img = Image::new(w, h, ColorSpace::Rgb(RgbChroma::C444))
                    .map_err(std::io::Error::other)?;

                //4 osobne płaszczyzny po 10 bitów każda
                heif_img.create_plane(Channel::R, w, h, 8).expect("Plane R fail");
                heif_img.create_plane(Channel::G, w, h, 8).expect("Plane G fail");
                heif_img.create_plane(Channel::B, w, h, 8).expect("Plane B fail");

                {
                    let planes = heif_img.planes_mut();

                    let stride = planes.r.as_ref().unwrap().stride;
                    let data_r = planes.r.unwrap().data;
                    let data_g = planes.g.unwrap().data;
                    let data_b = planes.b.unwrap().data;

                    // Przechodzimy przez obraz rząd po rzędzie
                    for (y, row_src) in raw_u8.chunks_exact(w as usize * 3).enumerate() {
                        let row_offset = y * stride;

                        // Wycinamy plasterki o długości dokładnie 'w' (bo to 1 bajt na piksel)
                        let row_r = &mut data_r[row_offset .. row_offset + w as usize];
                        let row_g = &mut data_g[row_offset .. row_offset + w as usize];
                        let row_b = &mut data_b[row_offset .. row_offset + w as usize];

                        for (x, pixel) in row_src.chunks_exact(3).enumerate() {
                            // Bezpośrednie kopiowanie bajtu 1:1, bez rzutowania i kombinowania
                            row_r[x] = pixel[0];
                            row_g[x] = pixel[1];
                            row_b[x] = pixel[2];
                        }
                    }
                    // dbg!("Bufor Planar 8-bit wypełniony zoptymalizowaną metodą. Przed encode_image");
                }
                (heif_img, "_8")
            }

    };
    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;

    let qual = match dane.lossy{
        None => {EncoderQuality::LossLess}
        Some(x) => {EncoderQuality::Lossy(x)}
    };

    let kompresja = match dane.metoda_kompresji{
        ForAvifKompresja::Undefined => {CompressionFormat::Undefined}
        ForAvifKompresja::Hevc => {CompressionFormat::Hevc}
        ForAvifKompresja::Avc => {CompressionFormat::Avc}
        ForAvifKompresja::Jpeg => {CompressionFormat::Jpeg}
        ForAvifKompresja::Av1 => {CompressionFormat::Av1}
        ForAvifKompresja::Vvc => {CompressionFormat::Vvc}
        ForAvifKompresja::Evc => {CompressionFormat::Evc}
        ForAvifKompresja::Jpeg2000 => {CompressionFormat::Jpeg2000}
        ForAvifKompresja::Uncompressed => {CompressionFormat::Uncompressed}
        ForAvifKompresja::Mask => {CompressionFormat::Mask}
        ForAvifKompresja::HtJ2k => {CompressionFormat::HtJ2k}
    };
    let chroma = match dane.chroma{
        ForAvifChroma::C444 => {"444".to_string()}
        ForAvifChroma::C422 => {"422".to_string()}
        ForAvifChroma::C420 => {"420".to_string()}
    };

    
    let lib = LibHeif::new();
    let mut context = HeifContext::new()
        .map_err(std::io::Error::other)?;

    let mut encoder = lib.encoder_for_format(kompresja)
        .map_err(std::io::Error::other)?;

    encoder.set_quality(qual)
        .map_err(std::io::Error::other)?;

    encoder.set_parameter_value("chroma", EncoderParameterValue::String(chroma)).ok();


    encoder.set_parameter_value("speed", EncoderParameterValue::Int(dane.speed)).ok(); // 0-10 (wolniej = lepsza kompresja)
    encoder.set_parameter_value("tune", EncoderParameterValue::String("ssim".to_string())).ok(); // Optymalizacja pod jakość wizualną
    // encoder.set_parameter_value("exif").ok();

    let handle =context.encode_image(&heif_img, &mut encoder, None).map_err(std::io::Error::other)?;
    // dbg!("Po encode_image");
    if let Some(ref surowy_exif) = dane.exif {
        if !surowy_exif.is_empty() {
            // Specyfikacja HEIF/AVIF wymaga 4 pustych bajtów (offsetu) na początku
            let mut heif_exif_payload = vec![0u8; 4];
            heif_exif_payload.extend_from_slice(surowy_exif);

            // Dodajemy metadane bezpośrednio do kontekstu, parując je z naszym `handle`
            context.add_exif_metadata(
                &handle,
                &heif_exif_payload,
            ).map_err(|e| std::io::Error::other(format!("Błąd zapisu metadanych EXIF: {}", e)))?;
        }
    }

    let final_bytes = context.write_to_bytes()
        .map_err(std::io::Error::other)?;

    let mut output_path = dane.sciezka_wyjsciowa.to_path_buf();

    if !output_path.exists() {
        create_dir_all(output_path.clone())?;
    }
    output_path.push(format!("{}{}{}.avif",dane.nazwa,nazwa_wariantu,nazwa_bd));

    std::fs::write(&output_path, &final_bytes)
        .map_err(|e| std::io::Error::other(format!("Błąd zapisu pliku: {}", e)))?;

    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;



    Ok(())


    
}

// #[allow(clippy::too_many_arguments)]
// pub async fn avif_zapis(
//     heif_img: Image,
//     nazwa_bd: String,
//     ścieżka_wyjściowa: &Path,
//     ścieżka_dopełniająca: &String,
//     nazwa_pliku: &str,
//     lossy: Option<u8>,
//     szybkość:i32,
//     chroma:&ForAvifChroma,
//     metoda_kompresji: &ForAvifKompresja,
// ) -> Result<(), tokio::io::Error> {
// 
//             let qual = match lossy{
//                 None => {EncoderQuality::LossLess}
//                 Some(x) => {EncoderQuality::Lossy(x)}
//             };
// 
//             let kompresja = match metoda_kompresji{
//                 ForAvifKompresja::Undefined => {CompressionFormat::Undefined}
//                 ForAvifKompresja::Hevc => {CompressionFormat::Hevc}
//                 ForAvifKompresja::Avc => {CompressionFormat::Avc}
//                 ForAvifKompresja::Jpeg => {CompressionFormat::Jpeg}
//                 ForAvifKompresja::Av1 => {CompressionFormat::Av1}
//                 ForAvifKompresja::Vvc => {CompressionFormat::Vvc}
//                 ForAvifKompresja::Evc => {CompressionFormat::Evc}
//                 ForAvifKompresja::Jpeg2000 => {CompressionFormat::Jpeg2000}
//                 ForAvifKompresja::Uncompressed => {CompressionFormat::Uncompressed}
//                 ForAvifKompresja::Mask => {CompressionFormat::Mask}
//                 ForAvifKompresja::HtJ2k => {CompressionFormat::HtJ2k}
//             };
//             let chroma = match chroma{
//                 ForAvifChroma::C444 => {"444".to_string()}
//                 ForAvifChroma::C422 => {"422".to_string()}
//                 ForAvifChroma::C420 => {"420".to_string()}
//             };
// 
// 
// 
//             // dbg!("jestem po match z bit_depth");
// 
//             let lib = LibHeif::new();
//             // dbg!("jestem po lib = LibHeif::new();");
//             let mut context = HeifContext::new()
//                 .map_err(std::io::Error::other)?;
// 
//             // dbg!("jestem po let mut context = HeifContext::new()");
//             let mut encoder = lib.encoder_for_format(kompresja)
//                 .map_err(std::io::Error::other)?;
//             // dbg!("jestem po let mut encoder = lib.encoder_for_format(CompressionFormat::Av1)");
// 
//             encoder.set_quality(qual)
//                 .map_err(std::io::Error::other)?;
//             // dbg!("jestem po encoder.set_quality(EncoderQuality::LossLess)");
//             // Dostępny parametr: chroma
//             // Dostępny parametr: quality
//             // Dostępny parametr: realtime
//             // Dostępny parametr: tune
//             // Dostępny parametr: alpha-min-q
//             // Dostępny parametr: lossless-alpha
//             // Dostępny parametr: enable-intrabc
//             // Dostępny parametr: alpha-max-q
//             // Dostępny parametr: max-q
//             // Dostępny parametr: lossless
//             // Dostępny parametr: auto-tiles
//             // Dostępny parametr: min-q
//             // Dostępny parametr: alpha-quality
//             // Dostępny parametr: threads
//             // Dostępny parametr: speed
//             encoder.set_parameter_value("chroma", EncoderParameterValue::String(chroma)).ok();
// 
// 
//             encoder.set_parameter_value("speed", EncoderParameterValue::Int(szybkość)).ok(); // 0-10 (wolniej = lepsza kompresja)
//             encoder.set_parameter_value("tune", EncoderParameterValue::String("ssim".to_string())).ok(); // Optymalizacja pod jakość wizualną
//             // let params = encoder.parameters_names();
//             // for p in params {
//             //     dbg!("Dostępny parametr: {}", p);
//             //
//             // }
//             // dbg!("chroma",encoder.parameter("chroma"));
//             // dbg!("speed",encoder.parameter("speed"));
//             // dbg!("tune",encoder.parameter("tune"));
// 
// 
//             // dbg!("Przed encode_image");
//             context.encode_image(&heif_img, &mut encoder, None).map_err(std::io::Error::other)?;
//             // dbg!("Po encode_image");
// 
// 
//             let final_bytes = context.write_to_bytes()
//                 .map_err(std::io::Error::other)?;
//             // dbg!("jestem po let final_bytes = context.write_to_bytes()");
// 
// 
// 
// 
// 
//             // 1. Przygotowanie ścieżki wyjściowej
//             // Zakładam, że masz dostęp do zmiennej ze ścieżką wejściową, np. `path`
//             let mut output_path = ścieżka_wyjściowa.to_path_buf();
//             output_path.push(ścieżka_dopełniająca);
// 
//             if !output_path.exists() {
//                 create_dir_all(output_path.clone())?;
//             }
//             output_path.push(format!("{}{}{}.avif",nazwa_pliku,nazwa_wariantu,nazwa_bd));
//             // output_path.push(".avif");
//             // Jeśli chcesz dodać sufix (np. obraz_rgba10.avif):
//             // let stem = path.file_stem().unwrap().to_str().unwrap();
//             // let output_path = path.with_file_name(format!("{}{}.avif", stem, suffix));
// 
//             // dbg!("ścieżka wyjściowa: {}", &output_path);
// 
// 
//             // 2. Fizyczny zapis na dysk
//             std::fs::write(&output_path, &final_bytes)
//                 .map_err(|e| std::io::Error::other(format!("Błąd zapisu pliku: {}", e)))?;
// 
// 
// 
// 
//     Ok(())
// }
