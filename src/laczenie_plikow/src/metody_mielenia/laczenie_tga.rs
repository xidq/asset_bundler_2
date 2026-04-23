use enumy::opcje::OptFormatyKoloruObrazuTga;
use futures::SinkExt;
use futures::channel::mpsc;
use futures::channel::mpsc::Sender;
use image::{ColorType, DynamicImage, GenericImageView, ImageBuffer, Rgba, imageops::FilterType};
use zbiorowa_konwersja_zdjec::pomocnicze::usun_kanal_alpha;
use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn laczenie_tga(
    mut bufor: Vec<DynamicImage>,
    ścieżka_wyjściowa: &Path,
    nazwa_pliku: &str,
    alfa_rgb: &(u16, u16, u16),
    bit_depth: &Vec<OptFormatyKoloruObrazuTga>,
    wymiar: (u32, u32),
) -> Result<(), tokio::io::Error> {
    let depth = bit_depth
        .first()
        .cloned()
        .unwrap_or(OptFormatyKoloruObrazuTga::TrueColor24);

    // --- OBSŁUGA BIT DEPTH I FORMATU ---
    let (final_img, nazwa_bd, color_type, szer, wys) = match depth {
        OptFormatyKoloruObrazuTga::HighColor16 => {
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
                let alpha_value = la.get_pixel(x, y)[0];
                let binary_alpha = if alpha_value > 128 { 255 } else { 0 };
                *pixel = image::Rgba([
                    lr.get_pixel(x, y)[0],
                    lg.get_pixel(x, y)[0],
                    lb.get_pixel(x, y)[0],
                    binary_alpha,
                ]);
            }

            (
                nowy_bufor.into_raw(),
                "_hc16",
                image::ExtendedColorType::Rgba8,
                wymiar.0,
                wymiar.1,
            )
        }

        OptFormatyKoloruObrazuTga::TrueColorA32 => {
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
                image::ExtendedColorType::Rgb8,
                wymiar.0,
                wymiar.1,
            )
        }
        _ => {
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
    };

    let finalna_nazwa = format!("{}{}.tga", nazwa_pliku, nazwa_bd);
    let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();

    if !ścieżka_pliku.exists() {
        create_dir_all(&ścieżka_pliku)?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    let f = File::create(&ścieżka_pliku)?;

    let encoder = image::codecs::tga::TgaEncoder::new(f);

    encoder
        .encode(&final_img, szer, wys, color_type)
        .map_err(std::io::Error::other)?;

    Ok(())
}
