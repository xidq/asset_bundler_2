use enumy::opcje::OptFormatyKoloruObrazOgólny;
use futures::SinkExt;
use futures::channel::mpsc;
use futures::channel::mpsc::Sender;
use image::{ColorType, DynamicImage, GenericImageView, ImageBuffer, Rgba, imageops::FilterType};
use zbiorowa_konwersja_zdjec::pomocnicze::usun_kanal_alpha;
use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn laczenie_png(
    mut bufor: Vec<DynamicImage>,
    ścieżka_wyjściowa: &Path,
    nazwa_pliku: &str,
    kompresja: &u8,
    alfa_rgb: &(u16, u16, u16),
    bit_depth: &Vec<OptFormatyKoloruObrazOgólny>,
    wymiar: (u32, u32),
) -> Result<(), tokio::io::Error> {
    let depth = bit_depth
        .first()
        .cloned()
        .unwrap_or(OptFormatyKoloruObrazOgólny::B8);

    let (final_img, nazwa_bd) = match depth {
        OptFormatyKoloruObrazOgólny::B8 | OptFormatyKoloruObrazOgólny::L8 => (
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
            "_8b",
        ),
        OptFormatyKoloruObrazOgólny::B8a | OptFormatyKoloruObrazOgólny::L8a => (
            {
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
                DynamicImage::ImageRgba8(nowy_bufor)
            },
            "_8bt",
        ),
        OptFormatyKoloruObrazOgólny::B16
        | OptFormatyKoloruObrazOgólny::L16
        | OptFormatyKoloruObrazOgólny::B32 => (
            {
                let img_r = bufor.remove(0);
                let img_g = bufor.remove(0);
                let img_b = bufor.remove(0);

                let lr = usun_kanal_alpha(img_r, *alfa_rgb).to_luma16();
                let lg = usun_kanal_alpha(img_g, *alfa_rgb).to_luma16();
                let lb = usun_kanal_alpha(img_b, *alfa_rgb).to_luma16();

                let mut nowy_bufor = image::ImageBuffer::new(wymiar.0, wymiar.1);

                for (x, y, pixel) in nowy_bufor.enumerate_pixels_mut() {
                    *pixel = image::Rgb([
                        lr.get_pixel(x, y)[0],
                        lg.get_pixel(x, y)[0],
                        lb.get_pixel(x, y)[0],
                    ]);
                }
                DynamicImage::ImageRgb16(nowy_bufor)
            },
            "_16b",
        ),
        OptFormatyKoloruObrazOgólny::B16a
        | OptFormatyKoloruObrazOgólny::L16a
        | OptFormatyKoloruObrazOgólny::B32a => (
            {
                let img_r = bufor.remove(0);
                let img_g = bufor.remove(0);
                let img_b = bufor.remove(0);
                let img_a = bufor.remove(0);

                let lr = usun_kanal_alpha(img_r, *alfa_rgb).to_luma16();
                let lg = usun_kanal_alpha(img_g, *alfa_rgb).to_luma16();
                let lb = usun_kanal_alpha(img_b, *alfa_rgb).to_luma16();
                let la = usun_kanal_alpha(img_a, *alfa_rgb).to_luma16();

                let mut nowy_bufor = image::ImageBuffer::new(wymiar.0, wymiar.1);

                for (x, y, pixel) in nowy_bufor.enumerate_pixels_mut() {
                    *pixel = image::Rgba([
                        lr.get_pixel(x, y)[0],
                        lg.get_pixel(x, y)[0],
                        lb.get_pixel(x, y)[0],
                        la.get_pixel(x, y)[0],
                    ]);
                }
                DynamicImage::ImageRgba16(nowy_bufor)
            },
            "_16bt",
        ),
    };

    // 4. Budowanie nazwy
    let finalna_nazwa = format!("{}{}.png", nazwa_pliku, nazwa_bd);
    let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
    if !ścieżka_pliku.exists() {
        create_dir_all(ścieżka_pliku.clone())?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    // 5. Zapis z kompresją
    let f = File::create(&ścieżka_pliku)?;

    // Mapowanie u8 kompresji (0-9) na poziomy PNG (Best, Fast, Default)
    let speed = match *kompresja {
        0 => image::codecs::png::CompressionType::Uncompressed,
        _ => image::codecs::png::CompressionType::Level(*kompresja),
    };

    let encoder = image::codecs::png::PngEncoder::new_with_quality(
        f,
        speed,
        image::codecs::png::FilterType::Adaptive,
    );

    final_img
        .write_with_encoder(encoder)
        .map_err(std::io::Error::other)?;

    Ok(())
}
