use crate::zapisy::inne_dds::aktualizuj_postep_dds;
use bzip2::Compression;
use bzip2::write::BzEncoder;
use enumy::opcje::OptMetodaKompresjiZdjecia;
use enumy::statusy::LogTxDoRozpakowanieDds;
use futures::channel::mpsc::Sender;
use image::DynamicImage;
use std::fs::{File, create_dir_all};
use std::path::Path;
use xz2::write::XzEncoder;

pub async fn dds_ex_ff(
    mut bufor: DynamicImage,
    ścieżka_wyjściowa: &Path,
    nazwa_pliku: &str,
    metryka_operacji: u32,
    obecna_operacja: &mut u32, // Zmień na &mut u32
    procent_progress: &mut u8,
    wybrana_kompresja: &OptMetodaKompresjiZdjecia,
    mut tx: Sender<LogTxDoRozpakowanieDds>,
) -> Result<(), tokio::io::Error> {
    let (docelowy_wymiar, nazwa_wariantu) = (0, "");
    // *obecna_operacja +=1;
    aktualizuj_postep_dds(obecna_operacja, procent_progress, metryka_operacji, &mut tx).await;

    let bombozooo = DynamicImage::ImageRgba16(bufor.clone().to_rgba16());

    let lambadziara = wybrana_kompresja;
    let dodatkowa_nazwa = match lambadziara {
        OptMetodaKompresjiZdjecia::Zstd(_) => ".zst",
        OptMetodaKompresjiZdjecia::Bzip2(_) => ".bz2",
        OptMetodaKompresjiZdjecia::Xz(_) => ".xz",
        OptMetodaKompresjiZdjecia::Brak => "",
    };

    let finalna_nazwa = format!("{}{}.ff{}", nazwa_pliku, nazwa_wariantu, dodatkowa_nazwa);
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
            bombozooo
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?;
        }
        OptMetodaKompresjiZdjecia::Bzip2(x) => {
            //kompresja 1-9
            let bz_encoder = BzEncoder::new(
                output_file,
                Compression::new((*x as f32 / 22.).round().clamp(1., 9.) as u32),
            );
            let buffered_writer = std::io::BufWriter::new(bz_encoder);

            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
            bombozooo
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?
        }
        OptMetodaKompresjiZdjecia::Xz(x) => {
            // 1-9 || 6def
            let xz_encoder =
                XzEncoder::new(output_file, (*x as f32 / 22.).round().clamp(1., 9.) as u32);
            let buffered_writer = std::io::BufWriter::new(xz_encoder);

            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
            bombozooo
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?
        }
        OptMetodaKompresjiZdjecia::Brak => {
            let buffered_writer = std::io::BufWriter::new(output_file);

            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);

            bombozooo
                .write_with_encoder(encoder)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        }
    }

    aktualizuj_postep_dds(obecna_operacja, procent_progress, metryka_operacji, &mut tx).await;

    Ok(())
}
