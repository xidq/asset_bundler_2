use crate::zapisy::edycja_ff::dds_ex_ff;
use crate::zapisy::edycja_jpg::dds_ex_jpg;
use dds::{ColorFormat, DataLayout, Decoder, ImageViewMut};
use enumy::dane_do_przetwarzania::DaneDdsUnpak;
use enumy::rozszerzenia::ext::ImgExt;
use enumy::statusy::LogTxDdsUnpak;
use futures::channel::mpsc::Sender;
use image::DynamicImage;
use std::fs::File;

pub async fn export_dds_array_to_jpg(
    dane: DaneDdsUnpak,
    mut tx: Sender<LogTxDdsUnpak>,
) -> Result<(), std::io::Error> {
    dbg!(dane.ścieżka_wejściowa.display());
    dbg!(dane.ścieżka_wyjściowa.display());
    dbg!(dane.ścieżka_wejściowa.extension());

    let mut obecna_operacja = 0;
    let mut procent_progress = 0;

    let file = File::open(&dane.ścieżka_wejściowa)?;
    let mut decoder =
        Decoder::new(file).map_err(std::io::Error::other)?;

    let layout = decoder.layout();
    let size = layout.main_size(); // Rozmiar pojedynczej tekstury (level 0)

    // Rozpakowujemy informacje o tablicy tekstur z enuma DataLayout
    let (array_len, mip_count) = match layout {
        DataLayout::TextureArray(ref array) => {
            // W Twoim przypadku (TextureArray) używamy .len() dla ilości tekstur
            // i .get(0).iter_mips().count() dla ilości mipmap
            (
                array.len(),
                array.get(0).map(|t| t.iter_mips().count()).unwrap_or(1),
            )
        }
        DataLayout::Texture(ref tex) => (1, tex.iter_mips().count()),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Nieobsługiwany układ danych (Volume nie jest wspierany",
            ));
        }
    };
    let metryka_operacji = array_len * 2; //zebrać

    let _ = tx.try_send(LogTxDdsUnpak::Start);

    for i in 0..array_len {
        // Przygotowujemy bufor na RGBA8 (4 bajty na piksel)
        let mut buffer = vec![0_u8; size.pixels() as usize * 4];

        {
            let view = ImageViewMut::new(
                    &mut buffer,
                    size,
                    ColorFormat::RGBA_U8
                ).ok_or_else(|| {
                    std::io::Error::other(

                        "Błąd ImageViewMut: Bufor ma nieprawidłowy rozmiar dla podanego formatu/wymiarów"
                    )
                })?;

            // Czytamy główną powierzchnię (level 0)
            decoder
                .read_surface(view)
                .map_err(|e| std::io::Error::other( format!("{:?}", e)))?;
        }

        // Jeśli są mipmapy, musimy je pominąć, aby kursor przeszedł do następnej tekstury w tablicy
        if mip_count > 1 {
            decoder
                .skip_mipmaps()
                .map_err(|e| std::io::Error::other( format!("{:?}", e)))?;
        }

        // Zapisujemy do JPG
        if let Some(img_buffer) = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(
            size.width,
            size.height,
            buffer,
        ) {
            let file_stem = dane.ścieżka_wejściowa
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("export");
            let nazawawawa = format!("{}_{}", file_stem, i);

            let dynamic_img = DynamicImage::ImageRgba8(img_buffer);

            let op_ref = &mut obecna_operacja;
            let pr_ref = &mut procent_progress;
            match dane.rozszerzenie {
                ImgExt::Jpg { jakosc, .. } => {
                    dds_ex_jpg(
                        dynamic_img,
                        &dane.ścieżka_wyjściowa,
                        &nazawawawa,
                        &jakosc,
                        &(0_u16, 0_u16, 0_u16),
                        metryka_operacji as u32,
                        op_ref,
                        pr_ref,
                        tx.clone(),
                    )
                    .await?
                }
                ImgExt::Ff { metoda_kompresji } => {
                    dds_ex_ff(
                        dynamic_img,
                        &dane.ścieżka_wyjściowa,
                        &nazawawawa,
                        metryka_operacji as u32,
                        op_ref,
                        pr_ref,
                        &metoda_kompresji,
                        tx.clone(),
                    )
                    .await?
                }
                _ => {}
            }
            // let file_stem = dane.ścieżka_wejściowa.file_stem().and_then(|s| s.to_str()).unwrap_or("export");
            // let out_path = dane.ścieżka_wyjściowa.join(format!("{}_{}.jpg", file_stem, i));

            // dynamic_img.save_with_format(out_path, ImageFormat::Jpeg).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        }

        // Raportowanie postępu
        let percent = (((i + 1) as f32 / array_len as f32) * 100.).round() as u8;
        let _ = tx.try_send(LogTxDdsUnpak::Pending(
            percent,
        ));
    }

    let _ = tx.try_send(LogTxDdsUnpak::Finito(
        format!("Wyeksportowano {} plików", array_len),
    ));
    Ok(())
}
