use crate::zapisy::inne_dds::{aktualizuj_postep_dds, dds_usun_kanal_alpha};

use enumy::statusy::LogTxDoRozpakowanieDds;
use futures::channel::mpsc::Sender;
use image::{DynamicImage, ImageEncoder};
use std::fs::{File, create_dir_all};
use std::path::Path;

pub async fn dds_ex_jpg(
    mut bufor: DynamicImage,
    ścieżka_wyjściowa: &Path,
    nazwa_pliku: &str,
    jakość: &u8,
    alfa_rgb: &(u16, u16, u16),
    metryka_operacji: u32,
    obecna_operacja: &mut u32, // Zmień na &mut u32
    procent_progress: &mut u8,
    mut tx: Sender<LogTxDoRozpakowanieDds>,
) -> Result<(), tokio::io::Error> {
    let (docelowy_wymiar, nazwa_wariantu) = (0, "");

    aktualizuj_postep_dds(obecna_operacja, procent_progress, metryka_operacji, &mut tx).await;

    let final_img =
        DynamicImage::ImageRgb8(dds_usun_kanal_alpha(bufor.clone(), *alfa_rgb).to_rgb8());

    // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
    let finalna_nazwa = format!("{}{}.jpg", nazwa_pliku, nazwa_wariantu);
    let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
    // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
    if !ścieżka_pliku.exists() {
        create_dir_all(ścieżka_pliku.clone())?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    let mut output_file = File::create(&ścieżka_pliku)?;

    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output_file, *jakość);

    // encoder.encode_image(&final_finalv3_temp_final_ostatecznyv5).map_err(std::io::Error::other)?;
    let width = final_img.width();
    let height = final_img.height();
    let color = final_img.color().into(); // Konwersja na ExtendedColorType

    // 4. Zapisujemy obraz używając write_image (metoda z traitu ImageEncoder)
    encoder
        .write_image(final_img.as_bytes(), width, height, color)
        .map_err(std::io::Error::other)?;

    aktualizuj_postep_dds(obecna_operacja, procent_progress, metryka_operacji, &mut tx).await;

    Ok(())
}
