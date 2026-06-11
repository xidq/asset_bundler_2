use crate::halper::{usun_kanal_alpha, zaszumianie};
use crate::send::wyslij_status;
use crate::zapisywanie::generic::{get_higher_tier_copy, InneDane};
use bzip2::write::BzEncoder;
use bzip2::Compression;
use enumy::opcje::OptIstniejePlik;
use enumy::przetwarzanie::PrzetwarzanieFf;
use enumy::rozszerzenia::kompresje::ForFfKompresja;
use enumy::statusy::Logi;
use futures::channel::mpsc::Sender;
use image::DynamicImage;
use std::fs::{create_dir_all, File};
use std::sync::Arc;
use tokio::sync::Mutex;
use xz2::write::XzEncoder;

pub async fn ff_match<T>(
    dane: PrzetwarzanieFf,
    dane2: InneDane<ForFfKompresja>,
    // wymiar: u32,
    // kompresja: ForFfKompresja,
    // nazwa_wariantu: String,
    // filtr: FilterType,
    metryka_operacji: Option<u32>,
    obecna_operacja: Arc<Mutex<u32>>,
    mut tx: Sender<T>,
) -> Result<(), tokio::io::Error>
where T: Logi,
{
    let bombozooo = if dane2.wymiar == 0 {
        DynamicImage::ImageRgba16(usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_rgba16())
    } else {
        DynamicImage::ImageRgba16(usun_kanal_alpha(dane.bufor.clone(), dane.alpha).to_rgba16())
            .resize(dane2.wymiar, dane2.wymiar, dane2.filtr)
    };

    let final_finalv3_temp_final_ostatecznyv5 = match dane.zaszumienie {
        Some(xoxo) => zaszumianie(xoxo, bombozooo),
        None => bombozooo,
    };

    let lambadziara = dane2.bdepth;
    // for lambadziara in wybrana_kompresja {
    let dodatkowa_nazwa = match lambadziara {
        ForFfKompresja::Zstd(_) => ".zst",
        ForFfKompresja::Bzip2(_) => ".bz2",
        ForFfKompresja::Xz(_) => ".xz",
        ForFfKompresja::Brak => "",
    };

    // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
    let finalna_nazwa = format!("{}{}.ff{}", dane.nazwa, dane2.nazwa_wariantu, dodatkowa_nazwa);
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

    let output_file = File::create(&ścieżka_pliku)?;

    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;

    match lambadziara {
        ForFfKompresja::Zstd(x) => {
            //kompresja 1-22 || 3def
            let compressor = zstd::Encoder::new(
                output_file,
                (x as f32 / 9.).round().clamp(1., 22.) as i32,
            )?
                .auto_finish();

            // 2. Dodajesz buforowanie dla wydajności
            let buffered_writer = std::io::BufWriter::new(compressor);

            // 3. Reszta bez zmian
            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
            final_finalv3_temp_final_ostatecznyv5
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?;
        }
        ForFfKompresja::Bzip2(x) => {
            //kompresja 1-9
            let bz_encoder = BzEncoder::new(
                output_file,
                Compression::new((x as f32 / 22.).round().clamp(1., 9.) as u32),
            );
            let buffered_writer = std::io::BufWriter::new(bz_encoder);

            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
            final_finalv3_temp_final_ostatecznyv5
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?
        }
        ForFfKompresja::Xz(x) => {
            // 1-9 || 6def
            let xz_encoder =
                XzEncoder::new(output_file, (x as f32 / 22.).round().clamp(1., 9.) as u32);
            let buffered_writer = std::io::BufWriter::new(xz_encoder);

            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);
            final_finalv3_temp_final_ostatecznyv5
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?
        }
        ForFfKompresja::Brak => {
            let buffered_writer = std::io::BufWriter::new(output_file);

            let encoder = image::codecs::farbfeld::FarbfeldEncoder::new(buffered_writer);

            final_finalv3_temp_final_ostatecznyv5
                .write_with_encoder(encoder)
                .map_err(std::io::Error::other)?
        }
    }

    let mut oopr = obecna_operacja.lock().await;
    *oopr += 1;
    let obecnie = *oopr;
    drop(oopr);

    wyslij_status(&mut tx, T::postep_liczbowy(obecnie, metryka_operacji)).await;



Ok(())
}