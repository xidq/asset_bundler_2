use enumy::opcje::OptFormatyKoloruObrazOgólny;
use image::{DynamicImage, ImageEncoder};
use zbiorowa_konwersja_zdjec::pomocnicze::usun_kanal_alpha;
use std::fs::{File, create_dir_all};
use std::path::Path;

pub async fn laczenie_jpg(
    mut bufor: Vec<DynamicImage>,
    ścieżka_wyjściowa: &Path,
    nazwa_pliku: &str,
    jakość: &u8,
    progresywny: &bool,
    bit_depth: &Vec<OptFormatyKoloruObrazOgólny>,
    alfa_rgb: &(u16, u16, u16),
    wymiar: (u32, u32),
) -> Result<(), tokio::io::Error> {
    let (docelowy_wymiar, nazwa_wariantu) = (0, "");
    // *obecna_operacja +=1;

    let (final_img, nazwa_bd) = match bit_depth {
        _ => (
            {
                let img_r = bufor.remove(0);
                let img_g = bufor.remove(0);
                let img_b = bufor.remove(0);

                let lr = usun_kanal_alpha(img_r, *alfa_rgb).to_luma8();
                let lg = usun_kanal_alpha(img_g, *alfa_rgb).to_luma8();
                let lb = usun_kanal_alpha(img_b, *alfa_rgb).to_luma8();
                let mut nowy_bufor = image::ImageBuffer::new(wymiar.0, wymiar.1);

                for (x, y, pixel) in nowy_bufor.enumerate_pixels_mut() {
                    *pixel = image::Rgb([
                        lr.get_pixel(x, y)[0],
                        lg.get_pixel(x, y)[0],
                        lb.get_pixel(x, y)[0],
                    ]);
                }
                DynamicImage::ImageRgb8(nowy_bufor)
            },
            "_l8b",
        ),
    };

    // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
    let finalna_nazwa = format!("{}{}{}.jpg", nazwa_pliku, nazwa_wariantu, nazwa_bd);
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

    Ok(())
}
