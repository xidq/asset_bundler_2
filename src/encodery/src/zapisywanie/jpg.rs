use crate::halper::{konwersja_float_na_mniejsze, usun_kanal_alpha, zaszumianie};
use enumy::send::wyslij_status;
use crate::transform::{konwertuj_przestrzen, profil_z_nclx};
use crate::zapisywanie::generic::{get_higher_tier_copy, InneDane};
use enumy::opcje::OptIstniejePlik;
use enumy::przetwarzanie::{DaneDoPrzetwarzania, PrzetwarzanieJpg};
use enumy::rozszerzenia::bdepth::BdepthJpg;
use enumy::rozszerzenia::kolor::{ColorProfilePhoto, ForJpgQuant, ForJpgSamplingFac};
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use jpeg_encoder::{Encoder, QuantizationTableType, SamplingFactor};
use lcms2::PixelFormat;
use std::fs::{create_dir_all, File};
use std::sync::Arc;
use tokio::sync::Mutex;
use enumy::inne_ui::WskaznikSzumu;

/// # Encoding jpg
pub async fn jpg_match<T>(
    dane: PrzetwarzanieJpg,
    dane2: InneDane<BdepthJpg>,
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

    let final_img = if dane2.wymiar == 0 {
        usun_kanal_alpha(fotu, dane.alpha)
    } else {
        usun_kanal_alpha(fotu, dane.alpha)
            .resize(
                dane2.wymiar,
                dane2.wymiar,
                dane2.filtr,
            )
    };

    let (nazwa_bd, profil, enco) = match dane2.bdepth {
        BdepthJpg::Luma8 => ("_l8b", PixelFormat::GRAY_8, jpeg_encoder::ColorType::Luma),
        BdepthJpg::Rgb8 => ("_8b", PixelFormat::RGB_8, jpeg_encoder::ColorType::Rgb),
    };


    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;




    let final_finalv3_temp_final_ostatecznyv5 = match dane.zaszumienie {
        WskaznikSzumu::Normalny { .. } | WskaznikSzumu::Perlin{ .. } => {zaszumianie(dane.zaszumienie, final_img)}
        WskaznikSzumu::NoNoise => {final_img}
    };

    let finalna_nazwa = format!("{}{}{}.jpg", dane.nazwa, dane2.nazwa_wariantu, nazwa_bd);
    let mut ścieżka_pliku = dane.sciezka_wyjsciowa.to_path_buf();
    // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
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
        ForJpgSamplingFac::R422 => {SamplingFactor::R_4_2_2}
        ForJpgSamplingFac::R420 => {SamplingFactor::R_4_2_0}
        ForJpgSamplingFac::R421 => {SamplingFactor::R_4_2_1}
        ForJpgSamplingFac::R411 => {SamplingFactor::R_4_1_1}
        ForJpgSamplingFac::R410 => {SamplingFactor::R_4_1_0}
    };


    let ungabunga = if let ColorProfilePhoto::ICC(ref xoxo) = icc {
        konwertuj_przestrzen(&final_finalv3_temp_final_ostatecznyv5, Some(xoxo), profil)
            .expect("Błąd konwersji ICC")

    } else if let ColorProfilePhoto::NCLX(ref nclx_data) = icc {
        // profil z NCLX, do bajtów ICC
        let wygenerowany_profil = profil_z_nclx(nclx_data).expect("Błąd generowania profilu z NCLX");
        let icc_bajty = wygenerowany_profil.icc().expect("Błąd serializacji profilu do ICC");

        konwertuj_przestrzen(&final_finalv3_temp_final_ostatecznyv5, Some(&icc_bajty), profil)
            .expect("Błąd konwersji z profilu NCLX")

    } else {
        // Brak profilu (None)
        konwertuj_przestrzen(&final_finalv3_temp_final_ostatecznyv5, None, profil)
            .expect("Błąd konwersji bez profilu")
    };


    let mut encoder = Encoder::new(&mut output_file, dane.jakosc);
    encoder.set_sampling_factor(samplerrr);
    encoder.set_quantization_tables(quant.clone(), quant);
    encoder.set_progressive(dane.progresywny);
    encoder.set_progressive_scans(dane.scans); //Number of scans must be between 2 and 64. There is at least one scan for the DC coefficients and one for the remaining 63 AC coefficients.
    encoder.set_optimized_huffman_tables(dane.progresywny);
    match dane.exif{
        None => {}
        Some(xxx) => { encoder.add_exif_metadata(&xxx).expect("Err jpg exif data");}
    }




    encoder
        .encode(
            &ungabunga,
            width,
            height,
            enco,
        )
        .map_err(std::io::Error::other)?;

    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;

    Ok(())
}
