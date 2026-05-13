use crate::halper::{usun_kanal_alpha, zaszumianie};
use crate::send::wyslij_status;
use enumy::przetwarzanie::{DaneDoPrzetwarzania, PrzetwarzanieJpg};
use enumy::rozszerzenia::bdepth::BdepthJpg;
use enumy::rozszerzenia::kolor::{ForJpgQuant, ForJpgSamplingFac};
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use image::imageops::FilterType;
use image::DynamicImage;
use jpeg_encoder::{Encoder, QuantizationTableType, SamplingFactor};
use std::fs::{create_dir_all, File};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn jpg_match<T>(
    dane: PrzetwarzanieJpg,
    wymiar: u32,
    bit_depth: BdepthJpg,
    nazwa_wariantu: String,
    filtr: FilterType,
    metryka_operacji: Option<u32>,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<T>,
) -> Result<(), tokio::io::Error>
    where T: Logi,
{
    let (final_img, nazwa_bd) = match bit_depth {
        BdepthJpg::Luma8 => (
            {
                if wymiar == 0 {
                    DynamicImage::ImageLuma8(
                        usun_kanal_alpha(dane.bufor().clone(), dane.alpha).to_luma8(),
                    )
                } else {
                    DynamicImage::ImageLuma8(
                        usun_kanal_alpha(dane.bufor().clone(), dane.alpha).to_luma8(),
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
                        usun_kanal_alpha(dane.bufor().clone(), dane.alpha).to_rgb8(),
                    )
                } else {
                    DynamicImage::ImageRgb8(
                        usun_kanal_alpha(dane.bufor().clone(), dane.alpha).to_rgb8(),
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


    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;




    let final_finalv3_temp_final_ostatecznyv5 = match dane.zaszumienie {
        Some(xoxo) => zaszumianie(xoxo, final_img),
        None => final_img,
    };

    let finalna_nazwa = format!("{}{}{}.jpg", dane.nazwa, nazwa_wariantu, nazwa_bd);
    let mut ścieżka_pliku = dane.sciezka_wyjsciowa.to_path_buf();
    // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
    if !ścieżka_pliku.exists() {
        create_dir_all(ścieżka_pliku.clone())?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    let mut output_file = File::create(&ścieżka_pliku)?;


    let width = final_finalv3_temp_final_ostatecznyv5.width() as u16;
    let height = final_finalv3_temp_final_ostatecznyv5.height() as u16;

    let quant = match dane.quant{
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

    let samplerrr = match dane.sampling{
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




    let mut encoder = Encoder::new(&mut output_file, dane.jakosc);
    encoder.set_sampling_factor(samplerrr);
    encoder.set_quantization_tables(quant.clone(), quant);
    encoder.set_progressive(dane.progresywny);
    encoder.set_progressive_scans(dane.skany ); //Number of scans must be between 2 and 64. There is at least one scan for the DC coefficients and one for the remaining 63 AC coefficients.
    encoder.set_optimized_huffman_tables(dane.progresywny);



    encoder
        .encode(
            final_finalv3_temp_final_ostatecznyv5.as_bytes(),
            width,
            height,
            kolorrrr,
        )
        .map_err(std::io::Error::other)?;

    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;

    Ok(())
}
