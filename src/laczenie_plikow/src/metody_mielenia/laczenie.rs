use std::path::PathBuf;
use encodery::halper::usun_kanal_alpha;
use enumy::opcje::OptFormatyKoloruObrazOgólny;
use image::DynamicImage;

pub async fn laczenie_vac_to_dyn(
    mut bufor: Vec<DynamicImage>,
    bit_depth: &OptFormatyKoloruObrazOgólny,
    wymiar: (u32, u32),
) -> Result<(DynamicImage), tokio::io::Error> {
    let depth = bit_depth;
    let alfa_rgb:(u16, u16, u16) = (0, 0, 0);

    let (final_img) = match depth {
        OptFormatyKoloruObrazOgólny::B8 | OptFormatyKoloruObrazOgólny::L8 => 
            {
                let img_r = bufor.remove(0);
                let img_g = bufor.remove(0);
                let img_b = bufor.remove(0);

                let lr = usun_kanal_alpha(img_r, alfa_rgb).to_luma8();
                let lg = usun_kanal_alpha(img_g, alfa_rgb).to_luma8();
                let lb = usun_kanal_alpha(img_b, alfa_rgb).to_luma8();

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

        OptFormatyKoloruObrazOgólny::B8a | OptFormatyKoloruObrazOgólny::L8a => 
            {
                let img_r = bufor.remove(0);
                let img_g = bufor.remove(0);
                let img_b = bufor.remove(0);
                let img_a = bufor.remove(0);

                let lr = usun_kanal_alpha(img_r, alfa_rgb).to_luma8();
                let lg = usun_kanal_alpha(img_g, alfa_rgb).to_luma8();
                let lb = usun_kanal_alpha(img_b, alfa_rgb).to_luma8();
                let la = usun_kanal_alpha(img_a, alfa_rgb).to_luma8();

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
        OptFormatyKoloruObrazOgólny::B16
        | OptFormatyKoloruObrazOgólny::L16
        | OptFormatyKoloruObrazOgólny::B32 => 
            {
                let img_r = bufor.remove(0);
                let img_g = bufor.remove(0);
                let img_b = bufor.remove(0);

                let lr = usun_kanal_alpha(img_r, alfa_rgb).to_luma16();
                let lg = usun_kanal_alpha(img_g, alfa_rgb).to_luma16();
                let lb = usun_kanal_alpha(img_b, alfa_rgb).to_luma16();

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
        OptFormatyKoloruObrazOgólny::B16a
        | OptFormatyKoloruObrazOgólny::L16a
        | OptFormatyKoloruObrazOgólny::B32a => 
            {
                let img_r = bufor.remove(0);
                let img_g = bufor.remove(0);
                let img_b = bufor.remove(0);
                let img_a = bufor.remove(0);

                let lr = usun_kanal_alpha(img_r, alfa_rgb).to_luma16();
                let lg = usun_kanal_alpha(img_g, alfa_rgb).to_luma16();
                let lb = usun_kanal_alpha(img_b, alfa_rgb).to_luma16();
                let la = usun_kanal_alpha(img_a, alfa_rgb).to_luma16();

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
    };
    Ok((final_img))
}

pub async fn ogarnij_sciezki_w_koncu(sciezka_out:PathBuf, nazwa:String)-> Result<(PathBuf), tokio::io::Error> {

    let mut gfdsdf = sciezka_out;
    gfdsdf.push(nazwa);
    Ok(gfdsdf)
}