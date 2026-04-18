use std::fs;
use std::fs::read;
use std::io::BufReader;
use std::path::PathBuf;
use futures::channel::mpsc;
use futures::SinkExt;
use image::{open, ColorType, DynamicImage, GenericImageView};
use crate::foty::zmiana_fot::{Obraz, PostepMieleniaZdjec, Rozszerzenia};
use crate::foty::wczytanie_zdjec::wczytaj_zdjęcie;

pub struct LaczenieFot{
    sciezka_r:Option<PathBuf>,
    sciezka_g:Option<PathBuf>,
    sciezka_b:Option<PathBuf>,
    sciezka_a:Option<PathBuf>,
    sciezka_out:PathBuf,
    out_format:Rozszerzenia,
    out_bit:Obraz,
    nazwa:String,
}

enum PostepLaczeniaZdjec{
    Start,
    Koniec,
    Błąd(String),
}

pub(crate) async fn fn_do_laczenia_fot(
    dane: LaczenieFot,
    mut tx: mpsc::Sender<PostepLaczeniaZdjec>,
) -> Result<(), tokio::io::Error> {

    let start_czas = std::time::Instant::now(); // Musisz mieć Instant na początku

    let sciezki = [dane.sciezka_r, dane.sciezka_g, dane.sciezka_b, dane.sciezka_a];
    let mut surowe_obrazy = [None, None, None, None];
    let mut max_x = 0u32;
    let mut max_y = 0u32;

    // --- ETAP 1: SZUKAMY MAKSYMALNYCH WYMIARÓW ---
    for (i, opt_p) in sciezki.iter().enumerate() {
        if let Some(p) = opt_p {
            let (img, _) = wczytaj_zdjęcie(p.clone())?;
            if img.width() > max_x { max_x = img.width(); }
            if img.height() > max_y { max_y = img.height(); }
            surowe_obrazy[i] = Some(img);
        }
    }

    if max_x == 0 || max_y == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Brak jakichkolwiek zdjęć"));
    }

    // --- ETAP 2: PRZYGOTOWANIE KANAŁÓW ---
    let mut przygotuj_final = |opt_img: Option<DynamicImage>| -> DynamicImage {
        match opt_img {
            Some(img) => {
                if img.width() < max_x || img.height() < max_y {
                    let mut tlo = DynamicImage::ImageLuma8(image::ImageBuffer::new(max_x, max_y));
                    image::imageops::replace(&mut tlo, &img, 0, 0);
                    tlo
                } else {
                    img
                }
            },
            None => DynamicImage::ImageLuma8(image::ImageBuffer::new(max_x, max_y))
        }
    };

    let img_r = przygotuj_final(surowe_obrazy[0].take());
    let img_g = przygotuj_final(surowe_obrazy[1].take());
    let img_b = przygotuj_final(surowe_obrazy[2].take());
    let img_a = przygotuj_final(surowe_obrazy[3].take());

    let mut ścieżka_wyjściowa = dane.sciezka_out.clone();
    ścieżka_wyjściowa.push(dane.nazwa);


    let wynik: Result<(), tokio::io::Error> = match dane.out_bit {

        Obraz::B8a => {

            let (lr, lg, lb, la) = (img_r.to_luma8(), img_g.to_luma8(), img_b.to_luma8(), img_a.to_luma8());
            let mut nowy_bufor = image::ImageBuffer::new(max_x, max_y);

            for (x, y, pixel) in nowy_bufor.enumerate_pixels_mut() {
                *pixel = image::Rgba([lr.get_pixel(x,y)[0], lg.get_pixel(x,y)[0], lb.get_pixel(x,y)[0], la.get_pixel(x,y)[0]]);
            }

            let out_path = ścieżka_wyjściowa.with_extension("png");
            let mut file = std::fs::File::create(&out_path)?;

            image::DynamicImage::ImageRgba8(nowy_bufor).write_to(&mut file, image::ImageFormat::Png)
                .map_err(std::io::Error::other)
        }
        Obraz::B16 => {
            let (lr, lg, lb) = (img_r.to_luma16(), img_g.to_luma16(), img_b.to_luma16());
            let mut nowy_bufor = image::ImageBuffer::new(max_x, max_y);

            for (x, y, pixel) in nowy_bufor.enumerate_pixels_mut() {
                *pixel = image::Rgb([lr.get_pixel(x,y)[0], lg.get_pixel(x,y)[0], lb.get_pixel(x,y)[0]]);
            }

            let out_path = ścieżka_wyjściowa.with_extension("png");
            let mut file = std::fs::File::create(&out_path)?;

            image::DynamicImage::ImageRgb16(nowy_bufor).write_to(&mut file, image::ImageFormat::Png)
                .map_err(std::io::Error::other)
        }
        Obraz::B16a => {
            let (lr, lg, lb,la) = (img_r.to_luma16(), img_g.to_luma16(), img_b.to_luma16(), img_a.to_luma16());
            let mut nowy_bufor = image::ImageBuffer::new(max_x, max_y);

            for (x, y, pixel) in nowy_bufor.enumerate_pixels_mut() {
                *pixel = image::Rgba([lr.get_pixel(x,y)[0], lg.get_pixel(x,y)[0], lb.get_pixel(x,y)[0], la.get_pixel(x,y)[0]]);
            }

            let out_path = ścieżka_wyjściowa.with_extension("png");
            let mut file = std::fs::File::create(&out_path)?;

            image::DynamicImage::ImageRgba16(nowy_bufor).write_to(&mut file, image::ImageFormat::Png)
                .map_err(std::io::Error::other)
        }
        // Domyślnie RGB 8-bit (JPG)
        _ => {
            let (lr, lg, lb) = (img_r.to_luma8(), img_g.to_luma8(), img_b.to_luma8());
            let mut nowy_bufor = image::ImageBuffer::new(max_x, max_y);

            for (x, y, pixel) in nowy_bufor.enumerate_pixels_mut() {
                *pixel = image::Rgb([lr.get_pixel(x,y)[0], lg.get_pixel(x,y)[0], lb.get_pixel(x,y)[0]]);
            }

            let out_path = ścieżka_wyjściowa.with_extension("jpg");
            let mut file = std::fs::File::create(&out_path)?;

            let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, 90);
            encoder.encode_image(&image::DynamicImage::ImageRgb8(nowy_bufor))
                .map_err(std::io::Error::other)
        }
    };


    match wynik {
        Ok(_) => {
            let czas_napis = format!("{:.2?}", start_czas.elapsed());
            let _ = tx.send(PostepLaczeniaZdjec::Koniec).await;
            Ok(())
        },
        Err(e) => {
            let _ = tx.send(PostepLaczeniaZdjec::Błąd(e.to_string())).await;
            Err(e)
        }
    }
}