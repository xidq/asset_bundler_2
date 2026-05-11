use std::fs::{create_dir_all, File};
use std::path::Path;
use image::DynamicImage;
use image::imageops::FilterType;
use jpeg_encoder::{Encoder, QuantizationTableType, SamplingFactor};
use enumy::rozszerzenia::bdepth::BdepthJpg;
use enumy::rozszerzenia::kolor::{ForJpgQuant, ForJpgSamplingFac};
use crate::halper::{usun_kanal_alpha, zaszumianie};

pub async fn jpg_match(
    mut bufor: &DynamicImage,
    wymiar: u32,
    filtr:FilterType,
    alfa_rgb:&(u16, u16, u16),
    bit_depth: &BdepthJpg,
) -> Result<(DynamicImage,String), tokio::io::Error> {
    let (final_img, nazwa_bd) = match bit_depth {
        BdepthJpg::Luma8 => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageLuma8(
                        usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_luma8(),
                    )
                } else {
                    DynamicImage::ImageLuma8(
                        usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_luma8(),
                    )
                        .resize(
                            wymiar,
                            wymiar,
                            filtr,
                        )
                }
            },
            "_l8b"
        ),
        BdepthJpg::Rgb8 => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageRgb8(
                        usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8(),
                    )
                } else {
                    DynamicImage::ImageRgb8(
                        usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8(),
                    )
                        .resize(
                            wymiar,
                            wymiar,
                            filtr,
                        )
                }
            },
            "_8b"
        ),
    };
    Ok((final_img, nazwa_bd.to_string()))
}

pub async fn jpg_zapis(
    fotu: DynamicImage,
    nazwa_bd: String,
    ścieżka_wyjściowa: &Path,
    do_zaszumienia: Option<u8>,
    nazwa_pliku: &str,
    nazwa_wariantu: &str,
    jakosc : &u8,
    progressif : &bool,
    samplerrrz:&ForJpgSamplingFac,
    quwanttt: &ForJpgQuant,
    skanyyy : &u8
) -> Result<(), tokio::io::Error> {

    let final_finalv3_temp_final_ostatecznyv5 = match do_zaszumienia {
        Some(xoxo) => zaszumianie(xoxo, fotu),
        None => fotu,
    };
    // let mut raw_img = match wybór{
    //     OptFormatyKoloruObrazOgólny::L8 => {
    //         final_finalv3_temp_final_ostatecznyv5.to_luma8().into_raw()
    //     }
    //     _ => {final_finalv3_temp_final_ostatecznyv5.to_rgb8().into_raw()}
    // };

    // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
    let finalna_nazwa = format!("{}{}{}.jpg", nazwa_pliku, nazwa_wariantu, nazwa_bd);
    let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
    // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
    if !ścieżka_pliku.exists() {
        create_dir_all(ścieżka_pliku.clone())?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    let mut output_file = File::create(&ścieżka_pliku)?;

    // let mut encoder =
    //     image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output_file, *jakość);

    // if let Some(data) = exif_data {
    //     // Używamy metody z traitu ImageEncoder
    //     // Ważne: musisz mieć 'use image::ImageEncoder;' na górze pliku!
    //     let _ = encoder.set_exif_metadata(data.clone());
    // }

    // encoder.encode_image(&final_finalv3_temp_final_ostatecznyv5).map_err(std::io::Error::other)?;
    let width = final_finalv3_temp_final_ostatecznyv5.width() as u16;
    let height = final_finalv3_temp_final_ostatecznyv5.height() as u16;
    // let color = final_finalv3_temp_final_ostatecznyv5.color().into(); // Konwersja na ExtendedColorType


    // 4. Zapisujemy obraz używając write_image (metoda z traitu ImageEncoder)
    // encoder
    //     .write_image(
    //         final_finalv3_temp_final_ostatecznyv5.as_bytes(),
    //         width,
    //         height,
    //         color,
    //     )
    //     .map_err(std::io::Error::other)?;
    let quant = match quwanttt{
        ForJpgQuant::Default => {QuantizationTableType::Default}
        ForJpgQuant::Flat => {QuantizationTableType::Flat}
        ForJpgQuant::CustomMsSsim => {QuantizationTableType::CustomMsSsim}
        ForJpgQuant::CustomPsnrHvs => {QuantizationTableType::CustomPsnrHvs}
        ForJpgQuant::ImageMagick => {QuantizationTableType::ImageMagick}
        ForJpgQuant::KleinSilversteinCarney => {QuantizationTableType::KleinSilversteinCarney}
        ForJpgQuant::DentalXRays => {QuantizationTableType::DentalXRays}
        ForJpgQuant::VisualDetectionModel => {QuantizationTableType::VisualDetectionModel}
        ForJpgQuant::ImprovedDetectionModel => {QuantizationTableType::ImprovedDetectionModel}
    };

    let samplerrr = match samplerrrz{
        ForJpgSamplingFac::R444 => {SamplingFactor::R_4_4_4}
        ForJpgSamplingFac::R440 => {SamplingFactor::R_4_4_0}
        ForJpgSamplingFac::R441 => {SamplingFactor::R_4_4_1}
        ForJpgSamplingFac::R422 => {SamplingFactor::R_4_2_2 }
        ForJpgSamplingFac::R420 => {SamplingFactor::R_4_2_0}
        ForJpgSamplingFac::R421 => {SamplingFactor::R_4_2_1}
        ForJpgSamplingFac::R411 => {SamplingFactor::R_4_1_1}
        ForJpgSamplingFac::R410 => {SamplingFactor::R_4_1_0}
    };
    let kolorrrr = match final_finalv3_temp_final_ostatecznyv5.color(){
        image::ColorType::L8 => {jpeg_encoder::ColorType::Luma},
        _ => {jpeg_encoder::ColorType::Rgb}

    };




    let mut encoder = Encoder::new(&mut output_file, *jakosc);
    encoder.set_sampling_factor(samplerrr);
    encoder.set_quantization_tables(quant.clone(), quant);
    encoder.set_progressive(*progressif);
    encoder.set_progressive_scans(* skanyyy ); //Number of scans must be between 2 and 64. There is at least one scan for the DC coefficients and one for the remaining 63 AC coefficients.
    encoder.set_optimized_huffman_tables(*progressif);

    encoder
        .encode(
            final_finalv3_temp_final_ostatecznyv5.as_bytes(),
            width,
            height,
            kolorrrr,
        )
        .map_err(std::io::Error::other)?;



    Ok(())

}