use bzip2::Compression;
use bzip2::write::BzEncoder;
use enumy::opcje::OptMetodaKompresjiZdjecia;
use futures::SinkExt;
use futures::channel::mpsc;
use futures::channel::mpsc::Sender;
use image::DynamicImage;
use image::imageops::FilterType;
use std::error::Error;
use std::fs::{File, create_dir, create_dir_all};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use xz2::write::XzEncoder;
use encodery::halper::usun_kanal_alpha;

pub async fn laczenie_ff(
    mut bufor: Vec<DynamicImage>,
    ścieżka_wyjściowa: &Path,
    nazwa_pliku: &str,
    alfa_rgb: &(u16, u16, u16),
    wybrana_kompresja: &OptMetodaKompresjiZdjecia,
    wymiar: (u32, u32),
) -> Result<(), tokio::io::Error> {
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

    let lambadziara = wybrana_kompresja;
    let dodatkowa_nazwa = match lambadziara {
        OptMetodaKompresjiZdjecia::Zstd(_) => ".zst",
        OptMetodaKompresjiZdjecia::Bzip2(_) => ".bz2",
        OptMetodaKompresjiZdjecia::Xz(_) => ".xz",
        OptMetodaKompresjiZdjecia::Brak => "",
    };

    // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
    let finalna_nazwa = format!("{}.ff{}", nazwa_pliku, dodatkowa_nazwa);
    let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
    // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
    if !ścieżka_pliku.exists() {
        create_dir_all(ścieżka_pliku.clone())?;
    }
    ścieżka_pliku.push(finalna_nazwa);

    let output_file = File::create(&ścieżka_pliku)?;

    match lambadziara {
        OptMetodaKompresjiZdjecia::Zstd(x) => {
            //kompresja 1-22 || 3def
            let compressor =
                zstd::Encoder::new(output_file, (*x as f32 / 9.).round().clamp(1., 22.) as i32)?
                    .auto_finish();

            // 2. Dodajesz buforowanie dla wydajności
            let buffered_writer = std::io::BufWriter::new(compressor);

            // 3. Reszta bez zmian
            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
            nowy_bufor
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?
        }
        OptMetodaKompresjiZdjecia::Bzip2(x) => {
            //kompresja 1-9
            let bz_encoder = BzEncoder::new(
                output_file,
                Compression::new((*x as f32 / 22.).round().clamp(1., 9.) as u32),
            );
            let buffered_writer = std::io::BufWriter::new(bz_encoder);

            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
            nowy_bufor
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?
        }
        OptMetodaKompresjiZdjecia::Xz(x) => {
            // 1-9 || 6def
            let xz_encoder =
                XzEncoder::new(output_file, (*x as f32 / 22.).round().clamp(1., 9.) as u32);
            let buffered_writer = std::io::BufWriter::new(xz_encoder);

            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
            nowy_bufor
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?
        }
        OptMetodaKompresjiZdjecia::Brak => {
            let buffered_writer = std::io::BufWriter::new(output_file);

            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);

            nowy_bufor
                .write_with_encoder(encoder)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        }
    }

    Ok(())
}
