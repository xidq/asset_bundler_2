use enumy::opcje::OptFormatyKoloruObrazuQoi;
use image::ImageEncoder;
use image::{DynamicImage, GenericImageView};
use std::fs::{File, create_dir_all};
use std::path::Path;
use encodery::halper::usun_kanal_alpha;

pub async fn laczenie_qoi(
    mut bufor: Vec<DynamicImage>,
    ścieżka_wyjściowa: &Path,
    nazwa_pliku: &str,
    alfa_rgb: &(u16, u16, u16),
    bit_depth: &OptFormatyKoloruObrazuQoi,
    wymiar: (u32, u32),
) -> Result<(), tokio::io::Error> {
    let depth = bit_depth;

    // --- OBSŁUGA BIT DEPTH I FORMATU ---
    let (final_img, nazwa_bd, color_type, szer, wys) = match depth {
        OptFormatyKoloruObrazuQoi::Color24 => {
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
            (
                nowy_bufor.into_raw(),
                "_tc24",
                image::ExtendedColorType::Rgb8,
                wymiar.0,
                wymiar.1,
            )
        }

        OptFormatyKoloruObrazuQoi::ColorA32 => {
            let img_r = bufor.remove(0);
            let img_g = bufor.remove(0);
            let img_b = bufor.remove(0);
            let img_a = bufor.remove(0);

            let lr = usun_kanal_alpha(img_r, *alfa_rgb).to_luma8();
            let lg = usun_kanal_alpha(img_g, *alfa_rgb).to_luma8();
            let lb = usun_kanal_alpha(img_b, *alfa_rgb).to_luma8();
            let la = usun_kanal_alpha(img_a, *alfa_rgb).to_luma8();

            let mut nowy_bufor = image::ImageBuffer::new(wymiar.0, wymiar.1);

            for (x, y, pixel) in nowy_bufor.enumerate_pixels_mut() {
                *pixel = image::Rgba([
                    lr.get_pixel(x, y)[0],
                    lg.get_pixel(x, y)[0],
                    lb.get_pixel(x, y)[0],
                    la.get_pixel(x, y)[0],
                ]);
            }
            (
                nowy_bufor.into_raw(),
                "_tc32",
                image::ExtendedColorType::Rgba8,
                wymiar.0,
                wymiar.1,
            )
        }
    };

    // 4. Budowanie nazwy
    let finalna_nazwa = format!("{}{}.qoi", nazwa_pliku, nazwa_bd);
    let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();

    if !ścieżka_pliku.exists() {
        create_dir_all(&ścieżka_pliku)?;
    }
    ścieżka_pliku.push(finalna_nazwa);
    dbg!("ścieżka pliku: {}", &ścieżka_pliku);

    // 5. Zapis z wykorzystaniem enkodera i naszych surowych danych
    let f = File::create(&ścieżka_pliku)?;

    let encoder = image::codecs::qoi::QoiEncoder::new(f);

    encoder
        .write_image(&final_img, szer, wys, color_type)
        .map_err(std::io::Error::other)?;

    Ok(())
}
