use enumy::rozszerzenia::bdepth::BdepthWebp;
use image::DynamicImage;
use std::fs::create_dir_all;
use std::path::Path;
use encodery::halper::usun_kanal_alpha;
#[allow(clippy::too_many_arguments)]
pub async fn laczenie_webp(
    mut bufor: Vec<DynamicImage>,
    ścieżka_wyjściowa: &Path,
    nazwa_pliku: &str,
    jakość: &u8,
    czy_lossless: bool,
    bit_depth: &BdepthWebp,
    alfa_rgb: &(u16, u16, u16),
    wymiar: (u32, u32),
) -> Result<(), tokio::io::Error> {
    let (_docelowy_wymiar, nazwa_wariantu) = (0, "");
    let depth = bit_depth;

        let (final_img, nazwa_bd) = match depth {
            BdepthWebp::Rgb8Alpha => (
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
                "_8b",
            ),
        };

        // println!("{:?}", final_finalv3_temp_final_ostatecznyv5);
        // 5. Budowanie nazwy pliku: nazwa + wariant + kolor + rozszerzenie
        let finalna_nazwa = format!("{}{}{}.webp", nazwa_pliku, nazwa_wariantu, nazwa_bd);
        let mut ścieżka_pliku = ścieżka_wyjściowa.to_path_buf();
        // println!("pokaż co mamy przed samym tworzeniem katalogu:\nścieżka pliku:   {:?}", ścieżka_pliku);
        if !ścieżka_pliku.exists() {
            create_dir_all(ścieżka_pliku.clone())?;
        }
        ścieżka_pliku.push(finalna_nazwa);

        let encoder = webp::Encoder::from_image(&final_img)
            .map_err(std::io::Error::other)?;

        // 2. Kodujesz z wybraną jakością (lossy) -> zwraca WebPMemory
        // *strata to Twoja wartość u8 (0-100)
        let webp_data = if czy_lossless {
            encoder.encode_lossless()
        } else {
            encoder.encode(*jakość as f32)
        };

        // 3. Zapisujesz gotowe bajty do pliku (zastępuje File::create i encode_image)
        std::fs::write(&ścieżka_pliku, &*webp_data)?;


    Ok(())
}
